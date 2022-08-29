use crate::traits::AllValues;
use inner_common::{*, xs::Xs};

use std::cmp::{max, min};
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Spread {
    LTR((u8, u8), u8),
    TTB((u8, u8), u8),
}

impl Default for Spread {
    fn default() -> Self {
        Spread::LTR((0, 0), 0)
    }
}

impl Spread {
    pub fn stack(x: u8, y: u8) -> Self {
        Spread::LTR((x, x.saturating_add(card::WIDTH)), y)
    }
}

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

#[derive(Clone, Debug, Default)]
pub struct Hand {
    cards: Vec<Card>,
    pub spread: Spread,
}

pub fn fresh_deck() -> Vec<Card> {
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

    pub fn new_shuffled_deck(rng: &mut Xs) -> Self {
        let mut deck = fresh_deck();

        xs::shuffle(rng, &mut deck);

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

    pub fn discard_randomly_to(&mut self, other: &mut Hand, rng: &mut Xs) {
        let len = self.cards.len();
        if len > 0 {
            let index = xs::range(rng, 0..len as _) as usize;
            other.cards.push(self.cards.remove(index));
        }
    }

    pub fn shuffle(&mut self, rng: &mut Xs) {
        xs::shuffle(rng, &mut self.cards);
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
        for (i, (suit, count)) in counts.into_iter().enumerate() {
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

    pub fn remove_selected(&mut self, selection: CardSelection) -> Option<PositionedCard> {
        let len = self.cards.len();
        if len == 0 {
            return None;
        }
        match selection {
            CardSelection::NthModuloCount(n) => {
                let i = (n.get() - 1) % len as u8;

                self.remove_if_present(i)
            }
        }
    }

    pub fn inverse_remove_selected(&mut self, selection: CardSelection) -> Option<PositionedCard> {
        let len = self.cards.len();
        if len == 0 {
            return None;
        }

        match selection {
            CardSelection::NthModuloCount(n) => {
                let len = len as u8;
                let i = (len - 1) - ((n.get() - 1) % len);

                self.remove_if_present(i)
            }
        }
    }
}

use std::num::NonZeroU8;
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CardSelection {
    NthModuloCount(NonZeroU8),
    // NthIfPresent(u8),
    // ChosenBy(PlayerID),
}

impl fmt::Display for CardSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            match *self {
                CardSelection::NthModuloCount(n) => {
                    ordinal_display(n.get(), f)?;
                    write!(f, "(%)")?
                }
            }

            return Ok(());
        }
        match *self {
            CardSelection::NthModuloCount(n) => {
                ordinal_display(n.get(), f)?;
                write!(f, " card, looping if needed")
            }
        }
    }
}

impl AllValues for CardSelection {
    fn all_values() -> Vec<CardSelection> {
        (1..=DECK_SIZE)
            .into_iter()
            .map(|n| nu8!(n))
            .map(CardSelection::NthModuloCount)
            .collect()
    }
}

//implement!(
        //Distribution<CardSelection> for Standard
        //by picking from CardSelection::all_values()
    //);
