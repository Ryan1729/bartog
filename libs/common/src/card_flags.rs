use english::*;
use inner_common::*;

use std::fmt;

use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CardFlags(u64);

const ONE_PAST_CARD_FLAGS_MAX: u64 = 1 << DECK_SIZE as u64;

// TODO make `Standard` generate mostly easy to describe subsets of the cards
//and add another distribution if needed
impl Distribution<CardFlags> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardFlags {
        CardFlags(rng.gen_range(0, ONE_PAST_CARD_FLAGS_MAX))
    }
}

pub const CLUBS_FLAGS: u64 = 0b0001_1111_1111_1111;
pub const DIAMONDS_FLAGS: u64 = CLUBS_FLAGS << RANK_COUNT;
pub const HEARTS_FLAGS: u64 = CLUBS_FLAGS << (RANK_COUNT * 2);
pub const SPADES_FLAGS: u64 = CLUBS_FLAGS << (RANK_COUNT * 3);

pub const SUIT_FLAGS: [u64; SUIT_COUNT as usize] =
    [CLUBS_FLAGS, DIAMONDS_FLAGS, HEARTS_FLAGS, SPADES_FLAGS];

macro_rules! rank_pattern {
    (0) => {
        0b0000000000001_0000000000001_0000000000001_0000000000001
    };
    (1) => {
        0b0000000000010_0000000000010_0000000000010_0000000000010
    };
    (2) => {
        0b0000000000100_0000000000100_0000000000100_0000000000100
    };
    (3) => {
        0b0000000001000_0000000001000_0000000001000_0000000001000
    };
    (4) => {
        0b0000000010000_0000000010000_0000000010000_0000000010000
    };
    (5) => {
        0b0000000100000_0000000100000_0000000100000_0000000100000
    };
    (6) => {
        0b0000001000000_0000001000000_0000001000000_0000001000000
    };
    (7) => {
        0b0000010000000_0000010000000_0000010000000_0000010000000
    };
    (8) => {
        0b0000100000000_0000100000000_0000100000000_0000100000000
    };
    (9) => {
        0b0001000000000_0001000000000_0001000000000_0001000000000
    };
    (10) => {
        0b0010000000000_0010000000000_0010000000000_0010000000000
    };
    (11) => {
        0b0100000000000_0100000000000_0100000000000_0100000000000
    };
    (12) => {
        0b1000000000000_1000000000000_1000000000000_1000000000000
    };
}

pub const RANK_FLAGS: [u64; RANK_COUNT as usize] = [
    rank_pattern!(0),
    rank_pattern!(1),
    rank_pattern!(2),
    rank_pattern!(3),
    rank_pattern!(4),
    rank_pattern!(5),
    rank_pattern!(6),
    rank_pattern!(7),
    rank_pattern!(8),
    rank_pattern!(9),
    rank_pattern!(10),
    rank_pattern!(11),
    rank_pattern!(12),
];

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

impl fmt::Display for CardFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let original_flags = self.0;
        let mut flags = self.0;

        let mut subsets = Vec::new();

        macro_rules! push_and_remove {
            ($subset:expr) => {{
                if original_flags & $subset == $subset {
                    subsets.push($subset);

                    #[allow(unused_assignments)]
                    {
                        flags &= !$subset;
                    };
                }
            }};
        }

        if flags == 0 {
            subsets.push(0);
        } else {
            push_and_remove!(CLUBS_FLAGS);
            push_and_remove!(DIAMONDS_FLAGS);
            push_and_remove!(HEARTS_FLAGS);
            push_and_remove!(SPADES_FLAGS);

            push_and_remove!(rank_pattern!(0));
            push_and_remove!(rank_pattern!(1));
            push_and_remove!(rank_pattern!(2));
            push_and_remove!(rank_pattern!(3));
            push_and_remove!(rank_pattern!(4));
            push_and_remove!(rank_pattern!(5));
            push_and_remove!(rank_pattern!(6));
            push_and_remove!(rank_pattern!(7));
            push_and_remove!(rank_pattern!(8));
            push_and_remove!(rank_pattern!(9));
            push_and_remove!(rank_pattern!(10));
            push_and_remove!(rank_pattern!(11));
            push_and_remove!(rank_pattern!(12));

            // TODO proper subsets of these like "The red twos" and "the even spades".
            // Also proper supersets of these like "the red cards" and "the even cards".
            // This requires different control flow, probably `loop` and `break;`.

            // TODO solve a toy version of the problem say one with only 8 bits, to try
            // and get a handle on this.
        }
        println!(
            "{:?}",
            subsets
                .iter()
                .map(|s| format!("{:052b}", s))
                .collect::<Vec<_>>()
        );
        write!(f, "{}", map_sentence_list(&subsets, write_card_set_str))
    }
}

const CARD_FLAGS_DISPLAY_FALLBACK: &'static str = "the selected cards";

use std::borrow::Cow;
fn write_card_set_str<'f, 's>(flags: &'f u64) -> Cow<'s, str> {
    macro_rules! rank_result {
        ($index:expr) => {{
            format!("the {}s", get_rank_str($index))
        }};
    }
    match *flags {
        0 => "{}".into(),
        //Suits
        CLUBS_FLAGS => "the clubs".into(),
        DIAMONDS_FLAGS => "the diamonds".into(),
        HEARTS_FLAGS => "the hearts".into(),
        SPADES_FLAGS => "the spades".into(),
        //Ranks
        rank_pattern!(0) => rank_result!(0).into(),
        rank_pattern!(1) => rank_result!(1).into(),
        rank_pattern!(2) => rank_result!(2).into(),
        rank_pattern!(3) => rank_result!(3).into(),
        rank_pattern!(4) => rank_result!(4).into(),
        rank_pattern!(5) => rank_result!(5).into(),
        rank_pattern!(6) => rank_result!(6).into(),
        rank_pattern!(7) => rank_result!(7).into(),
        rank_pattern!(8) => rank_result!(8).into(),
        rank_pattern!(9) => rank_result!(9).into(),
        rank_pattern!(10) => rank_result!(10).into(),
        rank_pattern!(11) => rank_result!(11).into(),
        rank_pattern!(12) => rank_result!(12).into(),
        //fs if flags.bit_count() == 1 => w!("{}", card_flag_to_card(fs)).into(),
        _ => CARD_FLAGS_DISPLAY_FALLBACK.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::*;

    #[test]
    fn test_no_card_flags_resort_to_the_fallback() {
        quickcheck(no_card_flags_resort_to_the_fallback as fn(CardFlags) -> TestResult)
    }
    fn no_card_flags_resort_to_the_fallback(flags: CardFlags) -> TestResult {
        let string = flags.to_string();

        test_println!("{:#?} => {} <=", flags, string);

        if string.contains(CARD_FLAGS_DISPLAY_FALLBACK) || string == "" {
            TestResult::failed()
        } else {
            TestResult::passed()
        }
    }

    #[test]
    fn test_suits_combined_with_rank_does_not_use_the_fallback() {
        let flags = CardFlags::new(rank_pattern!(0) | CLUBS_FLAGS);

        assert!(!no_card_flags_resort_to_the_fallback(flags).is_failure());
    }

    impl Arbitrary for CardFlags {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            CardFlags(g.gen_range(0, ONE_PAST_CARD_FLAGS_MAX))
        }

        fn shrink(&self) -> Box<Iterator<Item = Self>> {
            match self.0 {
                0 => empty_shrinker(),
                x => {
                    macro_rules! check {
                        ($to_remove:expr) => {
                            let new_x = x & !($to_remove);
                            if new_x != x {
                                return single_shrinker(CardFlags::new(new_x));
                            }
                        };
                    }

                    check!(CLUBS_FLAGS);
                    check!(DIAMONDS_FLAGS);
                    check!(HEARTS_FLAGS);
                    check!(SPADES_FLAGS);
                    check!(rank_pattern!(0));
                    check!(rank_pattern!(1));
                    check!(rank_pattern!(2));
                    check!(rank_pattern!(3));
                    check!(rank_pattern!(4));
                    check!(rank_pattern!(5));
                    check!(rank_pattern!(6));
                    check!(rank_pattern!(7));
                    check!(rank_pattern!(8));
                    check!(rank_pattern!(9));
                    check!(rank_pattern!(10));
                    check!(rank_pattern!(11));
                    check!(rank_pattern!(12));

                    for i in 0..DECK_SIZE {
                        check!(1 << i);
                    }

                    empty_shrinker()
                }
            }
        }
    }

    macro_rules! across_all_suits {
        ($flags:expr) => {
            ($flags & 0b0001_1111_1111_1111)
                | (($flags & 0b0001_1111_1111_1111) << RANK_COUNT)
                | (($flags & 0b0001_1111_1111_1111) << (RANK_COUNT * 2))
                | (($flags & 0b0001_1111_1111_1111) << (RANK_COUNT * 3))
        };
    }

    pub const OLD_RANK_FLAGS: [u64; RANK_COUNT as usize] = [
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

    #[test]
    fn RANK_FLAGS_matches_OLD_RANK_FLAGS() {
        for i in 0..RANK_COUNT as usize {
            assert_eq!(RANK_FLAGS[i], OLD_RANK_FLAGS[i]);
        }
    }
}

impl fmt::Debug for CardFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = self.0;
        if f.alternate() {
            return if v == 0 {
                write!(f, "{:52b}", v)
            } else {
                write!(f, "{:052b}", v)
            };
        }
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
