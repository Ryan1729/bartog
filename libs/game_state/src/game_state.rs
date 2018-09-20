use common::{
    bytes_lines, bytes_reflow, slice_until_first_0, CardAnimation, UIContext, DECK_SIZE, *,
};
use std::cmp::max;
use std::collections::VecDeque;
use std::fmt;

use platform_types::{log, Logger};

use rand::{Rng, SeedableRng, XorShiftRng};

pub mod can_play {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Edges(u64);

    impl Edges {
        pub fn new(edges: u64) -> Self {
            Edges(edges & ((1 << 52) - 1))
        }

        pub fn has_card(&self, card: Card) -> bool {
            self.0 & (1 << card) != 0
        }

        pub fn toggle_card(&mut self, card: Card) {
            let was = self.has_card(card);
            self.set_card_to(card, !was)
        }

        pub fn set_card_to(&mut self, card: Card, to: bool) {
            if to {
                self.set_card(card);
            } else {
                self.unset_card(card);
            }
        }

        pub fn set_card(&mut self, card: Card) {
            self.0 |= 1 << card;
        }
        pub fn unset_card(&mut self, card: Card) {
            self.0 &= !(1 << card);
        }

        pub fn cards(&self) -> Vec<Card> {
            let mut output = Vec::with_capacity(DECK_SIZE as _);

            for card in 0..DECK_SIZE {
                if self.has_card(card) {
                    output.push(card);
                }
            }

            output
        }

        pub fn get_bits(&self) -> u64 {
            self.0
        }
    }

    use std::ops::BitOr;

    impl BitOr<Edges> for Edges {
        type Output = Edges;
        fn bitor(self, other: Edges) -> Self::Output {
            Edges(self.0 | other.0)
        }
    }

    impl BitOr<u64> for Edges {
        type Output = Edges;
        fn bitor(self, other: u64) -> Self::Output {
            Edges(self.0 | other)
        }
    }

    impl BitOr<Edges> for u64 {
        type Output = Edges;
        fn bitor(self, other: Edges) -> Self::Output {
            Edges(self | other.0)
        }
    }

    impl Default for Edges {
        fn default() -> Self {
            Edges(0)
        }
    }

    impl fmt::Debug for Edges {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let v = self.0;
            if v >= 1 << 52 {
                write!(f, "INVALID EDGES: {:?}, valid portion:", v);
            }

            write!(
                f,
                "{:?}",
                self.cards()
                    .into_iter()
                    .map(get_card_string)
                    .collect::<Vec<_>>(),
            )
        }
    }

    #[derive(Clone)]
    pub struct Graph {
        pub nodes: [Edges; DECK_SIZE as usize],
    }

    impl Graph {
        pub fn is_playable_on(&self, card: Card, top_of_discard: Card) -> bool {
            let edges = self.nodes[card as usize].0;
            edges & (1 << top_of_discard as u64) != 0
        }

        pub fn get_edges(&self, card: Card) -> Edges {
            self.nodes[card as usize]
        }

        pub fn set_edges(&mut self, card: Card, edges: Edges) {
            self.nodes[card as usize] = edges;
        }
    }

    const CLUBS_FLAGS: u64 = 0b0001_1111_1111_1111;
    const DIAMONDS_FLAGS: u64 = CLUBS_FLAGS << RANK_COUNT;
    const HEARTS_FLAGS: u64 = CLUBS_FLAGS << (RANK_COUNT * 2);
    const SPADES_FLAGS: u64 = CLUBS_FLAGS << (RANK_COUNT * 3);

    const SUIT_FLAGS: [u64; SUIT_COUNT as usize] =
        [CLUBS_FLAGS, DIAMONDS_FLAGS, HEARTS_FLAGS, SPADES_FLAGS];

    macro_rules! across_all_suits {
        ($flags:expr) => {
            ($flags & 0b0001_1111_1111_1111)
                | (($flags & 0b0001_1111_1111_1111) << RANK_COUNT)
                | (($flags & 0b0001_1111_1111_1111) << (RANK_COUNT * 2))
                | (($flags & 0b0001_1111_1111_1111) << (RANK_COUNT * 3))
        };
    }

    const RANK_FLAGS: [u64; RANK_COUNT as usize] = [
        across_all_suits!(1),
        across_all_suits!(1 << 1),
        across_all_suits!(1 << 2),
        across_all_suits!(1 << 3),
        across_all_suits!(1 << 4),
        across_all_suits!(1 << 5),
        across_all_suits!(1 << 6),
        across_all_suits!(1 << 7),
        across_all_suits!(1 << 8),
        across_all_suits!(1 << 9),
        across_all_suits!(1 << 10),
        across_all_suits!(1 << 11),
        across_all_suits!(1 << 12),
    ];

    impl Default for Graph {
        fn default() -> Self {
            //Reminder:
            // the cards go from 0-51, in ascending rank order,
            // and in ♣ ♦ ♥ ♠ suit order (alphabetical)
            // A♣, 2♣, ... K♣, A♦, ..., A♥, ..., A♠, ..., K♠.
            let mut nodes = [Edges::default(); DECK_SIZE as usize];

            for suit in 0..SUIT_COUNT as usize {
                for rank in 0..RANK_COUNT as usize {
                    let i = rank + suit * RANK_COUNT as usize;

                    nodes[i] = Edges::new(SUIT_FLAGS[suit] | RANK_FLAGS[rank]);
                }
            }

            Graph { nodes }
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Change(u64);

    impl Change {
        pub fn new(edges: Edges, card: Card) -> Self {
            Change(((card as u64) << DECK_SIZE) | edges.0)
        }

        pub fn edges(&self) -> Edges {
            Edges::new(self.0)
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
    }

    impl Default for Layer {
        fn default() -> Self {
            Layer::Card
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct ChoiceState {
        pub changes: Vec<can_play::Change>,
        pub card: Card,
        pub edges: Edges,
        pub layer: Layer,
        pub scroll_card: Card,
        pub done: bool,
    }
}

#[derive(Clone, Copy)]
pub enum Spread {
    LTR((u8, u8), u8),
    TTB((u8, u8), u8),
}

impl Spread {
    fn stack(x: u8, y: u8) -> Self {
        Spread::LTR((x, x.saturating_add(card::WIDTH)), y)
    }
}

use std::cmp::min;

pub fn get_card_offset(spread: Spread, len: u8) -> u8 {
    if len == 0 {
        return 0;
    }

    let ((min_edge, max_edge), span) = match spread {
        Spread::LTR(edges, _) => (edges, card::WIDTH),
        Spread::TTB(edges, _) => (edges, card::HEIGHT),
    };

    let full_width = max_edge.saturating_sub(min_edge);
    let usable_width = full_width.saturating_sub(span);

    min(usable_width / len, span)
}

pub fn get_card_position(spread: Spread, len: u8, index: u8) -> (u8, u8) {
    let offset = get_card_offset(spread, len);

    match spread {
        Spread::LTR((min_edge, _), y) => (min_edge.saturating_add(offset.saturating_mul(index)), y),
        Spread::TTB((min_edge, _), x) => (x, min_edge.saturating_add(offset.saturating_mul(index))),
    }
}

pub struct Hand {
    cards: Vec<Card>,
    pub spread: Spread,
}

fn fresh_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(DECK_SIZE as usize);

    for i in 0..DECK_SIZE {
        deck.push(i);
    }

    deck
}

impl Hand {
    pub fn new(spread: Spread) -> Self {
        Hand {
            cards: Vec::with_capacity(DECK_SIZE as usize),
            spread,
        }
    }

    pub fn new_shuffled_deck<R: Rng>(rng: &mut R) -> Self {
        let mut deck = fresh_deck();

        rng.shuffle(&mut deck);

        Hand {
            cards: deck,
            spread: Spread::stack(DECK_X, DECK_Y),
        }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn get(&self, index: u8) -> Option<&Card> {
        self.cards.get(index as usize)
    }

    pub fn last(&self) -> Option<&Card> {
        self.cards.last()
    }

    pub fn draw_from(&mut self, other: &mut Hand) {
        if let Some(card) = other.cards.pop() {
            self.cards.push(card);
        }
    }

    pub fn discard_to(&mut self, other: &mut Hand, index: usize) {
        if index < self.cards.len() {
            other.cards.push(self.cards.remove(index));
        }
    }

    pub fn discard_randomly_to<R: Rng>(&mut self, other: &mut Hand, rng: &mut R) {
        let len = self.cards.len();
        if len > 0 {
            let index = rng.gen_range(0, len);
            other.cards.push(self.cards.remove(index));
        }
    }

    pub fn shuffle<R: Rng>(&mut self, rng: &mut R) {
        rng.shuffle(&mut self.cards);
    }

    pub fn len(&self) -> u8 {
        let len = self.cards.len();

        if len >= 255 {
            255
        } else {
            len as u8
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Card> {
        self.cards.iter()
    }

    pub fn drain<'a>(&'a mut self) -> impl Iterator<Item = Card> + 'a {
        self.cards.drain(..)
    }

    pub fn fill(&mut self, cards: impl Iterator<Item = Card>) {
        self.cards.extend(cards);
    }

    pub fn remove_if_present(&mut self, index: u8) -> Option<PositionedCard> {
        let len = self.len();
        let cards = &mut self.cards;

        if index < len {
            let (x, y) = get_card_position(self.spread, len, index);
            let card = cards.remove(index as usize);

            Some(PositionedCard { card, x, y })
        } else {
            None
        }
    }

    pub fn most_common_suits(&self) -> [Option<Suit>; 4] {
        let mut counts: [(u8, u8); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
        for suit in self.cards.iter().cloned().map(get_suit) {
            let (suit, count) = counts[suit as usize];
            counts[suit as usize] = (suit, count + 1);
        }

        counts.sort_by(
            |(_, c1), (_, c2)| c2.cmp(c1), //descending order
        );

        let mut max_count = 0;
        let mut result = [None; 4];
        for (i, &(suit, count)) in counts.into_iter().enumerate() {
            if count > max_count {
                result[i] = Some(suit);
            } else {
                break;
            }

            max_count = max(count, max_count);
        }

        result
    }

    pub fn most_common_suit(&self) -> Option<Suit> {
        let suits = self.most_common_suits();
        suits[0]
    }
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
        (0..=self.cpu_hands.len()).map(|id| id as u8).collect()
    }

    pub fn player_name(&self, playerId: PlayerID) -> String {
        let len = self.cpu_hands.len() as PlayerID;

        if playerId < len {
            format!("cpu {}", playerId)
        } else if playerId == len {
            "you".to_string()
        } else {
            "???".to_string()
        }
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
            .map(|&player| self.player_name(player))
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

pub struct EventLog {
    pub buffer: VecDeque<EventLine>,
}

type EventLine = [u8; EventLog::WIDTH];

impl EventLog {
    const WIDTH: usize = NINE_SLICE_MAX_INTERIOR_WIDTH_IN_CHARS as usize;
    const HEIGHT: usize = NINE_SLICE_MAX_INTERIOR_HEIGHT_IN_CHARS as usize;

    const BUFFER_SIZE: usize = 1024;

    pub fn new() -> Self {
        let buffer = VecDeque::with_capacity(EventLog::BUFFER_SIZE);
        EventLog { buffer }
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn push(&mut self, bytes: &[u8]) {
        let reflowed = bytes_reflow(bytes, EventLog::WIDTH);
        let lines = bytes_lines(&reflowed);

        for line in lines {
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

    pub fn get_window_slice<'a>(&'a self, top_index: usize) -> impl Iterator<Item = &'a [u8]> {
        self.iter().skip(top_index).take(EventLog::HEIGHT)
    }
}

#[derive(Clone, Debug)]
pub enum Choice {
    NoChoice,
    Already(Chosen),
    OfCanPlayGraph(can_play::ChoiceState),
    OfSuit,
    OfBool,
    OfUnit,
}

impl Choice {
    pub fn is_idle(&self) -> bool {
        match *self {
            Choice::NoChoice | Choice::Already(_) => true,
            Choice::OfCanPlayGraph(_) | Choice::OfSuit | Choice::OfBool | Choice::OfUnit => false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Chosen {
    CanPlayGraph(Vec<can_play::Change>),
    Suit(Suit),
    Bool(bool),
    Unit(()),
}

pub enum LogHeading {
    Up,
    Down,
}

pub enum Status {
    InGame,
    RuleSelection,
}

impl Default for Status {
    fn default() -> Self {
        //Status::InGame
        //For testing
        Status::RuleSelection
    }
}

#[derive(Default)]
pub struct Rules {
    pub can_play_graph: can_play::Graph,
}

impl Rules {
    fn empty() -> Self {
        Rules {
            can_play_graph: can_play::Graph {
                nodes: [can_play::Edges::new(0); DECK_SIZE as usize],
            },
        }
    }
}

pub struct GameState {
    pub deck: Hand,
    pub discard: Hand,
    pub cpu_hands: [Hand; 3],
    pub hand: Hand,
    pub hand_index: u8,
    pub current_player: PlayerID,
    pub card_animations: Vec<CardAnimation>,
    pub winners: Vec<PlayerID>,
    pub top_wild_declared_as: Option<Suit>,
    pub choice: Choice,
    pub rules: Rules,
    pub status: Status,
    pub context: UIContext,
    pub rng: XorShiftRng,
    pub event_log: EventLog,
    pub log_top_index: usize,
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
        GameState::new_with_previous(
            seed,
            Default::default(),
            Default::default(),
            logger,
            event_log,
        )
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
            choice: Choice::NoChoice,
            rules,
            status,
            context: UIContext::new(),
            rng,
            event_log,
            log_top_index: 0,
            log_height: 0,
            log_heading: LogHeading::Up,
            logger,
        }
    }

    fn add_cpu_rule(&mut self, player: PlayerID) {
        self.add_rule_change_log_header(player);

        //TODO add single-strongly connected component checking and start
        //generating non-additive changes;
        let count = self.rng.gen_range(5, DECK_SIZE as usize);
        let mut cards = Vec::with_capacity(count);
        for _ in 0..count {
            cards.push(self.rng.gen_range(0, DECK_SIZE));
        }

        let mut changes = Vec::with_capacity(count);
        for card in cards {
            let old_edges = self.rules.can_play_graph.get_edges(card);
            let edges = self.rng.gen_range(0, 1 << DECK_SIZE as u64) | old_edges;

            changes.push(can_play::Change::new(edges, card));
        }

        self.apply_can_play_graph_changes(changes, player);
    }

    pub fn apply_can_play_graph_changes(
        &mut self,
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
                let previous_edges = self.rules.can_play_graph.get_edges(new_card);

                let mut additions = Vec::new();
                let mut removals = Vec::new();

                for card in 0..DECK_SIZE {
                    let mask = 1 << card as usize;
                    let p_edge = mask & previous_edges.get_bits() != 0;
                    let n_edge = mask & new_edges.get_bits() != 0;

                    match (p_edge, n_edge) {
                        (true, false) => removals.push(card),
                        (false, true) => additions.push(card),
                        _ => {}
                    }
                }

                let pronoun = self.get_pronoun(player);
                let card_string = get_card_string(new_card);

                match (additions.len() > 0, removals.len() > 0) {
                    (false, false) => {}
                    (true, false) => {
                        let additions_string = get_card_list(&additions);
                        let text = &[
                            pronoun.as_bytes(),
                            b" allowed the ",
                            card_string.as_bytes(),
                            b" to be played on the following cards: ",
                            additions_string.as_bytes(),
                            b".",
                        ]
                            .concat();
                        self.event_log.push(text);
                    }
                    (false, true) => {
                        let removals_string = get_card_list(&removals);
                        let text = &[
                            pronoun.as_bytes(),
                            b" prevented the ",
                            card_string.as_bytes(),
                            b" from being played on the following cards: ",
                            removals_string.as_bytes(),
                            b".",
                        ]
                            .concat();
                        self.event_log.push(text);
                    }
                    (true, true) => {
                        let additions_string = get_card_list(&additions);
                        let removals_string = get_card_list(&removals);
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
                        self.event_log.push(text);
                    }
                };

                /////////

                self.rules.can_play_graph.set_edges(new_card, new_edges);
            }
        }
    }

    pub fn add_rule_change_log_header(&mut self, player: PlayerID) {
        self.event_log.push_hr();

        let player_name = self.player_name(player);

        let text = &[player_name.as_bytes(), b" changed the rules as follows:"].concat();

        self.event_log.push(text);
    }

    pub fn player_id(&self) -> PlayerID {
        self.cpu_hands.len() as PlayerID
    }

    pub fn reset(&mut self) {
        use std::mem::replace;

        let status = {
            let mut status = Status::InGame;

            let player_id = self.player_id();

            for &id in self.winners.clone().iter() {
                if id >= player_id {
                    status = Status::RuleSelection;
                    continue;
                }

                self.add_cpu_rule(id);
            }

            status
        };

        let old_log = replace(
            &mut self.event_log,
            EventLog {
                buffer: VecDeque::with_capacity(0),
            },
        );
        let old_rules = replace(&mut self.rules, Rules::empty());

        *self =
            GameState::new_with_previous(self.rng.gen(), status, old_rules, self.logger, old_log);
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
}
