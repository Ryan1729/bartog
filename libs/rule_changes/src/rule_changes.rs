use common::*;
use game_state::{can_play, CardFlags, GameState, Status, RULE_TYPES};
use rand::Rng;

struct CardFlagsDelta {
    pub additions: Vec<Card>,
    pub removals: Vec<Card>,
}

impl CardFlagsDelta {
    fn new(previous_flags: CardFlags, new_flags: CardFlags) -> Self {
        let mut additions = Vec::new();
        let mut removals = Vec::new();

        for card in 0..DECK_SIZE {
            let mask = 1 << card as usize;
            let p_edge = mask & previous_flags.get_bits() != 0;
            let n_edge = mask & new_flags.get_bits() != 0;

            match (p_edge, n_edge) {
                (true, false) => removals.push(card),
                (false, true) => additions.push(card),
                _ => {}
            }
        }

        CardFlagsDelta {
            additions,
            removals,
        }
    }
}

pub fn reset(state: &mut GameState) {
    let status = {
        let mut status = Status::InGame;

        let player_id = state.player_id();

        for &id in state.winners.clone().iter() {
            if id >= player_id {
                status = Status::RuleSelection;
                continue;
            }

            add_cpu_rule(state, id);
        }

        status
    };

    let old_log = state.event_log.take();
    let old_rules = state.rules.take();

    *state =
        GameState::new_with_previous(state.rng.gen(), status, old_rules, state.logger, old_log);
}

fn add_cpu_rule(state: &mut GameState, player: PlayerID) {
    let rule_type = {
        let index = state.rng.gen_range(0, RULE_TYPES.len());
        RULE_TYPES[index]
    };

    match rule_type {
        Status::RuleSelectionWild => add_cpu_wild_change(state, player),
        Status::RuleSelectionCanPlay => add_cpu_can_play_graph_change(state, player),
        Status::RuleSelection | Status::InGame => {
            invariant_violation!("add_cpu_rule generated a non-rule type status");
        }
    }
}

fn add_cpu_wild_change(state: &mut GameState, player: PlayerID) {
    let count = state.rng.gen_range(0, 9);
    let cards = gen_cards(&mut state.rng, count);
    let new_wild = CardFlags::from_cards(cards);

    apply_wild_change(state, new_wild, player);
}

pub fn apply_wild_change(state: &mut GameState, new_wild: CardFlags, player: PlayerID) {
    //logging
    add_rule_change_log_header(state, player);

    let CardFlagsDelta {
        additions,
        removals,
    } = CardFlagsDelta::new(state.rules.wild, new_wild);

    let pronoun = state.get_pronoun(player);

    match (additions.len() > 0, removals.len() > 0) {
        (false, false) => {}
        (true, false) => {
            let additions_string = get_card_list(&additions);
            let text = &[
                pronoun.as_bytes(),
                b" made the following cards wild: ",
                additions_string.as_bytes(),
                b".",
            ]
                .concat();
            state.event_log.push(text);
        }
        (false, true) => {
            let removals_string = get_card_list(&removals);
            let text = &[
                pronoun.as_bytes(),
                b" made these cards not wild: ",
                removals_string.as_bytes(),
                b".",
            ]
                .concat();
            state.event_log.push(text);
        }
        (true, true) => {
            let additions_string = get_card_list(&additions);
            let removals_string = get_card_list(&removals);
            let text = &[
                pronoun.as_bytes(),
                b" made the following cards wild: ",
                additions_string.as_bytes(),
                b". but ",
                pronoun.as_bytes(),
                b" also made these cards not wild: ",
                removals_string.as_bytes(),
                b".",
            ]
                .concat();
            state.event_log.push(text);
        }
    };

    /////////

    state.rules.wild = new_wild;
}

fn add_cpu_can_play_graph_change(state: &mut GameState, player: PlayerID) {
    //TODO add single-strongly connected component checking and start
    //generating non-additive changes;
    let count = state.rng.gen_range(5, DECK_SIZE as usize);
    let cards = gen_cards(&mut state.rng, count);

    let mut changes = Vec::with_capacity(count);
    for card in cards {
        let old_edges = state.rules.can_play_graph.get_edges(card);
        let edges = state.rng.gen::<CardFlags>() | old_edges;

        changes.push(can_play::Change::new(edges, card));
    }

    apply_can_play_graph_changes(state, changes, player);
}

pub fn apply_can_play_graph_changes(
    state: &mut GameState,
    changes: Vec<can_play::Change>,
    player: PlayerID,
) {
    //TODO enforce a single strongly connected component in the graph

    let mut flattened_changes = [None; DECK_SIZE as usize];

    for &change in changes.iter() {
        let index = change.card() as usize;

        flattened_changes[index] = Some(change);
    }

    for possible_change in flattened_changes.into_iter() {
        if let Some(change) = possible_change {
            let new_card = change.card();
            let new_edges = change.edges();

            //logging
            add_rule_change_log_header(state, player);

            let previous_edges = state.rules.can_play_graph.get_edges(new_card);
            let CardFlagsDelta {
                additions,
                removals,
            } = CardFlagsDelta::new(previous_edges, new_edges);

            let pronoun = state.get_pronoun(player);
            let card_string = get_card_string(new_card);

            match (additions.len() > 0, removals.len() > 0) {
                (false, false) => {}
                (true, false) => {
                    let additions_string = get_suit_rank_pair_list(&additions);
                    let text = &[
                        pronoun.as_bytes(),
                        b" allowed the ",
                        card_string.as_bytes(),
                        b" to be played on the following cards: ",
                        additions_string.as_bytes(),
                        b".",
                    ]
                        .concat();
                    state.event_log.push(text);
                }
                (false, true) => {
                    let removals_string = get_suit_rank_pair_list(&removals);
                    let text = &[
                        pronoun.as_bytes(),
                        b" prevented the ",
                        card_string.as_bytes(),
                        b" from being played on the following cards: ",
                        removals_string.as_bytes(),
                        b".",
                    ]
                        .concat();
                    state.event_log.push(text);
                }
                (true, true) => {
                    let additions_string = get_suit_rank_pair_list(&additions);
                    let removals_string = get_suit_rank_pair_list(&removals);
                    let text = &[
                        pronoun.as_bytes(),
                        b" allowed the ",
                        card_string.as_bytes(),
                        b" to be played on the following cards: ",
                        additions_string.as_bytes(),
                        b". but ",
                        pronoun.as_bytes(),
                        b" also prevented it from being played on the following cards: ",
                        removals_string.as_bytes(),
                        b".",
                    ]
                        .concat();
                    state.event_log.push(text);
                }
            };

            /////////

            state.rules.can_play_graph.set_edges(new_card, new_edges);
        }
    }
}

fn add_rule_change_log_header(state: &mut GameState, player: PlayerID) {
    state.event_log.push_hr();

    let player_name = state.player_name(player);

    let text = &[player_name.as_bytes(), b" changed the rules as follows:"].concat();

    state.event_log.push(text);
}