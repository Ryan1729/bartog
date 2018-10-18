use common::*;

use std::fmt;

use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CardFlags(u64);

const ONE_PAST_CARD_FLAGS_MAX: u64 = 1 << DECK_SIZE as u64;

impl Distribution<CardFlags> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardFlags {
        CardFlags(rng.gen_range(0, ONE_PAST_CARD_FLAGS_MAX))
    }
}

impl CardFlags {
    pub fn new(edges: u64) -> Self {
        CardFlags(edges & (ONE_PAST_CARD_FLAGS_MAX - 1))
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

    pub fn from_cards(cards: Vec<Card>) -> Self {
        let mut output = CardFlags(0);

        for card in cards {
            output.set_card(card);
        }

        output
    }

    pub fn get_bits(&self) -> u64 {
        self.0
    }
}

use std::ops::BitOr;

impl BitOr<CardFlags> for CardFlags {
    type Output = CardFlags;
    fn bitor(self, other: CardFlags) -> Self::Output {
        CardFlags(self.0 | other.0)
    }
}

impl BitOr<u64> for CardFlags {
    type Output = CardFlags;
    fn bitor(self, other: u64) -> Self::Output {
        CardFlags(self.0 | other)
    }
}

impl BitOr<CardFlags> for u64 {
    type Output = CardFlags;
    fn bitor(self, other: CardFlags) -> Self::Output {
        CardFlags(self | other.0)
    }
}

impl Default for CardFlags {
    fn default() -> Self {
        CardFlags(0)
    }
}

impl fmt::Debug for CardFlags {
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

pub const CLUBS_FLAGS: u64 = 0b0001_1111_1111_1111;
pub const DIAMONDS_FLAGS: u64 = CLUBS_FLAGS << RANK_COUNT;
pub const HEARTS_FLAGS: u64 = CLUBS_FLAGS << (RANK_COUNT * 2);
pub const SPADES_FLAGS: u64 = CLUBS_FLAGS << (RANK_COUNT * 3);

pub const SUIT_FLAGS: [u64; SUIT_COUNT as usize] =
    [CLUBS_FLAGS, DIAMONDS_FLAGS, HEARTS_FLAGS, SPADES_FLAGS];

macro_rules! across_all_suits {
    ($flags:expr) => {
        ($flags & 0b0001_1111_1111_1111)
            | (($flags & 0b0001_1111_1111_1111) << RANK_COUNT)
            | (($flags & 0b0001_1111_1111_1111) << (RANK_COUNT * 2))
            | (($flags & 0b0001_1111_1111_1111) << (RANK_COUNT * 3))
    };
}

pub const RANK_FLAGS: [u64; RANK_COUNT as usize] = [
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
