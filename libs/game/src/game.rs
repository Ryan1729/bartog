use choices::{
    choose_can_play_graph, choose_in_game_changes, choose_play_again, choose_rule,
    choose_wild_flags, do_choices, show_rules_screen,
};
use common::{GLOBAL_ERROR_LOGGER, GLOBAL_LOGGER, *};
use game_state::{in_game, GameState, LogHeading, Rules, Status};
use platform_types::{Button, Input, Speaker, State, SFX};
pub use platform_types::StateParams;
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
    pub fn new((seed, logger, error_logger): StateParams) -> Self {
        let framebuffer = Framebuffer::new();

        unsafe {
            GLOBAL_LOGGER = logger;
            GLOBAL_ERROR_LOGGER = error_logger;
        }

        features::log!(seed);

        BartogState {
            game_state: GameState::new(seed),
            framebuffer,
            input: Input::new(),
            speaker: Speaker::new(),
        }
    }
}

impl State for BartogState {
    fn frame(&mut self) -> (&[u32], &[SFX]) {
        self.speaker.clear();
        update_and_render(
            &mut self.framebuffer,
            &mut self.game_state,
            self.input,
            &mut self.speaker,
        );

        self.input.previous_gamepad = self.input.gamepad;

        (&self.framebuffer.buffer, self.speaker.slice())
    }

    fn press(&mut self, button: Button) {
        if self.input.previous_gamepad.contains(button) {
            //This is meant to pass along the key repeat, if any.
            //Not sure if rewriting history is the best way to do this.
            self.input.previous_gamepad.remove(button);
        }

        self.input.gamepad.insert(button);
    }

    fn release(&mut self, button: Button) {
        self.input.gamepad.remove(button);
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
    if input.pressed_this_frame(Button::RIGHT) {
        if state.hand_index < state.hand.len().saturating_sub(1) {
            state.hand_index = state.hand_index.saturating_add(1);
        }
        speaker.request_sfx(SFX::CardSlide);
        true
    } else if input.pressed_this_frame(Button::LEFT) {
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
        rules.is_wild(card)
            || if rules.is_wild(top_of_discard) {
                state.top_wild_declared_as == Some(get_suit(card))
                    //this can happen depending on the card movement rules
                    || state.top_wild_declared_as == None
            } else {
                rules.can_play_graph.is_playable_on(card, top_of_discard)
            }
    } else {
        true
    }
}

//Since this uses rng, calling this in response to repeatable user input allows rng manipulation.
fn cpu_would_play(
    state: &mut in_game::State,
    rng: &mut Xs,
    rules: &Rules,
    player_id: PlayerID,
) -> Option<u8> {
    let playable: Vec<(usize, Card)> = {
        let hand = state.get_hand(player_id);
        hand.iter()
            .cloned()
            .enumerate()
            .filter(|(_, card)| can_play(state, rules, card))
            .collect()
    };

    let sim_state = get_sim_state(state, rng, player_id);

    let mut indexes_and_hand_deltas = Vec::with_capacity(playable.len());

    for (i, card) in playable {
        let delta = get_hand_delta(&sim_state, rules, rng, player_id, card);

        indexes_and_hand_deltas.push((i, delta));
    }

    //if we make repeated decisions with equal weight, sometimes choose differently.
    xs_shuffle(rng, &mut indexes_and_hand_deltas);

    indexes_and_hand_deltas.sort_by_key(
        |&(_, delta)| -delta, //highest negative delta to end
    );

    indexes_and_hand_deltas.pop().map(|(i, _)| i as u8)
}

fn get_hand_delta(
    state: &in_game::State,
    rules: &Rules,
    rng: &mut Xs,
    player_id: PlayerID,
    card: Card,
) -> i8 {
    let original = state.get_hand(player_id).len() as i8;
    let mut s: in_game::State = (*state).clone();

    animations::play_to_discard_parts(&mut s, rules, rng, &mut None, card);

    let new = s.get_hand(player_id).len() as i8;

    new - original
}

fn get_sim_state(
    state: &in_game::State,
    rng: &mut Xs,
    player_id: PlayerID,
) -> in_game::State {
    // We don't want the cpu to cheat, so don't let them see what is really on top of the deck.
    // or which cards are in each player's hand either. So put all unknown cards in one pile
    // then shuffle them randomly to each unknown zone, maintaining the original amounts.
    let mut pile = Vec::with_capacity(DECK_SIZE as usize);

    macro_rules! add_to_pile {
        ($e:expr) => {
            pile.extend($e.clone().drain());
        };
    }

    add_to_pile!(state.deck);
    add_to_pile!(state.discard);

    match player_id {
        0 => {
            add_to_pile!(state.cpu_hands[1]);
            add_to_pile!(state.cpu_hands[2]);
            add_to_pile!(state.hand);
        }
        1 => {
            add_to_pile!(state.cpu_hands[0]);
            add_to_pile!(state.cpu_hands[2]);
            add_to_pile!(state.hand);
        }
        2 => {
            add_to_pile!(state.cpu_hands[1]);
            add_to_pile!(state.cpu_hands[2]);
            add_to_pile!(state.hand);
        }
        MAX_PLAYER_ID => {
            add_to_pile!(state.cpu_hands[0]);
            add_to_pile!(state.cpu_hands[1]);
            add_to_pile!(state.cpu_hands[2]);
        }
        _ => invariant_violation!({}, "get_sim_state called with bad PlayerID"),
    }

    xs_shuffle(rng, &mut pile);

    let mut output = in_game::State {
        current_player: state.current_player,
        top_wild_declared_as: state.top_wild_declared_as,
        ..d!()
    };

    macro_rules! deal_into {
        ($hand:ident) => {
            let pile_drain = pile.drain(..state.$hand.len() as usize);
            output.$hand.fill(pile_drain);
        };
        ($hand:ident, [$index:expr]) => {
            let pile_drain = pile.drain(..state.$hand[$index].len() as usize);
            output.$hand[$index].fill(pile_drain);
        };
    }

    deal_into!(deck);
    deal_into!(discard);

    match player_id {
        0 => {
            deal_into!(cpu_hands, [1]);
            deal_into!(cpu_hands, [2]);
            deal_into!(hand);
        }
        1 => {
            deal_into!(cpu_hands, [0]);
            deal_into!(cpu_hands, [2]);
            deal_into!(hand);
        }
        2 => {
            deal_into!(cpu_hands, [1]);
            deal_into!(cpu_hands, [2]);
            deal_into!(hand);
        }
        MAX_PLAYER_ID => {
            deal_into!(cpu_hands, [0]);
            deal_into!(cpu_hands, [1]);
            deal_into!(cpu_hands, [2]);
        }
        _ => invariant_violation!({}, "get_sim_state called with bad PlayerID"),
    }

    output
}

fn incremented_current_player(state: &in_game::State) -> PlayerID {
    if state.current_player >= MAX_PLAYER_ID {
        0
    } else {
        state.current_player + 1
    }
}

fn take_turn(game_state: &mut GameState, input: Input, speaker: &mut Speaker) {
    let state = &mut game_state.in_game;

    let rules = &game_state.rules;
    let event_log = &mut game_state.event_log;
    let rng = &mut game_state.rng;

    //Doing this here and assigning to `current_player` later means that to start on a given
    //player we need to set `current_player` to the previous player, but it means that
    //`current_player` is set to the same player only during the actual player's turn and for the
    //entire turn, rather than just until we get to the increment.
    let next_player = incremented_current_player(state);
    match next_player {
        p if is_cpu_player(p) => {
            state.current_player = next_player;
            if let Some(index) = cpu_would_play(state, rng, rules, p) {
                animations::add_discard_animation(state, index, event_log, rules);
            } else {
                animations::add_draw_animation(state, event_log, rng);
            }
        }
        PLAYER_ID => {
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
                    state.current_player = next_player;
                    animations::add_discard_animation(state, index, event_log, rules);
                } else {
                    //TODO good feedback. Tint the card red or shake it or something?
                }
            } else if input.pressed_this_frame(Button::B) {
                state.current_player = next_player;
                animations::add_draw_animation(state, event_log, rng);
            }
        }
        _id => {
            invariant_violation!("`current_player` was set to invalid player ID {}!", _id);
        }
    }

    let winners: Vec<PlayerID> = all_player_ids()
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
            card_set,
            ref changes,
            ..
        } if changes.len() > 0 => {
            apply_when_played_changes(state, card_set, changes.clone(), PLAYER_ID);
            state.start_new_round();
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
            apply_wild_change(state, wild, PLAYER_ID);
            state.start_new_round();
        }
    }
}

fn update_can_play_graph(state: &mut GameState) {
    match choose_can_play_graph(state) {
        ref x if x.len() == 0 => {
            //wait until they choose
        }
        changes => {
            apply_can_play_graph_changes(state, changes, PLAYER_ID);
            state.start_new_round();
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

    if input.pressed_this_frame(Button::START) {
        state.log_heading = match state.log_heading {
            LogHeading::Up => LogHeading::Down,
            LogHeading::Down => LogHeading::Up,
        };
    }

    if state.log_height > 0 {
        if input.pressed_this_frame(Button::UP) {
            state.event_log.top_index = state.event_log.top_index.saturating_sub(1);
        //TODO feedback when you hit the top edge
        } else if input.pressed_this_frame(Button::DOWN) {
            if state.event_log.top_index < state.event_log.len() {
                state.event_log.top_index += 1;
            } else {
                //TODO feedback when you hit the bottom edge
            }
        } else if input.pressed_this_frame(Button::A) {
            state.event_log.jump_backward();
        } else if input.pressed_this_frame(Button::B) {
            state.event_log.jump_forward();
        }
    } else if state.choice.is_idle() {
        if state.animations_settled() {
            if state.in_game.no_winners_yet() {
                take_turn(state, input, speaker);
            }
        } else {
            animations::advance(state, speaker);

            move_cursor(&mut state.in_game, input, speaker);
        }
    }
}

fn print_number_below_card(framebuffer: &mut Framebuffer, number: u8, x: u8, y: u8) {
    framebuffer.print_single_line_number(
        number as usize,
        x + (card::WIDTH / 2) - FONT_ADVANCE,
        y + card::HEIGHT + FONT_SIZE / 2,
        BLACK_INDEX,
    );
}

#[inline]
pub fn render_in_game(framebuffer: &mut Framebuffer, state: &in_game::State) {
    for hand in state.cpu_hands.iter() {
        draw_hand(framebuffer, hand, Face::Down);
    }

    let deck_len = state.deck.len();
    if deck_len > 0 {
        framebuffer.draw_card_back(DECK_X, DECK_Y);
    }

    print_number_below_card(framebuffer, deck_len, DECK_X, DECK_Y);

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

    print_number_below_card(framebuffer, state.discard.len(), DISCARD_X, DISCARD_Y);

    draw_hand_with_cursor(framebuffer, &state.hand, state.hand_index as usize);

    for &CardAnimation {
        card,
        completion_action,
        ..
    } in state.card_animations.iter()
    {
        match completion_action {
            Action::MoveToDeck | Action::MoveToDiscard | Action::MoveToHand(_) => {
                framebuffer.draw_card_back(card.x, card.y)
            }
            Action::PlayToDiscard | Action::SelectWild(_) => {
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

    invariant_assert_eq!(state.in_game.missing_cards(), vec![0; 0]);

    framebuffer.clearTo(GREEN);

    if state.show_rules || input.pressed_this_frame(Button::SELECT) {
        state.show_rules = true;
        show_rules_screen(framebuffer, state, input, speaker);
        return;
    } else {
        update(state, input, speaker);

        render_in_game(framebuffer, &state.in_game);
    }

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
