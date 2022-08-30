use common::*;
use std::fmt;

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

        for (suit_i, suit) in SUIT_FLAGS.iter().enumerate() {
            for (rank_i, rank) in RANK_FLAGS.iter().enumerate() {
                let i = suit_i * RANK_COUNT as usize + rank_i;

                nodes[i] = CardFlags::new(suit | rank);
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    pub reset_edges: CardFlags,
    pub edges: CardFlags,
    pub edges_card: Card,
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
    fn reset(&mut self) {
        *self = d!();
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

implement!(BorrowPairMut<Card, CardFlags> for ChoiceState: s, (s.edges_card, s.edges));

impl CardFlagsSubChoice for ChoiceState {
    fn mark_done(&mut self) {
        self.layer = Layer::Card;
    }
    fn reset(&mut self) {
        self.edges = self.reset_edges;
    }
}
