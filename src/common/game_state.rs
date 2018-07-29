use common::Logger;
use inner_common::*;

extern crate rand;

use self::rand::{Rng, SeedableRng, XorShiftRng};

pub struct Hand(Vec<Card>);

impl Hand {
    pub fn new() -> Self {
        Hand(Vec::with_capacity(DECK_SIZE as usize))
    }

    pub fn new_shuffled_deck<R: Rng>(rng: &mut R) -> Self {
        let mut deck = Vec::with_capacity(DECK_SIZE as usize);

        for i in 0..DECK_SIZE {
            deck.push(i);
        }

        rng.shuffle(&mut deck);

        Hand(deck)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Card> {
        self.0.iter()
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.0.pop()
    }

    pub fn draw_from(&mut self, other: &mut Hand) {
        if let Some(card) = other.0.pop() {
            self.0.push(card);
        }
    }

    pub fn discard_to(&mut self, other: &mut Hand, index: usize) {
        if index < self.0.len() {
            other.0.push(self.0.remove(index));
        }
    }

    pub fn discard_randomly_to<R: Rng>(&mut self, other: &mut Hand, rng: &mut R) {
        let len = self.0.len();
        if len > 0 {
            let index = rng.gen_range(0, len);
            other.0.push(self.0.remove(index));
        }
    }
}

#[derive(Copy, Clone)]
pub enum Action {
    MoveToDiscard,
    MoveToHand(PlayerID),
}

#[derive(Default)]
pub struct PositionedCard {
    card: Card,
    x: u8,
    y: u8,
}

pub struct CardAnimation {
    pub card: PositionedCard,
    pub x: u8,
    pub y: u8,
    pub completion_action: Action,
}

impl CardAnimation {
    pub fn is_complete(&self) -> bool {
        self.card.x == self.x && self.card.x == self.y
    }

    pub fn approach_target(&mut self) {
        let (d_x, d_y) = self.get_delta();

        self.card.x = (self.card.x as i8).saturating_add(d_x) as _;
        self.card.y = (self.card.y as i8).saturating_add(d_y) as _;
    }

    fn get_delta(&self) -> (i8, i8) {
        // TODO is this faster, slower, or equivalent to Bresenham?
        // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
        let both = (
            if self.x == self.card.x {
                0
            } else if self.x > self.card.x {
                1
            } else {
                -1
            },
            if self.y == self.card.y {
                0
            } else if self.y > self.card.y {
                1
            } else {
                -1
            },
        );

        let x_diff = if self.card.x > self.x {
            self.card.x - self.x
        } else {
            self.x - self.card.x
        };
        let y_diff = if self.card.y > self.y {
            self.card.y - self.y
        } else {
            self.y - self.card.y
        };

        let furthest_only = if x_diff == y_diff {
            return both;
        } else if x_diff > y_diff {
            (both.0, 0)
        } else {
            (0, both.0)
        };

        (
            (both.0 + furthest_only.0) / 2,
            (both.0 + furthest_only.0) / 2,
        )
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
    pub rng: XorShiftRng,
    logger: Logger,
}

macro_rules! dealt_hand {
    ($deck:expr) => {{
        let mut hand = Hand::new();

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
        let mut rng = rand::XorShiftRng::from_seed(seed);

        let mut deck = Hand::new_shuffled_deck(&mut rng);

        let discard = Hand::new();

        let hand = dealt_hand!(&mut deck);
        let cpu_hands = [
            dealt_hand!(&mut deck),
            dealt_hand!(&mut deck),
            dealt_hand!(&mut deck),
        ];

        let current_player = rng.gen_range(0, cpu_hands.len() as u8 + 1);

        let card_animations = Vec::with_capacity(DECK_SIZE as _);

        GameState {
            deck,
            discard,
            cpu_hands,
            hand,
            hand_index: 0,
            current_player,
            card_animations,
            rng,
            logger,
        }
    }
}
