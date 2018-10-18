use card_flags::{CardFlags, RANK_FLAGS, SUIT_FLAGS};
use common::{
    bytes_lines, bytes_reflow, slice_until_first_0, CardAnimation, UIContext, DECK_SIZE, *,
};
use platform_types::{log, Logger};

use std::collections::VecDeque;
use std::fmt;

use rand::distributions::{Distribution, Standard};
use rand::{Rng, SeedableRng, XorShiftRng};

macro_rules! implement {
    (BorrowMut<$borrowed:ty> for $implementing:ty: $that:ident, $ref_expr:expr) => {
        use std::borrow::Borrow;
        impl Borrow<$borrowed> for $implementing {
            fn borrow(&self) -> &$borrowed {
                let $that = self;
                &$ref_expr
            }
        }

        use std::borrow::BorrowMut;
        impl BorrowMut<$borrowed> for $implementing {
            fn borrow_mut(&mut self) -> &mut $borrowed {
                let $that = self;
                &mut $ref_expr
            }
        }
    };
    (<$a:lifetime> BorrowMut<$borrowed:ty> for $implementing:ty: $that:ident, $ref_expr:expr) => {
        use std::borrow::Borrow;
        impl<$a> Borrow<$borrowed> for $implementing {
            fn borrow(&self) -> &$borrowed {
                let $that = self;
                &$ref_expr
            }
        }

        use std::borrow::BorrowMut;
        impl<$a> BorrowMut<$borrowed> for $implementing {
            fn borrow_mut(&mut self) -> &mut $borrowed {
                let $that = self;
                &mut $ref_expr
            }
        }
    };
}

impl GameState {
    pub fn remove_positioned_card(
        &mut self,
        playerId: PlayerID,
        card_index: u8,
    ) -> Option<PositionedCard> {
        let hand = self.get_hand_mut(playerId);
        hand.remove_if_present(card_index)
    }

    pub fn get_hand(&self, playerId: PlayerID) -> &Hand {
        let index = playerId as usize;
        let len = self.cpu_hands.len();
        if index < len {
            &self.cpu_hands[index]
        } else if index == len {
            &self.hand
        } else {
            invariant_violation!({ &self.discard }, "Could not find hand for {:?}", playerId)
        }
    }

    pub fn get_hand_mut(&mut self, playerId: PlayerID) -> &mut Hand {
        let index = playerId as usize;
        let len = self.cpu_hands.len();
        if index < len {
            &mut self.cpu_hands[index]
        } else if index == len {
            &mut self.hand
        } else {
            invariant_violation!(
                { &mut self.discard },
                "Could not find hand for {:?}",
                playerId
            )
        }
    }

    pub fn player_ids(&self) -> Vec<PlayerID> {
        // While we don't expect to change the maximum number of players,
        // we might allow a smaller minimum. So this seems like it should
        // stay here.
        (0..=self.cpu_hands.len())
            .map(|id| id as PlayerID)
            .collect()
    }

    pub fn get_pronoun(&self, playerId: PlayerID) -> String {
        let len = self.cpu_hands.len() as PlayerID;

        if playerId == len {
            "you".to_string()
        } else {
            "they".to_string()
        }
    }

    pub fn get_winner_text(&self) -> String {
        let winner_names: Vec<_> = self
            .winners
            .iter()
            .map(|&player| player_name(player))
            .collect();

        let mut winner_text = get_sentence_list(&winner_names);

        let suffix = if self.winners.len() == 1 && winner_text != "you" {
            " wins."
        } else if self.winners.len() > 3 {
            " all win."
        } else {
            " win."
        };

        winner_text.push_str(suffix);

        winner_text
    }
}

pub fn get_sentence_list<T: AsRef<str>>(elements: &[T]) -> String {
    let mut text = String::new();

    let len = elements.len();
    if len >= 2 {
        for i in 0..len {
            text.push_str(elements[i].as_ref());

            if i == len - 2 {
                text.push_str(", and ");
            } else if i < len - 2 {
                text.push_str(", ");
            }
        }
    } else if len == 1 {
        text.push_str(elements[0].as_ref());
    }

    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::*;

    #[ignore]
    #[test]
    fn test_get_sentence_list() {
        quickcheck(get_sentence_list_produces_expected_results as fn(Vec<String>) -> TestResult)
    }
    fn get_sentence_list_produces_expected_results(elements: Vec<String>) -> TestResult {
        if elements
            .iter()
            .any(|s| s.is_empty() || s.contains("and") || s.contains(","))
        {
            return TestResult::discard();
        }

        let result = get_sentence_list(&elements);

        let len = elements.len();
        let passes = if len == 0 {
            result.is_empty()
        } else if len == 1 {
            result == elements[0]
        } else if len == 2 {
            assert_eq!(result, format!("{}, and {}", elements[0], elements[1]));
            result == format!("{}, and {}", elements[0], elements[1])
        } else {
            result.matches(",").count() == len - 1 && result.matches(", and").count() == 1
        };

        if !passes {
            test_println!("Failed with: {}", result);
        }

        TestResult::from_bool(passes)
    }
}

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

pub mod can_play {
    use super::*;

    #[derive(Clone)]
    pub struct Graph {
        pub nodes: [CardFlags; DECK_SIZE as usize],
    }

    impl Graph {
        pub fn is_playable_on(&self, card: Card, top_of_discard: Card) -> bool {
            self.nodes[card as usize].has_card(top_of_discard)
        }

        pub fn get_edges(&self, card: Card) -> CardFlags {
            self.nodes[card as usize]
        }

        pub fn set_edges(&mut self, card: Card, edges: CardFlags) {
            self.nodes[card as usize] = edges;
        }
    }

    impl Default for Graph {
        fn default() -> Self {
            //Reminder:
            // the cards go from 0-51, in ascending rank order,
            // and in ♣ ♦ ♥ ♠ suit order (alphabetical)
            // A♣, 2♣, ... K♣, A♦, ..., A♥, ..., A♠, ..., K♠.
            let mut nodes = [CardFlags::default(); DECK_SIZE as usize];

            for suit in 0..SUIT_COUNT as usize {
                for rank in 0..RANK_COUNT as usize {
                    let i = rank + suit * RANK_COUNT as usize;

                    nodes[i] = CardFlags::new(SUIT_FLAGS[suit] | RANK_FLAGS[rank]);
                }
            }

            Graph { nodes }
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Change(u64);

    impl Change {
        pub fn new(edges: CardFlags, card: Card) -> Self {
            Change(((card as u64) << DECK_SIZE) | edges.get_bits())
        }

        pub fn edges(&self) -> CardFlags {
            CardFlags::new(self.0)
        }

        pub fn card(&self) -> Card {
            (self.0 >> DECK_SIZE as u64) as u8 & 0b0011_1111
        }
    }

    const RESET_ALL: Change = Change(-1i64 as u64);

    impl fmt::Debug for Change {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if *self == RESET_ALL {
                write!(f, "reset to default")?;
                return Ok(());
            }

            write!(
                f,
                "Card: {}, Edges: {:?}",
                get_card_string(self.card()),
                self.edges()
            )
        }
    }

    #[derive(Debug, Clone)]
    pub enum Layer {
        Card,
        Edges,
        Done,
    }

    impl Default for Layer {
        fn default() -> Self {
            Layer::Card
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct ChoiceState {
        pub changes: Vec<Change>,
        pub card: Card,
        pub edges: CardFlags,
        pub layer: Layer,
        pub scroll_card: Card,
    }

    implement!(BorrowMut<Card> for ChoiceState: s, s.card);

    impl CardSubChoice for ChoiceState {
        fn should_show_done_button(&self) -> bool {
            let changes_len = self.changes.len();
            changes_len > 0
        }
        fn mark_done(&mut self) {
            self.layer = Layer::Done;
        }
        fn next_layer(&mut self) {
            self.layer = Layer::Edges;
        }
        fn get_status_lines(&self, _card: Card) -> StatusLines {
            let changes_len = self.changes.len();
            [
                bytes_to_status_line(format!("{}", changes_len).as_bytes()),
                bytes_to_status_line(if changes_len == 1 {
                    b"change. "
                } else {
                    b"changes."
                }),
            ]
        }
    }
}

pub const MAX_PLAYER_ID: PlayerID = 3;
pub const PLAYER_ID_COUNT: usize = (MAX_PLAYER_ID + 1) as _;

pub fn all_player_ids() -> [PlayerID; PLAYER_ID_COUNT] {
    let mut output = [0; PLAYER_ID_COUNT];
    for i in 0..=MAX_PLAYER_ID {
        output[i as usize] = i;
    }
    output
}

pub fn player_name(playerId: PlayerID) -> String {
    if playerId < MAX_PLAYER_ID {
        format!("cpu {}", playerId)
    } else if playerId == MAX_PLAYER_ID {
        "you".to_string()
    } else {
        "???".to_string()
    }
}

pub mod in_game {
    use super::*;
    use std::fmt;

    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum Change {
        CurrentPlayer(CurrentPlayer),
        //CardLocation(CardLocation),
        //TopWild(TopWild),
    }

    impl AllValues for Change {
        //TODO write a procedural macro or something to make mainatining this easier.
        fn all_values() -> Vec<Self> {
            CurrentPlayer::all_values()
                .into_iter()
                .map(Change::CurrentPlayer)
                .collect()
        }
    }

    impl fmt::Debug for Change {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self)
        }
    }

    impl fmt::Display for Change {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Change::CurrentPlayer(v) => write!(f, "{}", v.to_string()),
            }
        }
    }

    impl Distribution<Change> for Standard {
        #[inline]
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Change {
            match rng.gen_range(0, 1) {
                _ => Change::CurrentPlayer(rng.gen()),
            }
        }
    }

    //This relies on MAX_PLAYER_ID being 3, and will require structural changes if it changes!
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct CurrentPlayer(u8);

    impl AllValues for CurrentPlayer {
        fn all_values() -> Vec<Self> {
            u8::all_values().into_iter().map(CurrentPlayer).collect()
        }
    }

    impl fmt::Display for CurrentPlayer {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "change whose turn it is based on the current player as follows: "
            )?;
            for &id in all_player_ids().into_iter() {
                write!(f, "{} -> {}", player_name(id), player_name(self.apply(id)))?;

                if id != MAX_PLAYER_ID {
                    write!(f, ", ")?;
                }
            }
            Ok(())
        }
    }

    impl Distribution<CurrentPlayer> for Standard {
        #[inline]
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CurrentPlayer {
            CurrentPlayer(rng.gen())
        }
    }

    impl CurrentPlayer {
        pub fn apply(&self, playerId: PlayerID) -> PlayerID {
            match playerId {
                0 => self.0 & 0b11,
                1 => (self.0 & 0b1100) >> 2,
                2 => (self.0 & 0b11_0000) >> 4,
                MAX_PLAYER_ID => (self.0 & 0b1100_0000) >> 6,
                _ => {
                    // The player is least likely to be annoyed with extra turns for them.
                    MAX_PLAYER_ID
                }
            }
        }
    }

    #[derive(Clone, Debug)]
    pub enum Layer {
        Card,
        Changes,
        Done,
    }

    impl Default for Layer {
        fn default() -> Self {
            Layer::Card
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct ChoiceState {
        pub changes: Vec<Change>,
        pub layer: Layer,
        pub card: Card,
        pub left_scroll: u8,
        pub right_scroll: u8,
        pub description: Vec<u8>,
    }

    pub struct ChoiceStateAndRules<'a> {
        pub choice_state: &'a mut ChoiceState,
        pub rules: &'a Rules,
    }

    implement!(<'a> BorrowMut<Card> for ChoiceStateAndRules<'a>: s, s.choice_state.card);

    impl<'a> Reset for ChoiceStateAndRules<'a> {
        fn reset(&mut self) {
            self.choice_state.reset();
        }
    }

    impl<'a> CardSubChoice for ChoiceStateAndRules<'a> {
        fn should_show_done_button(&self) -> bool {
            true //TODO check if there has been any change to the changes
        }
        fn mark_done(&mut self) {
            self.choice_state.layer = Layer::Done;
        }
        fn next_layer(&mut self) {
            self.choice_state.layer = Layer::Changes;
        }
        fn get_status_lines(&self, card: Card) -> StatusLines {
            let len = self.rules.when_played.0[card as usize].len();
            [
                bytes_to_status_line(format!("{}", len).as_bytes()),
                bytes_to_status_line(if len == 1 { b"change. " } else { b"changes." }),
            ]
        }
    }

}

pub struct GameState {
    // start in-game state
    pub deck: Hand,
    pub discard: Hand,
    pub cpu_hands: [Hand; 3],
    pub hand: Hand,
    pub current_player: PlayerID,
    pub winners: Vec<PlayerID>,
    pub top_wild_declared_as: Option<Suit>,
    pub card_animations: Vec<CardAnimation>,
    // end in-game state
    pub hand_index: u8,
    pub choice: Choice,
    pub rules: Rules,
    pub status: Status,
    pub context: UIContext,
    pub rng: XorShiftRng,
    pub event_log: EventLog,
    pub log_height: u8,
    pub log_heading: LogHeading,
    pub logger: Logger,
}

macro_rules! dealt_hand {
    ($deck:expr, $spread:expr) => {{
        let mut hand = Hand::new($spread);

        hand.draw_from($deck);
        hand.draw_from($deck);
        hand.draw_from($deck);
        hand.draw_from($deck);
        hand.draw_from($deck);

        hand
    }};
}

impl GameState {
    pub fn new(seed: [u8; 16], logger: Logger) -> GameState {
        let event_log = EventLog::new();
        GameState::new_with_previous(seed, d!(), d!(), logger, event_log)
    }

    pub fn new_with_previous(
        seed: [u8; 16],
        status: Status,
        rules: Rules,
        logger: Logger,
        mut event_log: EventLog,
    ) -> GameState {
        log(logger, &format!("{:?}", seed));

        event_log.push_hr();
        //TODO keep track of round count and change to "started round N"
        //TODO scroll the event log to the start of the new round?
        event_log.push(b"started a new round.");

        let mut rng = XorShiftRng::from_seed(seed);

        let mut deck = Hand::new_shuffled_deck(&mut rng);

        let discard = Hand::new(Spread::stack(DISCARD_X, DISCARD_Y));

        let hand = dealt_hand!(
            &mut deck,
            Spread::LTR(TOP_AND_BOTTOM_HAND_EDGES, PLAYER_HAND_HEIGHT)
        );

        let cpu_hands = [
            dealt_hand!(
                &mut deck,
                Spread::TTB(LEFT_AND_RIGHT_HAND_EDGES, LEFT_CPU_HAND_X)
            ),
            dealt_hand!(
                &mut deck,
                Spread::LTR(TOP_AND_BOTTOM_HAND_EDGES, MIDDLE_CPU_HAND_HEIGHT,)
            ),
            dealt_hand!(
                &mut deck,
                Spread::TTB(LEFT_AND_RIGHT_HAND_EDGES, RIGHT_CPU_HAND_X)
            ),
        ];

        let current_player = cpu_hands.len() as u8; //rng.gen_range(0, cpu_hands.len() as u8 + 1);

        invariant_assert!(current_player <= cpu_hands.len() as u8);

        let card_animations = Vec::with_capacity(DECK_SIZE as _);

        //We expect this to be replaced with a new vector when ever it it changed.
        let winners = Vec::with_capacity(0);

        GameState {
            deck,
            discard,
            cpu_hands,
            hand,
            hand_index: 0,
            current_player,
            card_animations,
            winners,
            top_wild_declared_as: None,
            choice: Choice::OfInGameChanges(in_game::ChoiceState {
                //for testing
                layer: in_game::Layer::Changes,
                ..d!()
            }), // Choice::NoChoice,
            rules,
            status,
            context: UIContext::new(),
            rng,
            event_log,
            log_height: 0,
            log_heading: LogHeading::Up,
            logger,
        }
    }

    pub fn player_id(&self) -> PlayerID {
        self.cpu_hands.len() as PlayerID
    }

    pub fn missing_cards(&self) -> Vec<Card> {
        use std::collections::BTreeSet;

        let example_deck: BTreeSet<Card> = fresh_deck().into_iter().collect();

        let mut observed_deck = BTreeSet::new();

        let card_iter = self
            .deck
            .iter()
            .chain(self.discard.iter())
            .chain(self.cpu_hands.iter().flat_map(|h| h.iter()))
            .chain(self.hand.iter())
            .chain(self.card_animations.iter().map(|a| &a.card.card));

        for c in card_iter {
            observed_deck.insert(c.clone());
        }

        example_deck.difference(&observed_deck).cloned().collect()
    }

    pub fn log(&self, s: &str) {
        log(self.logger, s);
    }

    pub fn get_logger(&self) -> Logger {
        self.logger.clone()
    }

    pub fn is_wild(&self, card: Card) -> bool {
        self.rules.wild.has_card(card)
    }
}
