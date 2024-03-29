use crate::{
    can_play,
    in_game,
};
use common::{bytes_lines, bytes_reflow, slice_until_first_0, CardFlags, UIContext, RANK_FLAGS, xs::{Xs, Seed}, *};

use std::collections::VecDeque;
use std::cmp::min;

#[derive(Debug)]
pub struct EventLog {
    pub buffer: VecDeque<EventLine>,
    pub top_index: usize,
}

impl Default for EventLog {
    fn default() -> EventLog {
        let buffer = VecDeque::with_capacity(EventLog::BUFFER_SIZE);
        EventLog {
            buffer,
            top_index: 0,
        }
    }
}

type EventLine = [u8; EventLog::WIDTH];

impl EventLog {
    const WIDTH: usize = NINE_SLICE_MAX_INTERIOR_WIDTH_IN_CHARS as usize;
    const HEIGHT: usize = NINE_SLICE_MAX_INTERIOR_HEIGHT_IN_CHARS as usize;

    const BUFFER_SIZE: usize = 1 << 11;

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.top_index = 0;
    }

    pub fn push(&mut self, bytes: &[u8]) {
        let reflowed = bytes_reflow(bytes, EventLog::WIDTH);

        for line in bytes_lines(&reflowed) {
            debug_assert!(line.len() <= EventLog::WIDTH);
            self.push_line(line);
        }
    }

    pub fn push_line(&mut self, bytes: &[u8]) {
        let bytes = &bytes[..min(bytes.len(), EventLog::WIDTH)];

        let next = self.next_mut();

        for e in next.iter_mut() {
            *e = 0;
        }

        next[..bytes.len()].copy_from_slice(bytes);
    }

    pub fn push_hr(&mut self) {
        self.push(&[b'-'; NINE_SLICE_MAX_INTERIOR_WIDTH_IN_CHARS as usize])
    }

    pub fn next_mut(&mut self) -> &mut EventLine {
        #[allow(unknown_lints, clippy::assertions_on_constants)]
        const _: () = assert!(EventLog::BUFFER_SIZE > 0);

        debug_assert!(self.buffer.len() <= EventLog::BUFFER_SIZE);

        if self.is_full() {
            self.buffer.pop_front();
        }

        self.buffer.push_back([0; EventLog::WIDTH]);

        self.buffer.back_mut().unwrap()
    }

    pub fn is_full(&self) -> bool {
        self.buffer.len() >= EventLog::BUFFER_SIZE
    }

    pub fn iter(&self) -> impl Iterator<Item = &[u8]> {
        self.buffer.iter().map(|line| slice_until_first_0(line))
    }

    pub fn get_window_slice(&self) -> impl Iterator<Item = &[u8]> {
        self.iter().skip(self.top_index).take(EventLog::HEIGHT)
    }

    pub fn is_at_hr(&self) -> bool {
        let line = self.buffer[self.top_index];

        line.iter().all(|&c| c == b'-')
    }

    pub fn jump_backward(&mut self) {
        if self.top_index == 0 {
            self.top_index = self.len() - 1;
            return;
        }

        loop {
            self.top_index = self.top_index.saturating_sub(1);

            if self.top_index == 0 || self.is_at_hr() {
                break;
            }
        }
    }
    pub fn jump_forward(&mut self) {
        if self.top_index == self.len() - 1 {
            self.top_index = 0;
            return;
        }

        loop {
            self.top_index += 1;

            if self.top_index == self.len() - 1 || self.is_at_hr() {
                break;
            }
        }
    }
}

#[macro_export]
macro_rules! event_push {
    ($event_log:expr, $($byte_strings:tt)*) => {{
        $event_log.push(bytes_concat!($($byte_strings)*));
    }}
}

#[macro_export]
macro_rules! optionally_event_push {
    ($event_log:expr, $($byte_strings:tt)*) => {{
        if let Some(e) = $event_log {
            event_push!(e, $($byte_strings)*);
        }
    }}
}

impl Empty for EventLog {
    fn empty() -> Self {
        EventLog {
            buffer: VecDeque::with_capacity(0),
            top_index: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CardFlagsChoiceState {
    original_flags: CardFlags,
    pub flags: CardFlags,
    pub card: Card,
    pub done: bool,
}

impl CardFlagsChoiceState {
    pub fn new(original_flags: CardFlags) -> Self {
        CardFlagsChoiceState {
            original_flags,
            flags: original_flags,
            card: d!(),
            done: d!(),
        }
    }
}

impl CardFlagsChoiceState {
    pub fn get_chosen(&self) -> Option<CardFlags> {
        if self.done {
            Some(self.flags)
        } else {
            None
        }
    }
}

implement!(BorrowPairMut<Card, CardFlags> for CardFlagsChoiceState: s, (s.card, s.flags));

impl CardFlagsSubChoice for CardFlagsChoiceState {
    fn mark_done(&mut self) {
        self.done = true;
    }
    fn reset(&mut self) {
        self.flags = self.original_flags;
    }
}

#[derive(Clone, Debug)]
pub enum Choice {
    NoChoice,
    Already(Chosen),
    OfCanPlayGraph(can_play::ChoiceState),
    OfCardFlags(CardFlagsChoiceState),
    OfInGameChanges(in_game::ChoiceState),
    OfStatus,
    OfSuit,
    OfBool,
    OfUnit,
}

impl Choice {
    pub fn is_idle(&self) -> bool {
        matches!(*self, Choice::NoChoice | Choice::Already(_))
    }
}

impl Default for Choice {
    fn default() -> Self {
        Choice::NoChoice
    }
}

impl Empty for Choice {
    fn empty() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug)]
pub enum Chosen {
    InGameChanges(in_game::ChoiceState),
    CanPlayGraph(Vec<can_play::Change>),
    CardFlags(CardFlags),
    Status(Status),
    Suit(Suit),
    Bool(bool),
    Unit(()),
}

pub enum LogHeading {
    Up,
    Down,
}

#[derive(Copy, Clone, Debug)]
pub enum Status {
    InGame,
    RuleSelection,
    RuleSelectionCanPlay,
    RuleSelectionWild,
    RuleSelectionWhenPlayed,
}

impl Default for Status {
    fn default() -> Self {
        Status::InGame
    }
}

pub const RULE_TYPES: [Status; 3] = [
    Status::RuleSelectionCanPlay,
    Status::RuleSelectionWild,
    Status::RuleSelectionWhenPlayed,
];

pub fn get_status_text(status: Status) -> &'static str {
    match status {
        Status::InGame => "InGame!?",
        Status::RuleSelection => "RuleSelection!?",
        Status::RuleSelectionCanPlay => "card playability",
        Status::RuleSelectionWild => "wildness",
        Status::RuleSelectionWhenPlayed => "when played",
    }
}

pub struct Rules {
    pub can_play_graph: can_play::Graph,
    pub wild: CardFlags,
    pub when_played: CardChangeTable,
}

impl Rules {
    pub fn is_wild(&self, card: Card) -> bool {
        self.wild.has_card(card)
    }
}

type Generation = u32;

#[derive(Default)]
pub struct CardChanges {
    changes: Vec<in_game::Change>,
    generation: Generation,
}

use std::collections::HashMap;

pub struct CardChangeTable {
    map: HashMap<CardFlags, CardChanges>,
    index: HashMap<Card, Vec<CardFlags>>,
    next_generation: Generation,
}

impl Default for CardChangeTable {
    fn default() -> Self {
        CardChangeTable{
            map: HashMap::new(),
            index: HashMap::with_capacity(DECK_SIZE as usize),
            next_generation: 1,
        }
    }
}

impl CardChangeTable {
    pub fn get_card_changes(&self, card: Card) -> impl Iterator<Item = in_game::Change> {
        let output: Vec<in_game::Change> = self
            .index
            .get(&card)
            .map(|flags: &Vec<CardFlags>| {
                //we assume tese flags are already in the right order.
                flags
                    .iter()
                    .flat_map(|f| {
                        self.map
                            .get(f)
                            .into_iter()
                            .flat_map(|c| c.changes.iter())
                    })
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        output.into_iter()
    }
    pub fn get_card_flags_changes(
        &self,
        card_flags: CardFlags,
    ) -> impl Iterator<Item = in_game::Change> + '_ {
        self
            .map
            .get(&card_flags)
            .into_iter()
            .flat_map(|c| c.changes.iter())
            .cloned()
    }
    pub fn set_changes(&mut self, card_flags: CardFlags, changes: Vec<in_game::Change>) {
        self.map.insert(
            card_flags,
            CardChanges {
                changes,
                generation: self.next_generation,
            },
        );
        self.next_generation += 1;

        for card in card_flags {
            let flag_vec = self.index.entry(card).or_default();
            let map = &self.map;
            let search_result = flag_vec.binary_search_by_key(
                &CardChangeTable::get_flag_sort_key(map, &card_flags),
                |flags| CardChangeTable::get_flag_sort_key(map, flags),
            );

            match search_result {
                Ok(_) => {}
                Err(i) => flag_vec.insert(i, card_flags),
            };
        }
    }

    fn get_flag_sort_key(map: &HashMap<CardFlags, CardChanges>, flags: &CardFlags) -> Generation {
        map.get(flags)
            .map(|ch| ch.generation)
            .unwrap_or(Generation::max_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_changes_with_ALL_FLAGS_causes_get_card_changes_to_return_for_any_card() {
        let mut table = CardChangeTable::default();

        let changes: Vec<in_game::Change> = in_game::RelativePlayer::all_values()
            .into_iter()
            .map(in_game::Change::CurrentPlayer)
            .collect();

        table.set_changes(CardFlags::new(ALL_FLAGS), changes.clone());

        let card = 0; //TODO make this a quickcheck test that selects a card

        let actual: Vec<in_game::Change> = table.get_card_changes(card).collect();

        assert_eq!(changes, actual);
    }
}

impl Default for Rules {
    fn default() -> Self {
        Rules {
            wild: CardFlags::new(RANK_FLAGS[ranks::EIGHT as usize]),
            can_play_graph: d!(),
            when_played: d!(),
        }
    }
}

use std::mem;

impl Empty for Rules {
    fn empty() -> Self {
        Rules {
            can_play_graph: unsafe { mem::zeroed() },
            wild: unsafe { mem::zeroed() },
            when_played: d!(),
        }
    }
}

pub struct GameState {
    pub in_game: in_game::State,
    pub choice: Choice,
    pub rules: Rules,
    pub status: Status,
    pub context: UIContext,
    pub rng: Xs,
    pub event_log: EventLog,
    pub log_heading: LogHeading,
    pub log_height: u8,
    pub round_count: u32,
    pub show_rules: bool,
}

impl GameState {
    pub fn new(seed: Seed) -> GameState {
        GameState::new_with_previous(seed, d!(), d!(), d!(), 0, true)
    }

    pub fn new_with_previous(
        seed: Seed,
        status: Status,
        rules: Rules,
        event_log: EventLog,
        round_count: u32,
        show_rules: bool,
    ) -> GameState {
        // We always want to log the seed, if there is a logger available, so use the function,
        // not the macro.
        log(&format!("{:?}", seed));

        let mut rng = xs::from_seed(seed);

        GameState {
            in_game: in_game::State::new(&mut rng),
            choice: Choice::NoChoice,
            rules,
            status,
            context: UIContext::default(),
            rng,
            event_log,
            log_heading: LogHeading::Up,
            log_height: 0,
            round_count,
            show_rules,
        }
    }

    pub fn winners(&self) -> &Vec<PlayerID> {
        &self.in_game.winners
    }

    pub fn start_new_round(&mut self) {
        self.status = Status::InGame;

        self.event_log.push_hr();

        self.round_count += 1;

        self.event_log
            .push(format!("started round {}.", self.round_count).as_bytes());

        self.event_log.push_hr();
    }

    pub fn animations_settled(&self) -> bool {
        self.in_game.animations_settled()
    }

    pub fn round_is_over(&self) -> bool {
        self.in_game.round_is_over()
    }

    pub fn is_wild(&self, card: Card) -> bool {
        self.rules.is_wild(card)
    }
}
