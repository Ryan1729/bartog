use inner_common::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    MoveToDiscard,
    MoveToHand(PlayerID),
    SelectWild(PlayerID),
}

use std::cmp::{max, min};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CardAnimation {
    pub card: PositionedCard,
    pub x: u8,
    pub y: u8,
    pub x_rate: u8,
    pub y_rate: u8,
    pub completion_action: Action,
}

const DELAY_FACTOR: u8 = 16;

impl CardAnimation {
    pub fn new(card: PositionedCard, x: u8, y: u8, completion_action: Action) -> Self {
        let (x_diff, y_diff) = (
            if x == card.x {
                0
            } else if card.x > x {
                card.x - x
            } else {
                x - card.x
            },
            if y == card.y {
                0
            } else if card.y > y {
                card.y - y
            } else {
                y - card.y
            },
        );

        CardAnimation {
            card,
            x,
            y,
            x_rate: max(x_diff / DELAY_FACTOR, 1),
            y_rate: max(y_diff / DELAY_FACTOR, 1),
            completion_action,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.card.x == self.x && self.card.y == self.y
    }

    pub fn approach_target(&mut self) {
        let (d_x, d_y) = self.get_delta();

        self.card.x = match d_x {
            x if x > 0 => self.card.x.saturating_add(x as u8),
            x if x < 0 => self.card.x.saturating_sub(x.abs() as u8),
            _ => self.card.x,
        };
        self.card.y = match d_y {
            y if y > 0 => self.card.y.saturating_add(y as u8),
            y if y < 0 => self.card.y.saturating_sub(y.abs() as u8),
            _ => self.card.y,
        };
    }

    fn get_delta(&self) -> (i8, i8) {
        (
            if self.x == self.card.x {
                0
            } else if self.card.x > self.x {
                let x_diff = self.card.x - self.x;
                -(min(x_diff, self.x_rate) as i8)
            } else {
                let x_diff = self.x - self.card.x;
                min(x_diff, self.x_rate) as i8
            },
            if self.y == self.card.y {
                0
            } else if self.card.y > self.y {
                let y_diff = self.card.y - self.y;
                -(min(y_diff, self.y_rate) as i8)
            } else {
                let y_diff = self.y - self.card.y;
                min(y_diff, self.y_rate) as i8
            },
        )
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

    #[test]
    fn test_approach_target_reaches_target() {
        quickcheck(approach_target_reaches_target as fn(CardAnimation) -> TestResult)
    }
    fn approach_target_reaches_target(animation: CardAnimation) -> TestResult {
        if animation.is_complete() {
            return TestResult::discard();
        }

        let mut temp = animation.clone();

        for _ in 0..SCREEN_LENGTH + 1 {
            temp.approach_target();

            if temp.is_complete() {
                return TestResult::from_bool(true);
            }
        }

        TestResult::from_bool(false)
    }

    impl Arbitrary for CardAnimation {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            CardAnimation::new(
                PositionedCard {
                    card: g.gen_range(0, DECK_SIZE),
                    x: g.gen(),
                    y: g.gen(),
                },
                g.gen(),
                g.gen(),
                Action::arbitrary(g),
            )
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
                            x_rate: animation.x_rate,
                            y_rate: animation.y_rate,
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
                Action::SelectWild(n) => {
                    let chain = single_shrinker(Action::MoveToDiscard)
                        .chain(n.shrink().map(Action::SelectWild));
                    Box::new(chain)
                }
            }
        }
    }
}
