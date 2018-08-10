use common::Logger;
use inner_common::*;

extern crate rand;

use self::rand::{Rng, SeedableRng, XorShiftRng};

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

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
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
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    MoveToDiscard,
    MoveToHand(PlayerID),
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct PositionedCard {
    pub card: Card,
    pub x: u8,
    pub y: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CardAnimation {
    pub card: PositionedCard,
    pub x: u8,
    pub y: u8,
    pub completion_action: Action,
}

impl CardAnimation {
    pub fn is_complete(&self) -> bool {
        self.card.x == self.x && self.card.y == self.y
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

        let result_x = match (both.0, furthest_only.0) {
            (1, 0) | (0, 1) => 1,
            (-1, 0) | (0, -1) => -1,
            _ => (both.0 + furthest_only.0) / 2,
        };
        let result_y = match (both.1, furthest_only.1) {
            (1, 0) | (0, 1) => 1,
            (-1, 0) | (0, -1) => -1,
            _ => (both.1 + furthest_only.1) / 2,
        };

        (result_x, result_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::*;

    #[test]
    fn test_approach_target_does_not_get_stuck() {
        quickcheck(approach_target_does_not_get_stuck as fn(CardAnimation) -> TestResult)
    }
    fn approach_target_does_not_get_stuck(animation: CardAnimation) -> TestResult {
        if animation.is_complete() {
            return TestResult::discard();
        }

        let mut after = animation.clone();

        after.approach_target();

        TestResult::from_bool(after != animation)
    }

    impl Arbitrary for CardAnimation {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            CardAnimation {
                card: PositionedCard {
                    card: g.gen_range(0, DECK_SIZE),
                    x: g.gen(),
                    y: g.gen(),
                },
                x: g.gen(),
                y: g.gen(),
                completion_action: Action::arbitrary(g),
            }
        }

        fn shrink(&self) -> Box<Iterator<Item = CardAnimation>> {
            let animation = self.clone();
            let card = animation.card.card;

            let tuple = (
                animation.card.x,
                animation.card.y,
                animation.x,
                animation.y,
                animation.completion_action,
            );

            Box::new(
                tuple
                    .shrink()
                    .map(
                        move |(card_x, card_y, x, y, completion_action)| CardAnimation {
                            card: PositionedCard {
                                card,
                                x: card_x,
                                y: card_y,
                            },
                            x,
                            y,
                            completion_action,
                        },
                    ),
            )
        }
    }

    impl Arbitrary for Action {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            if g.gen() {
                Action::MoveToHand(g.gen_range(0, 10))
            } else {
                Action::MoveToDiscard
            }
        }

        fn shrink(&self) -> Box<Iterator<Item = Action>> {
            match *self {
                Action::MoveToDiscard => empty_shrinker(),
                Action::MoveToHand(n) => {
                    let chain = single_shrinker(Action::MoveToDiscard)
                        .chain(n.shrink().map(Action::MoveToHand));
                    Box::new(chain)
                }
            }
        }
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
        let mut rng = rand::XorShiftRng::from_seed(seed);

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
}
