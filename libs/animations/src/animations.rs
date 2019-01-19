use choices::choose_suit;
use common::*;
use game_state::{
    event_push,
    in_game::{self, CardMovement, Change, RelativeHand, RelativePlayer},
    optionally_event_push, EventLog, GameState, Rules,
};

use rand::Rng;

pub trait ApplyToState {
    fn apply_to_state<R: Rng>(
        &self,
        s: &mut in_game::State,
        rng: &mut R,
        e: &mut Option<&mut EventLog>,
    );
}

impl ApplyToState for Change {
    fn apply_to_state<R: Rng>(
        &self,
        state: &mut in_game::State,
        rng: &mut R,
        event_log: &mut Option<&mut EventLog>,
    ) {
        change_match! {*self, {
            v => v.apply_to_state(state, rng, event_log)
        }}
    }
}

impl ApplyToState for CardMovement {
    fn apply_to_state<R: Rng>(
        &self,
        state: &mut in_game::State,
        rng: &mut R,
        event_log: &mut Option<&mut EventLog>,
    ) {
        if self.source == self.target {
            let source_str = self.source.to_string();
            optionally_event_push!(
                event_log,
                b"cards moved from ",
                source_str.as_bytes(),
                b" ... back to",
                source_str.as_bytes(),
                b".",
            );
        }

        let players = self.affected.absolute_players(state.current_player);

        for player in players {
            let source_str = self.source.apply(player).to_string();
            let card = {
                if self.source == RelativeHand::Deck {
                    let source: &Hand = state.get_relative_hand_mut(self.source, player);
                    if source.len() == 0 {
                        state.reshuffle_discard(rng);
                    }
                }

                let source: &mut Hand = state.get_relative_hand_mut(self.source, player);

                if self.source == RelativeHand::Discard {
                    //the first card of a pile intuitively refers to the top.
                    source.inverse_remove_selected(self.selection)
                } else {
                    // the deck already works correctly.
                    source.remove_selected(self.selection)
                }
            };

            if let Some(card) = card {
                optionally_event_push!(
                    event_log,
                    player_name(player).as_bytes(),
                    b" moves ",
                    self.selection.to_string().as_bytes(),
                    b" from ",
                    source_str.as_bytes(),
                    b" to ",
                    self.target.to_string().as_bytes(),
                    b".",
                );

                let game_player_hand = RelativeHand::get_game_player_hand(state.current_player);
                if self.target == RelativeHand::Discard
                    || self.source == game_player_hand
                    || self.target == game_player_hand
                {
                    optionally_event_push!(
                        event_log,
                        b"the card was the ",
                        get_card_string(card.card).as_bytes(),
                        b".",
                    );
                }

                if self.target == RelativeHand::Discard
                    || (self.source == RelativeHand::Discard
                        && self.selection == CardSelection::NthModuloCount(nu8!(1)))
                {
                    state.top_wild_declared_as = None;
                }

                let (x, y) = state.get_new_card_position(self.target, player);

                state.card_animations.push(CardAnimation::new(
                    card,
                    x,
                    y,
                    get_move_action(self.target, player),
                ));
            } else {
                optionally_event_push!(
                    event_log,
                    player_name(player).as_bytes(),
                    b" tries to move ",
                    self.selection.to_string().as_bytes(),
                    b" from ",
                    source_str.as_bytes(),
                    b" to ",
                    self.target.to_string().as_bytes(),
                    b" but ",
                    source_str.as_bytes(),
                    b" didn't have enough cards.",
                );
            }
        }
    }
}

fn get_move_action(hand: RelativeHand, player: PlayerID) -> Action {
    match hand {
        RelativeHand::Deck => Action::MoveToDeck,
        RelativeHand::Discard => Action::MoveToDiscard,
        RelativeHand::Player(p) => Action::MoveToHand(p.apply(player)),
    }
}

impl ApplyToState for RelativePlayer {
    fn apply_to_state<R: Rng>(
        &self,
        state: &mut in_game::State,
        _rng: &mut R,
        event_log: &mut Option<&mut EventLog>,
    ) {
        let new_player = self.apply(state.current_player);
        let new_player_str = new_player.to_string();

        optionally_event_push!(
            event_log,
            b"it becomes ",
            new_player_str.as_bytes(),
            b"'s turn"
        );

        state.current_player =
                    //apply Previous to undo the autonatic incrementation that will happen later
                        RelativePlayer::Previous.apply(new_player);
    }
}

pub fn play_to_discard(state: &mut GameState, card: Card) {
    play_to_discard_parts(
        &mut state.in_game,
        &state.rules,
        &mut state.rng,
        &mut Some(&mut state.event_log),
        card,
    )
}
pub fn play_to_discard_parts<R: Rng>(
    in_game: &mut in_game::State,
    rules: &Rules,
    rng: &mut R,
    event_log: &mut Option<&mut EventLog>,
    card: Card,
) {
    if !rules.is_wild(card) {
        in_game.top_wild_declared_as = None;
    }

    in_game.discard.push(card);
    for change in rules.when_played.get_card_changes(card) {
        if let Some(event_log) = event_log {
            event_push!(
                event_log,
                b"change" as &[u8],
                format!("{:?}", change).as_bytes() as &[u8]
            );
        }

        change.apply_to_state(in_game, rng, event_log);
    }
}

fn log_wild_selection(state: &mut GameState, player: PlayerID) {
    if let Some(suit) = state.in_game.top_wild_declared_as {
        let player_name = player_name(player);
        let suit_str = get_suit_str(suit);
        event_push!(
            state.event_log,
            player_name.as_bytes(),
            b" selected ",
            suit_str.as_bytes(),
            b".",
        );
    }
}

pub fn advance(state: &mut GameState) {
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
                Action::PlayToDiscard => {
                    play_to_discard(state, card);
                }
                Action::SelectWild(player_id) => {
                    if is_cpu_player(player_id) {
                        state.in_game.top_wild_declared_as = {
                            let hand = state.in_game.get_hand(player_id);
                            hand.most_common_suit()
                        };
                        log_wild_selection(state, player_id);
                        play_to_discard(state, card);
                    } else {
                        if let Some(suit) = choose_suit(state) {
                            state.in_game.top_wild_declared_as = Some(suit);
                            log_wild_selection(state, player_id);
                            play_to_discard(state, card);
                        } else {
                            //wait until they choose
                            animation.card.x = last_pos.0;
                            animation.card.y = last_pos.1;
                            state.in_game.card_animations.push(animation);
                        }
                    }
                }
                Action::MoveToDeck => {
                    state.in_game.deck.push(card);
                }
                Action::MoveToDiscard => {
                    state.in_game.discard.push(card);
                }
                Action::MoveToHand(player_id) => {
                    state.in_game.get_hand_mut(player_id).push(card);
                }
            }
        }

        if i == 0 {
            break;
        }
        i -= 1;
    }
}

pub fn add_discard_animation(
    state: &mut in_game::State,
    card_index: u8,
    event_log: &mut EventLog,
    rules: &Rules,
) {
    let player = state.current_player;
    if let Some(card) = state.remove_positioned_card(player, card_index) {
        let player_name = player_name(player);

        let card_string = get_card_string(card.card);

        let rank = get_rank(card.card);

        event_push!(
            event_log,
            player_name.as_bytes(),
            if rank == ranks::ACE || rank == ranks::EIGHT {
                b" played an "
            } else {
                b" played a "
            },
            card_string.as_bytes(),
            b".",
        );

        let animation = if rules.is_wild(card.card) {
            CardAnimation::new(card, DISCARD_X, DISCARD_Y, Action::SelectWild(player))
        } else {
            CardAnimation::new(card, DISCARD_X, DISCARD_Y, Action::PlayToDiscard)
        };

        state.card_animations.push(animation);
    }
}

pub fn add_draw_animation<R: Rng>(
    state: &mut in_game::State,
    event_log: &mut EventLog,
    rng: &mut R,
) {
    let player = state.current_player;
    if let Some(animation) = get_draw_animation(state, player, event_log, rng) {
        state.card_animations.push(animation);
    }
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
            state.reshuffle_discard(rng)?;

            state.deck.draw()
        }
    }?;

    let (x, y) = get_card_position(spread, len + 1, len);

    let player_name = player_name(player);

    event_push!(event_log, player_name.as_bytes(), b" drew a card.");

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
