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

#[derive(Default)]
pub struct PositionedCard {
    card: Card,
    x: u8,
    y: u8,
}

pub enum CardAnimation {
    None,
    Draw(PositionedCard),
    Discard(PositionedCard),
}

impl Default for CardAnimation {
    fn default() -> Self {
        CardAnimation::None
    }
}

#[derive(Default)]
pub struct Turn {
    player: u8,
    animation: CardAnimation,
}

pub struct GameState {
    pub deck: Hand,
    pub discard: Hand,
    pub cpu_hands: [Hand; 3],
    pub hand: Hand,
    pub hand_index: u8,
    pub turn: Turn,
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

        let turn = rng.gen_range(0, cpu_hands.len() as u8 + 1);

        GameState {
            deck,
            discard,
            cpu_hands,
            hand,
            hand_index: 0,
            turn: Default::default(),
            rng,
            logger,
        }
    }
}
