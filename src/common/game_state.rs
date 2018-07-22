use common::Logger;
use inner_common::*;

extern crate rand;

use self::rand::{Rng, SeedableRng, XorShiftRng};

pub struct Hand(Vec<Card>);

/// There should be no operations that can cause cards not to be conserved between all Hand
/// instances.
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

pub struct GameState {
    pub deck: Hand,
    pub hand: Hand,
    pub rng: XorShiftRng,
    logger: Logger,
}

impl GameState {
    pub fn new(seed: [u8; 16], logger: Logger) -> GameState {
        let mut rng = rand::XorShiftRng::from_seed(seed);

        let mut deck = Hand::new_shuffled_deck(&mut rng);

        let mut hand = Hand::new();

        hand.draw_from(&mut deck);
        hand.draw_from(&mut deck);
        hand.draw_from(&mut deck);
        hand.draw_from(&mut deck);
        hand.draw_from(&mut deck);

        GameState {
            deck,
            hand,
            rng,
            logger,
        }
    }
}
