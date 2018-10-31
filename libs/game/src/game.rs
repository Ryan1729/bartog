use choices::{
    choose_can_play_graph, choose_in_game_changes, choose_play_again, choose_rule, choose_suit,
    choose_wild_flags, do_choices,
};
use common::*;
use game_state::{
    in_game::{self, player_name, ApplyToState},
    EventLog, GameState, LogHeading, Rules, Status,
};
use platform_types::{Button, Input, Speaker, State, StateParams, SFX};
use rand::Rng;
use rule_changes::{
    apply_can_play_graph_changes, apply_when_played_changes, apply_wild_change, reset,
};

pub struct BartogState {
    pub game_state: GameState,
    pub framebuffer: Framebuffer,
    pub input: Input,
    pub speaker: Speaker,
}

impl BartogState {
    pub fn new((seed, logger): StateParams) -> Self {
        let framebuffer = Framebuffer::new();

        BartogState {
            game_state: GameState::new(seed, logger),
            framebuffer,
            input: Input::new(),
            speaker: Speaker::new(),
        }
    }
}

impl State for BartogState {
    fn frame(&mut self, handle_sound: fn(SFX)) {
        update_and_render(
            &mut self.framebuffer,
            &mut self.game_state,
            self.input,
            &mut self.speaker,
        );

        self.input.previous_gamepad = self.input.gamepad;

        for request in self.speaker.drain() {
            handle_sound(request);
        }
    }

    fn press(&mut self, button: Button::Ty) {
        if self.input.previous_gamepad.contains(button) {
            //This is meant to pass along the key repeat, if any.
            //Not sure if rewriting history is the best way to do this.
            self.input.previous_gamepad.remove(button);
        }

        self.input.gamepad.insert(button);
    }

    fn release(&mut self, button: Button::Ty) {
        self.input.gamepad.remove(button);
    }

    fn get_frame_buffer(&self) -> &[u32] {
        &self.framebuffer.buffer
    }
}

#[allow(dead_code)]
enum Face {
    Up,
    Down,
}

fn draw_hand_ltr(
    framebuffer: &mut Framebuffer,
    hand: &Hand,
    offset: u8,
    (mut x, y): (u8, u8),
    face: Face,
) {
    match face {
        Face::Up => {
            for &card in hand.iter() {
                framebuffer.draw_card(card, x, y);

                x += offset;
            }
        }
        Face::Down => {
            for &_card in hand.iter() {
                framebuffer.draw_card_back(x, y);

                x += offset;
            }
        }
    }
}

fn draw_hand_ttb(
    framebuffer: &mut Framebuffer,
    hand: &Hand,
    offset: u8,
    (x, mut y): (u8, u8),
    face: Face,
) {
    match face {
        Face::Up => {
            for &card in hand.iter() {
                framebuffer.draw_card(card, x, y);

                y += offset;
            }
        }
        Face::Down => {
            for &_card in hand.iter() {
                framebuffer.draw_card_back(x, y);

                y += offset;
            }
        }
    }
}

fn draw_hand(framebuffer: &mut Framebuffer, hand: &Hand, face: Face) {
    let offset = get_card_offset(hand.spread, hand.len());

    match hand.spread {
        Spread::LTR((x, _), y) => {
            draw_hand_ltr(framebuffer, hand, offset, (x, y), face);
        }
        Spread::TTB((y, _), x) => {
            draw_hand_ttb(framebuffer, hand, offset, (x, y), face);
        }
    }
}

fn draw_hand_with_cursor_ltr(
    framebuffer: &mut Framebuffer,
    hand: &Hand,
    offset: u8,
    (mut x, y): (u8, u8),
    index: usize,
) {
    let mut selected_card_and_offset = None;
    for (i, &card) in hand.iter().enumerate() {
        if i == index {
            selected_card_and_offset = Some((card, x));
            x += offset;

            continue;
        }
        framebuffer.draw_card(card, x, y);

        x += offset;
    }

    if let Some((card, cursor_offset)) = selected_card_and_offset {
        framebuffer.draw_highlighted_card(card, cursor_offset, y);
    }
}

fn draw_hand_with_cursor_ttb(
    framebuffer: &mut Framebuffer,
    hand: &Hand,
    offset: u8,
    (x, mut y): (u8, u8),
    index: usize,
) {
    let mut selected_card_and_offset = None;
    for (i, &card) in hand.iter().enumerate() {
        if i == index {
            selected_card_and_offset = Some((card, y));
            y += offset;

            continue;
        }
        framebuffer.draw_card(card, x, y);

        y += offset;
    }

    if let Some((card, cursor_offset)) = selected_card_and_offset {
        framebuffer.draw_highlighted_card(card, x, cursor_offset);
    }
}

fn draw_hand_with_cursor(framebuffer: &mut Framebuffer, hand: &Hand, index: usize) {
    let offset = get_card_offset(hand.spread, hand.len());

    match hand.spread {
        Spread::LTR((x, _), y) => {
            draw_hand_with_cursor_ltr(framebuffer, hand, offset, (x, y), index);
        }
        Spread::TTB((y, _), x) => {
            draw_hand_with_cursor_ttb(framebuffer, hand, offset, (x, y), index);
        }
    }
}

fn draw_event_log(framebuffer: &mut Framebuffer, state: &GameState) {
    framebuffer.bottom_six_slice(WINDOW_TOP_LEFT, 0, 0, SCREEN_WIDTH as u8, state.log_height);

    let mut y = SPRITE_SIZE;
    for line in state.event_log.get_window_slice() {
        framebuffer.print_line(line, SPRITE_SIZE, y, WHITE_INDEX);

        y += FONT_SIZE;
        if y >= state.log_height {
            break;
        }
    }
}

fn move_cursor(state: &mut in_game::State, input: Input, speaker: &mut Speaker) -> bool {
    if input.pressed_this_frame(Button::Right) {
        if state.hand_index < state.hand.len().saturating_sub(1) {
            state.hand_index = state.hand_index.saturating_add(1);
        }
        speaker.request_sfx(SFX::CardSlide);
        true
    } else if input.pressed_this_frame(Button::Left) {
        state.hand_index = state.hand_index.saturating_sub(1);
        speaker.request_sfx(SFX::CardSlide);
        true
    } else {
        false
    }
}

fn can_play(state: &in_game::State, rules: &Rules, &card: &Card) -> bool {
    if let Some(&top_of_discard) = state.discard.last() {
        // TODO should a card that is wild allow a non-wild card of the same rank
        // to be played on it?
        rules.is_wild(card) || if rules.is_wild(top_of_discard) {
            state.top_wild_declared_as == Some(get_suit(card))
        } else {
            rules.can_play_graph.is_playable_on(card, top_of_discard)
        }
    } else {
        true
    }
}

//Since this uses rng, calling this in response to repeatable user input allows rng manipulation.
fn cpu_would_play<R: Rng>(
    state: &mut in_game::State,
    rng: &mut R,
    rules: &Rules,
    playerId: PlayerID,
) -> Option<u8> {
    let playable: Vec<(usize, Card)> = {
        let hand = state.get_hand(playerId);
        hand.iter()
            .cloned()
            .enumerate()
            .filter(|(_, card)| can_play(state, rules, card))
            .collect()
    };

    rng.choose(&playable).map(|&(i, _)| i as u8)
}

fn move_to_discard(state: &mut GameState, card: Card) {
    if !state.rules.is_wild(card) {
        state.in_game.top_wild_declared_as = None;
    }

    state.in_game.discard.push(card);

    for change in state.rules.when_played.0[card as usize].iter() {
        change.apply_to_state(&mut state.in_game)
    }
}

fn log_wild_selection(state: &mut GameState, player: PlayerID) {
    if let Some(suit) = state.in_game.top_wild_declared_as {
        let player_name = player_name(player);
        let suit_str = get_suit_str(suit);
        let event_str = &[
            player_name.as_bytes(),
            b" selected ",
            suit_str.as_bytes(),
            b".",
        ]
            .concat();
        state.event_log.push(event_str)
    }
}

fn advance_card_animations(state: &mut GameState) {
    // I should really be able to use `Vec::retain` here,
    // but that passes a `&T` insteead of a `&mut T`.

    let mut i = state.in_game.card_animations.len() - 1;
    loop {
        let (is_complete, last_pos) = {
            let animation = &mut state.in_game.card_animations[i];

            let last_pos = (animation.card.x, animation.card.y);

            animation.approach_target();

            (animation.is_complete(), last_pos)
        };

        if is_complete {
            let mut animation = state.in_game.card_animations.remove(i);

            let card = animation.card.card;

            match animation.completion_action {
                Action::MoveToDiscard => {
                    move_to_discard(state, card);
                }
                Action::SelectWild(playerId) => {
                    if is_cpu_player(&state.in_game, playerId) {
                        state.in_game.top_wild_declared_as = {
                            let hand = state.in_game.get_hand(playerId);
                            hand.most_common_suit()
                        };
                        log_wild_selection(state, playerId);
                        move_to_discard(state, card);
                    } else {
                        if let Some(suit) = choose_suit(state) {
                            state.in_game.top_wild_declared_as = Some(suit);
                            log_wild_selection(state, playerId);
                            move_to_discard(state, card);
                        } else {
                            //wait until they choose
                            animation.card.x = last_pos.0;
                            animation.card.y = last_pos.1;
                            state.in_game.card_animations.push(animation);
                        }
                    }
                }
                Action::MoveToHand(playerId) => {
                    state.in_game.get_hand_mut(playerId).push(card);
                }
            }
        }

        if i == 0 {
            break;
        }
        i -= 1;
    }
}

fn get_discard_animation(
    state: &mut in_game::State,
    player: PlayerID,
    card_index: u8,
    event_log: &mut EventLog,
    rules: &Rules,
) -> Option<CardAnimation> {
    state
        .remove_positioned_card(player, card_index)
        .map(|card| {
            let player_name = player_name(player);

            let card_string = get_card_string(card.card);

            let rank = get_rank(card.card);

            let event_str = &[
                player_name.as_bytes(),
                if rank == Ranks::ACE || rank == Ranks::EIGHT {
                    b" played an "
                } else {
                    b" played a "
                },
                card_string.as_bytes(),
                b".",
            ]
                .concat();

            event_log.push(event_str);

            if rules.is_wild(card.card) {
                CardAnimation::new(card, DISCARD_X, DISCARD_Y, Action::SelectWild(player))
            } else {
                CardAnimation::new(card, DISCARD_X, DISCARD_Y, Action::MoveToDiscard)
            }
        })
}

fn get_draw_animation<R: Rng>(
    state: &mut in_game::State,
    player: PlayerID,
    event_log: &mut EventLog,
    rng: &mut R,
) -> Option<CardAnimation> {
    let (spread, len) = {
        let hand = state.get_hand(player);

        (hand.spread, hand.len())
    };
    let card = {
        if let Some(c) = state.deck.draw() {
            Some(c)
        } else {
            let top_card = state.discard.draw()?;

            state.deck.fill(state.discard.drain());
            state.deck.shuffle(rng);

            state.discard.push(top_card);

            state.deck.draw()
        }
    }?;

    let (x, y) = get_card_position(spread, len + 1, len);

    let player_name = player_name(player);

    let event_str = &[player_name.as_bytes(), b" drew a card."].concat();

    event_log.push(event_str);

    Some(CardAnimation::new(
        PositionedCard {
            card,
            x: DECK_X,
            y: DECK_Y,
        },
        x,
        y,
        Action::MoveToHand(player),
    ))
}

#[inline]
fn push_if<T>(vec: &mut Vec<T>, op: Option<T>) {
    if let Some(t) = op {
        vec.push(t);
    }
}

fn is_cpu_player(state: &in_game::State, playerId: PlayerID) -> bool {
    (playerId as usize) < state.cpu_hands.len()
}

fn take_turn(game_state: &mut GameState, input: Input, speaker: &mut Speaker) {
    let state = &mut game_state.in_game;
    let rules = &game_state.rules;
    let event_log = &mut game_state.event_log;
    let rng = &mut game_state.rng;

    let player = state.current_player;
    match player {
        p if is_cpu_player(&state, p) => {
            if let Some(index) = cpu_would_play(state, rng, rules, p) {
                let animation = get_discard_animation(state, player, index, event_log, rules);
                push_if(&mut state.card_animations, animation);
            } else {
                let animation = get_draw_animation(state, player, event_log, rng);
                push_if(&mut state.card_animations, animation);
            }

            state.current_player += 1;
        }
        _ => {
            if move_cursor(state, input, speaker) {
                //Already handled.
            } else if input.pressed_this_frame(Button::A) {
                let index = state.hand_index;

                let can_play_it = {
                    state
                        .hand
                        .get(index)
                        .map(|card| can_play(&state, rules, card))
                        .unwrap_or(false)
                };

                if can_play_it {
                    let animation = get_discard_animation(state, player, index, event_log, rules);

                    push_if(&mut state.card_animations, animation);

                    state.current_player = 0;
                } else {
                    //TODO good feedback. Tint the card red or shake it or something?
                }
            } else if input.pressed_this_frame(Button::B) {
                let animation = get_draw_animation(state, player, event_log, rng);
                push_if(&mut state.card_animations, animation);

                state.current_player = 0;
            }
        }
    }

    let winners: Vec<PlayerID> = in_game::all_player_ids()
        .iter()
        .filter(|&&player| state.get_hand(player).len() == 0)
        .cloned()
        .collect();

    if state.no_winners_yet() {
        state.winners = winners;
    }
}

fn update(state: &mut GameState, input: Input, speaker: &mut Speaker) {
    match state.status {
        Status::InGame => update_in_game(state, input, speaker),
        Status::RuleSelection => update_rule_selection(state),
        Status::RuleSelectionCanPlay => update_can_play_graph(state),
        Status::RuleSelectionWild => update_wild(state),
        Status::RuleSelectionWhenPlayed => update_when_played(state),
    }
}

fn update_when_played(state: &mut GameState) {
    match choose_in_game_changes(state) {
        in_game::ChoiceState {
            card, ref changes, ..
        }
            if changes.len() > 0 =>
        {
            apply_when_played_changes(state, card, changes.clone(), in_game::PLAYER_ID);

            state.status = Status::InGame;
        }
        _ => {
            //wait until they choose
        }
    }
}

fn update_wild(state: &mut GameState) {
    match choose_wild_flags(state) {
        None => {
            //wait until they choose
        }
        Some(wild) => {
            apply_wild_change(state, wild, in_game::PLAYER_ID);

            state.status = Status::InGame;
        }
    }
}

fn update_can_play_graph(state: &mut GameState) {
    match choose_can_play_graph(state) {
        ref x if x.len() == 0 => {
            //wait until they choose
        }
        changes => {
            apply_can_play_graph_changes(state, changes, in_game::PLAYER_ID);

            state.status = Status::InGame;
        }
    }
}

fn update_rule_selection(state: &mut GameState) {
    match choose_rule(state) {
        None => {
            //wait until they choose
        }
        Some(status) => {
            state.status = status;
        }
    }
}

fn update_in_game(state: &mut GameState, input: Input, speaker: &mut Speaker) {
    match state.log_heading {
        LogHeading::Up => {
            state.log_height = state.log_height.saturating_sub(SPRITE_SIZE);
        }
        LogHeading::Down => {
            if state.log_height <= SCREEN_HEIGHT as u8 - SPRITE_SIZE {
                state.log_height += SPRITE_SIZE;
            }
        }
    }

    if input.pressed_this_frame(Button::Start) {
        state.log_heading = match state.log_heading {
            LogHeading::Up => LogHeading::Down,
            LogHeading::Down => LogHeading::Up,
        };
    }

    if state.log_height > 0 {
        if input.pressed_this_frame(Button::Up) {
            state.event_log.top_index = state.event_log.top_index.saturating_sub(1);
        //TODO feedback when you hit the top edge
        } else if input.pressed_this_frame(Button::Down) {
            if state.event_log.top_index < state.event_log.len() {
                state.event_log.top_index += 1;
            } else {
                //TODO feedback when you hit the bottom edge
            }
        }
    } else if state.choice.is_idle() {
        if state.animations_settled() {
            if state.in_game.no_winners_yet() {
                take_turn(state, input, speaker);
            }
        } else {
            advance_card_animations(state);

            move_cursor(&mut state.in_game, input, speaker);
        }
    }
}

#[inline]
pub fn render_in_game(framebuffer: &mut Framebuffer, state: &in_game::State) {
    for hand in state.cpu_hands.iter() {
        draw_hand(framebuffer, hand, Face::Down);
    }

    framebuffer.draw_card_back(DECK_X, DECK_Y);

    match state.top_wild_declared_as {
        Some(suit) => {
            let (colour, suit_char) = get_suit_colour_and_char(suit);

            framebuffer.print_char(
                suit_char,
                DECK_X + card::WIDTH + 2,
                DECK_Y + (card::HEIGHT - FONT_SIZE) / 2,
                colour,
            );
        }
        None => {}
    }

    state
        .discard
        .iter()
        .last()
        .map(|&c| framebuffer.draw_card(c, DISCARD_X, DISCARD_Y));

    draw_hand_with_cursor(framebuffer, &state.hand, state.hand_index as usize);

    for &CardAnimation {
        card,
        completion_action,
        ..
    } in state.card_animations.iter()
    {
        match completion_action {
            Action::MoveToHand(_) => framebuffer.draw_card_back(card.x, card.y),
            Action::MoveToDiscard | Action::SelectWild(_) => {
                framebuffer.draw_card(card.card, card.x, card.y)
            }
        }
    }
}

#[inline]
pub fn update_and_render(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
    speaker: &mut Speaker,
) {
    state.context.frame_init();

    update(state, input, speaker);

    invariant_assert_eq!(state.in_game.missing_cards(), vec![0; 0]);

    framebuffer.clearTo(GREEN);

    render_in_game(framebuffer, &state.in_game);

    if state.round_is_over() {
        if let Some(()) = choose_play_again(state) {
            reset(state);
        }
    }

    if state.log_height > 0 {
        draw_event_log(framebuffer, &state);
    } else {
        do_choices(framebuffer, state, input, speaker);
    }
}
