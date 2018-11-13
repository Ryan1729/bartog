use can_play;
use common::{
    bytes_lines, bytes_reflow, slice_until_first_0, CardFlags, UIContext, DECK_SIZE, RANK_FLAGS, *,
};
use in_game;

use std::collections::VecDeque;

use rand::{SeedableRng, XorShiftRng};

use std::cmp::min;

#[derive(Debug)]
pub struct EventLog {
    pub buffer: VecDeque<EventLine>,
    pub top_index: usize,
}

type EventLine = [u8; EventLog::WIDTH];

impl EventLog {
    const WIDTH: usize = NINE_SLICE_MAX_INTERIOR_WIDTH_IN_CHARS as usize;
    const HEIGHT: usize = NINE_SLICE_MAX_INTERIOR_HEIGHT_IN_CHARS as usize;

    const BUFFER_SIZE: usize = 1024;

    pub fn new() -> Self {
        let buffer = VecDeque::with_capacity(EventLog::BUFFER_SIZE);
        EventLog {
            buffer,
            top_index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
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

        for i in 0..next.len() {
            next[i] = 0;
        }

        for i in 0..bytes.len() {
            next[i] = bytes[i];
        }
    }

    pub fn push_hr(&mut self) {
        self.push(&[b'-'; NINE_SLICE_MAX_INTERIOR_WIDTH_IN_CHARS as usize])
    }

    pub fn next_mut(&mut self) -> &mut EventLine {
        debug_assert!(EventLog::BUFFER_SIZE > 0);
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

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a [u8]> {
        self.buffer.iter().map(|line| slice_until_first_0(line))
    }

    pub fn get_window_slice<'a>(&'a self) -> impl Iterator<Item = &'a [u8]> {
        self.iter().skip(self.top_index).take(EventLog::HEIGHT)
    }
}

#[macro_export]
macro_rules! event_push {
    ($event_log:expr, $($byte_strings:tt)*) => {{
        $event_log.push(bytes_concat!($($byte_strings)*));
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
    pub flags: CardFlags,
    pub card: Card,
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
        match *self {
            Choice::NoChoice | Choice::Already(_) => true,
            _ => false,
        }
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
        //Status::InGame
        //For testing
        Status::RuleSelection
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
    pub when_played: CardChanges,
}

impl Rules {
    pub fn is_wild(&self, card: Card) -> bool {
        self.wild.has_card(card)
    }
}

pub struct CardChanges(pub [Vec<in_game::Change>; DECK_SIZE as usize]);

impl Default for CardChanges {
    fn default() -> Self {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        CardChanges(
            [
                Vec::new(), Vec::new(), Vec::new(), Vec::new(),
                Vec::new(), Vec::new(), Vec::new(), Vec::new(),
                Vec::new(), Vec::new(), Vec::new(), Vec::new(),
                Vec::new(), Vec::new(), Vec::new(), Vec::new(),
                Vec::new(), Vec::new(), Vec::new(), Vec::new(),
                Vec::new(), Vec::new(), Vec::new(), Vec::new(),
                Vec::new(), Vec::new(), Vec::new(), Vec::new(),
                Vec::new(), Vec::new(), Vec::new(), Vec::new(),
                Vec::new(), Vec::new(), Vec::new(), Vec::new(),
                Vec::new(), Vec::new(), Vec::new(), Vec::new(),
                Vec::new(), Vec::new(), Vec::new(), Vec::new(),
                Vec::new(), Vec::new(), Vec::new(), Vec::new(),
                Vec::new(), Vec::new(), Vec::new(), Vec::new(),
            ]
        )
    }
}

impl CardChanges {
    pub fn get_changes(&self, card: Card) -> &Vec<in_game::Change> {
        &self.0[card as usize]
    }
    pub fn get_changes_mut(&mut self, card: Card) -> &mut Vec<in_game::Change> {
        &mut self.0[card as usize]
    }
}

impl Default for Rules {
    fn default() -> Self {
        Rules {
            wild: CardFlags::new(RANK_FLAGS[Ranks::EIGHT as usize]),
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
    pub rng: XorShiftRng,
    pub event_log: EventLog,
    pub log_height: u8,
    pub log_heading: LogHeading,
}

impl GameState {
    pub fn new(seed: [u8; 16]) -> GameState {
        let event_log = EventLog::new();
        GameState::new_with_previous(seed, d!(), d!(), event_log)
    }

    pub fn new_with_previous(
        seed: [u8; 16],
        status: Status,
        rules: Rules,
        mut event_log: EventLog,
    ) -> GameState {
        // We always want to log the seed, if there is a logger available, so use the function,
        // not the macro.
        log(&format!("{:?}", seed));

        event_log.push_hr();
        //TODO keep track of round count and change to "started round N"
        //TODO scroll the event log to the start of the new round?
        event_log.push(b"started a new round.");

        let mut rng = XorShiftRng::from_seed(seed);

        GameState {
            in_game: in_game::State::new(&mut rng),
            choice: Choice::NoChoice,
            rules,
            status,
            context: UIContext::new(),
            rng,
            event_log,
            log_height: 0,
            log_heading: LogHeading::Up,
        }
    }

    pub fn winners(&self) -> &Vec<PlayerID> {
        &self.in_game.winners
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
