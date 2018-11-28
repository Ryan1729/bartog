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

macro_rules! all_suits_consts {
    {
        $vis:vis const $clubs:ident: $type:ty = $clubs_expr:expr;
        $diamonds:ident;
        $hearts:ident;
        $spades:ident;
    } => {
        $vis const $clubs: $type = $clubs_expr;
        $vis const $diamonds: $type = $clubs_expr << RANK_COUNT;
        $vis const $hearts: $type = $clubs_expr << (RANK_COUNT * 2);
        $vis const $spades: $type = $clubs_expr << (RANK_COUNT * 3);
    }
}

all_suits_consts! {
    pub const CLUBS_FLAGS: u64 = 0b0001_1111_1111_1111;
    DIAMONDS_FLAGS;
    HEARTS_FLAGS;
    SPADES_FLAGS;
}

pub const BLACK_FLAGS: u64 = CLUBS_FLAGS | SPADES_FLAGS;
pub const RED_FLAGS: u64 = DIAMONDS_FLAGS | HEARTS_FLAGS;

all_suits_consts! {
    pub const CLUBS_FACE_FLAGS: u64 = 0b0001_1100_0000_0000;
    DIAMONDS_FACE_FLAGS;
    HEARTS_FACE_FLAGS;
    SPADES_FACE_FLAGS;
}

all_suits_consts! {
    pub const CLUBS_NUMBER_FLAGS: u64 = 0b0000_0011_1111_1111;
    DIAMONDS_NUMBER_FLAGS;
    HEARTS_NUMBER_FLAGS;
    SPADES_NUMBER_FLAGS;
}

all_suits_consts! {
    pub const CLUBS_EVEN_PLUS_Q: u64 = 0b0000_1010_1010_1010;
    DIAMONDS_EVEN_PLUS_Q;
    HEARTS_EVEN_PLUS_Q;
    SPADES_EVEN_PLUS_Q;
}

all_suits_consts! {
    pub const CLUBS_EVEN_SANS_Q: u64 = 0b0000_0010_1010_1010;
    DIAMONDS_EVEN_SANS_Q;
    HEARTS_EVEN_SANS_Q;
    SPADES_EVEN_SANS_Q;
}

all_suits_consts! {
    pub const CLUBS_ODD_PLUS_K_AND_J: u64 = 0b0001_0101_0101_0101;
    DIAMONDS_ODD_PLUS_K_AND_J;
    HEARTS_ODD_PLUS_K_AND_J;
    SPADES_ODD_PLUS_K_AND_J;
}

all_suits_consts! {
    pub const CLUBS_ODD_SANS_K_AND_J: u64 = 0b0000_0001_0101_0101;
    DIAMONDS_ODD_SANS_K_AND_J;
    HEARTS_ODD_SANS_K_AND_J;
    SPADES_ODD_SANS_K_AND_J;
}

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
    (0 red) => {
        0b0000000000000_0000000000001_0000000000001_0000000000000
    };
    (1 red) => {
        0b0000000000000_0000000000010_0000000000010_0000000000000
    };
    (2 red) => {
        0b0000000000000_0000000000100_0000000000100_0000000000000
    };
    (3 red) => {
        0b0000000000000_0000000001000_0000000001000_0000000000000
    };
    (4 red) => {
        0b0000000000000_0000000010000_0000000010000_0000000000000
    };
    (5 red) => {
        0b0000000000000_0000000100000_0000000100000_0000000000000
    };
    (6 red) => {
        0b0000000000000_0000001000000_0000001000000_0000000000000
    };
    (7 red) => {
        0b0000000000000_0000010000000_0000010000000_0000000000000
    };
    (8 red) => {
        0b0000000000000_0000100000000_0000100000000_0000000000000
    };
    (9 red) => {
        0b0000000000000_0001000000000_0001000000000_0000000000000
    };
    (10 red) => {
        0b0000000000000_0010000000000_0010000000000_0000000000000
    };
    (11 red) => {
        0b0000000000000_0100000000000_0100000000000_0000000000000
    };
    (12 red) => {
        0b0000000000000_1000000000000_1000000000000_0000000000000
    };
    (0 black) => {
        0b0000000000001_0000000000000_0000000000000_0000000000001
    };
    (1 black) => {
        0b0000000000010_0000000000000_0000000000000_0000000000010
    };
    (2 black) => {
        0b0000000000100_0000000000000_0000000000000_0000000000100
    };
    (3 black) => {
        0b0000000001000_0000000000000_0000000000000_0000000001000
    };
    (4 black) => {
        0b0000000010000_0000000000000_0000000000000_0000000010000
    };
    (5 black) => {
        0b0000000100000_0000000000000_0000000000000_0000000100000
    };
    (6 black) => {
        0b0000001000000_0000000000000_0000000000000_0000001000000
    };
    (7 black) => {
        0b0000010000000_0000000000000_0000000000000_0000010000000
    };
    (8 black) => {
        0b0000100000000_0000000000000_0000000000000_0000100000000
    };
    (9 black) => {
        0b0001000000000_0000000000000_0000000000000_0001000000000
    };
    (10 black) => {
        0b0010000000000_0000000000000_0000000000000_0010000000000
    };
    (11 black) => {
        0b0100000000000_0000000000000_0000000000000_0100000000000
    };
    (12 black) => {
        0b1000000000000_0000000000000_0000000000000_1000000000000
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

pub const BLACK_RANK_FLAGS: [u64; RANK_COUNT as usize] = [
    rank_pattern!(0 black),
    rank_pattern!(1 black),
    rank_pattern!(2 black),
    rank_pattern!(3 black),
    rank_pattern!(4 black),
    rank_pattern!(5 black),
    rank_pattern!(6 black),
    rank_pattern!(7 black),
    rank_pattern!(8 black),
    rank_pattern!(9 black),
    rank_pattern!(10 black),
    rank_pattern!(11 black),
    rank_pattern!(12 black),
];

pub const RED_RANK_FLAGS: [u64; RANK_COUNT as usize] = [
    rank_pattern!(0 red),
    rank_pattern!(1 red),
    rank_pattern!(2 red),
    rank_pattern!(3 red),
    rank_pattern!(4 red),
    rank_pattern!(5 red),
    rank_pattern!(6 red),
    rank_pattern!(7 red),
    rank_pattern!(8 red),
    rank_pattern!(9 red),
    rank_pattern!(10 red),
    rank_pattern!(11 red),
    rank_pattern!(12 red),
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

const SPECIAL_FLAGS: [u64; 69] = [
    BLACK_FLAGS,
    RED_FLAGS,
    CLUBS_FLAGS,
    DIAMONDS_FLAGS,
    HEARTS_FLAGS,
    SPADES_FLAGS,
    CLUBS_FACE_FLAGS,
    DIAMONDS_FACE_FLAGS,
    HEARTS_FACE_FLAGS,
    SPADES_FACE_FLAGS,
    CLUBS_NUMBER_FLAGS,
    DIAMONDS_NUMBER_FLAGS,
    HEARTS_NUMBER_FLAGS,
    SPADES_NUMBER_FLAGS,
    CLUBS_EVEN_PLUS_Q,
    DIAMONDS_EVEN_PLUS_Q,
    HEARTS_EVEN_PLUS_Q,
    SPADES_EVEN_PLUS_Q,
    CLUBS_EVEN_SANS_Q,
    DIAMONDS_EVEN_PLUS_Q,
    HEARTS_EVEN_PLUS_Q,
    SPADES_EVEN_PLUS_Q,
    CLUBS_ODD_PLUS_K_AND_J,
    DIAMONDS_ODD_PLUS_K_AND_J,
    HEARTS_ODD_PLUS_K_AND_J,
    SPADES_ODD_PLUS_K_AND_J,
    CLUBS_ODD_SANS_K_AND_J,
    DIAMONDS_ODD_PLUS_K_AND_J,
    HEARTS_ODD_PLUS_K_AND_J,
    SPADES_ODD_PLUS_K_AND_J,
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
    BLACK_RANK_FLAGS[0],
    BLACK_RANK_FLAGS[1],
    BLACK_RANK_FLAGS[2],
    BLACK_RANK_FLAGS[3],
    BLACK_RANK_FLAGS[4],
    BLACK_RANK_FLAGS[5],
    BLACK_RANK_FLAGS[6],
    BLACK_RANK_FLAGS[7],
    BLACK_RANK_FLAGS[8],
    BLACK_RANK_FLAGS[9],
    BLACK_RANK_FLAGS[10],
    BLACK_RANK_FLAGS[11],
    BLACK_RANK_FLAGS[12],
    BLACK_RANK_FLAGS[0],
    RED_RANK_FLAGS[1],
    RED_RANK_FLAGS[2],
    RED_RANK_FLAGS[3],
    RED_RANK_FLAGS[4],
    RED_RANK_FLAGS[5],
    RED_RANK_FLAGS[6],
    RED_RANK_FLAGS[7],
    RED_RANK_FLAGS[8],
    RED_RANK_FLAGS[9],
    RED_RANK_FLAGS[10],
    RED_RANK_FLAGS[11],
    RED_RANK_FLAGS[12],
];

impl fmt::Display for CardFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let original_flags = self.0;
        let mut flags = self.0;

        let mut subsets = Vec::new();

        let mut tracking_flags = 0;
        if flags == 0 {
            subsets.push(0);
        } else {
            for &f in SPECIAL_FLAGS.iter() {
                if tracking_flags & f != f {
                    if original_flags & f == f {
                        tracking_flags |= f;
                        subsets.push(f);

                        flags &= !f;
                        if flags == 0 {
                            break;
                        }
                    }
                }
            }
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
        ($index:expr, black) => {{
            format!("the black {}s", get_rank_str($index))
        }};
        ($index:expr, red) => {{
            format!("the red {}s", get_rank_str($index))
        }};
    }
    match *flags {
        0 => "{}".into(),
        //Colours
        BLACK_FLAGS => "the black cards".into(),
        RED_FLAGS => "the red cards".into(),
        //Suits
        CLUBS_FLAGS => "the clubs".into(),
        DIAMONDS_FLAGS => "the diamonds".into(),
        HEARTS_FLAGS => "the hearts".into(),
        SPADES_FLAGS => "the spades".into(),

        CLUBS_FACE_FLAGS => "the club faces".into(),
        DIAMONDS_FACE_FLAGS => "the diamond faces".into(),
        HEARTS_FACE_FLAGS => "the heart faces".into(),
        SPADES_FACE_FLAGS => "the spade faces".into(),

        CLUBS_NUMBER_FLAGS => "the club numbers".into(),
        DIAMONDS_NUMBER_FLAGS => "the diamond numbers".into(),
        HEARTS_NUMBER_FLAGS => "the heart numbers".into(),
        SPADES_NUMBER_FLAGS => "the spade numbers".into(),

        CLUBS_EVEN_SANS_Q => "the even clubs (sans queen)".into(),
        DIAMONDS_EVEN_SANS_Q => "the even diamonds (sans queen)".into(),
        HEARTS_EVEN_SANS_Q => "the even hearts (sans queen)".into(),
        SPADES_EVEN_SANS_Q => "the even spades (sans queen)".into(),

        CLUBS_EVEN_PLUS_Q => "the even clubs (including queen)".into(),
        DIAMONDS_EVEN_PLUS_Q => "the even diamonds (including queen)".into(),
        HEARTS_EVEN_PLUS_Q => "the even hearts (including queen)".into(),
        SPADES_EVEN_PLUS_Q => "the even spades (including queen)".into(),

        CLUBS_ODD_SANS_K_AND_J => "the odd clubs (sans the king and jack)".into(),
        DIAMONDS_ODD_SANS_K_AND_J => "the odd diamonds (sans the king and jack)".into(),
        HEARTS_ODD_SANS_K_AND_J => "the odd hearts (sans the king and jack)".into(),
        SPADES_ODD_SANS_K_AND_J => "the odd spades (sans the king and jack)".into(),

        CLUBS_ODD_PLUS_K_AND_J => "the odd clubs (including the king and jack)".into(),
        DIAMONDS_ODD_PLUS_K_AND_J => "the odd diamonds (including the king and jack)".into(),
        HEARTS_ODD_PLUS_K_AND_J => "the odd hearts (including the king and jack)".into(),
        SPADES_ODD_PLUS_K_AND_J => "the odd spades (including the king and jack)".into(),

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

        rank_pattern!(0 black) => rank_result!(0, black).into(),
        rank_pattern!(1 black) => rank_result!(1, black).into(),
        rank_pattern!(2 black) => rank_result!(2, black).into(),
        rank_pattern!(3 black) => rank_result!(3, black).into(),
        rank_pattern!(4 black) => rank_result!(4, black).into(),
        rank_pattern!(5 black) => rank_result!(5, black).into(),
        rank_pattern!(6 black) => rank_result!(6, black).into(),
        rank_pattern!(7 black) => rank_result!(7, black).into(),
        rank_pattern!(8 black) => rank_result!(8, black).into(),
        rank_pattern!(9 black) => rank_result!(9, black).into(),
        rank_pattern!(10 black) => rank_result!(10, black).into(),
        rank_pattern!(11 black) => rank_result!(11, black).into(),
        rank_pattern!(12 black) => rank_result!(12, black).into(),

        rank_pattern!(0 red) => rank_result!(0, red).into(),
        rank_pattern!(1 red) => rank_result!(1, red).into(),
        rank_pattern!(2 red) => rank_result!(2, red).into(),
        rank_pattern!(3 red) => rank_result!(3, red).into(),
        rank_pattern!(4 red) => rank_result!(4, red).into(),
        rank_pattern!(5 red) => rank_result!(5, red).into(),
        rank_pattern!(6 red) => rank_result!(6, red).into(),
        rank_pattern!(7 red) => rank_result!(7, red).into(),
        rank_pattern!(8 red) => rank_result!(8, red).into(),
        rank_pattern!(9 red) => rank_result!(9, red).into(),
        rank_pattern!(10 red) => rank_result!(10, red).into(),
        rank_pattern!(11 red) => rank_result!(11, red).into(),
        rank_pattern!(12 red) => rank_result!(12, red).into(),

        _ => CARD_FLAGS_DISPLAY_FALLBACK.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::*;

    #[test]
    fn test_no_card_flags_resort_to_the_fallback() {
        //TODO generate only flags not covered by `no_special_flag_uses_the_fallback`
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
                    let mut tracking_flags = 0;
                    macro_rules! check {
                        ($to_remove:expr) => {
                            if tracking_flags & $to_remove == $to_remove {
                                let new_x = x & !($to_remove);
                                if new_x != x {
                                    return single_shrinker(CardFlags::new(new_x));
                                }
                            }
                        };
                    }

                    check!(CLUBS_FLAGS);
                    check!(DIAMONDS_FLAGS);
                    check!(HEARTS_FLAGS);
                    check!(SPADES_FLAGS);

                    check!(CLUBS_FACE_FLAGS);
                    check!(DIAMONDS_FACE_FLAGS);
                    check!(HEARTS_FACE_FLAGS);
                    check!(SPADES_FACE_FLAGS);

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

    #[derive(Clone, Debug)]
    struct Special<T>(T);

    impl Arbitrary for Special<CardFlags> {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Special(CardFlags(
                SPECIAL_FLAGS[g.gen_range(0, SPECIAL_FLAGS.len())],
            ))
        }

        fn shrink(&self) -> Box<Iterator<Item = Self>> {
            Box::new(self.0.shrink().map(Special))
        }
    }

    #[test]
    fn test_no_special_flag_uses_the_fallback() {
        quickcheck(no_special_flag_uses_the_fallback as fn(Special<CardFlags>) -> TestResult)
    }
    fn no_special_flag_uses_the_fallback(Special(flags): Special<CardFlags>) -> TestResult {
        no_card_flags_resort_to_the_fallback(flags)
    }

    #[test]
    fn test_each_special_flag_produces_the_expected_outcome() {
        quickcheck(
            each_special_flag_produces_the_expected_outcome as fn(Special<CardFlags>) -> TestResult,
        )
    }
    fn each_special_flag_produces_the_expected_outcome(
        Special(flags): Special<CardFlags>,
    ) -> TestResult {
        let expected = write_card_set_str(&flags.0);
        let string = flags.to_string();

        let passes = expected == string;

        if !passes {
            test_println!("{:#?} => {} =/= {}", flags, string, expected);
        }

        TestResult::from_bool(passes)
    }

    #[cfg(feature = "false")]
    mod toy {
        use super::*;
        type ToyFlags = u8;

        pub const RANK_COUNT: ToyFlags = 2;

        pub const CLUBS_FLAGS: ToyFlags = 0b0000_0011;
        pub const DIAMONDS_FLAGS: ToyFlags = CLUBS_FLAGS << RANK_COUNT;
        pub const HEARTS_FLAGS: ToyFlags = CLUBS_FLAGS << (RANK_COUNT * 2);
        pub const SPADES_FLAGS: ToyFlags = CLUBS_FLAGS << (RANK_COUNT * 3);

        macro_rules! rank_pattern {
            (0) => {
                0b0101_0101
            };
            (1) => {
                0b1010_1010
            };
        }

        pub const RANK_FLAGS: [ToyFlags; RANK_COUNT as usize] = [0b0101_0101, 0b1010_1010];

        const FLAGS_DISPLAY_FALLBACK: &'static str = "the selected cards";

        fn flags_string<'s>(flags: ToyFlags) -> Cow<'s, str> {
            match flags {
                0 => "{}".into(),
                //Suits
                CLUBS_FLAGS => "the clubs".into(),
                DIAMONDS_FLAGS => "the diamonds".into(),
                HEARTS_FLAGS => "the hearts".into(),
                SPADES_FLAGS => "the spades".into(),
                //Ranks
                rank_pattern!(0) => "the aces".into(),
                rank_pattern!(1) => "the twos".into(),
                fs if flags.count_ones() == 1 => card_string(card_flag_to_card(fs)).into(),
                _ => FLAGS_DISPLAY_FALLBACK.into(),
            }

            // idea of how the working version should work:
            // go through te special subsets, largest to amallest.
            // When you find a subset that is represented in the flags
            // set those bits in another set of flags.call these the
            // tracking flags, If a subset is completely covered by the
            // set bit in the tracking flags then skip it.
        }

        type ToyCard = u8;

        fn card_flag_to_card(flags: ToyFlags) -> ToyCard {
            flags.trailing_zeros() as ToyCard
        }

        fn card_string(flags: ToyCard) -> &'static str {
            match flags {
                0 => "the 1 of clubs",
                1 => "the 2 of clubs",
                2 => "the 1 of diamonds",
                3 => "the 2 of diamonds",
                4 => "the 1 of hearts",
                5 => "the 2 of hearts",
                6 => "the 1 of spades",
                7 => "the 2 of spades",
                _ => "fn card_string(flags: ToyCard) -> &'static str",
            }
        }

        #[test]
        fn card_flag_to_card_does_what_i_want() {
            assert_eq!(card_flag_to_card(0b1), 0);
            assert_eq!(card_flag_to_card(0b10), 1);
            assert_eq!(card_flag_to_card(0b100), 2);
            assert_eq!(card_flag_to_card(0b1000), 3);
            assert_eq!(card_flag_to_card(0b10000), 4);
            assert_eq!(card_flag_to_card(0b100000), 5);
            assert_eq!(card_flag_to_card(0b1000000), 6);
            assert_eq!(card_flag_to_card(0b10000000), 7);
        }

        #[test]
        fn test_no_flags_resort_to_the_fallback() {
            quickcheck(no_flags_resort_to_the_fallback as fn(ToyFlags) -> TestResult)
        }
        fn no_flags_resort_to_the_fallback(flags: ToyFlags) -> TestResult {
            let string = flags_string(flags);

            test_println!("{:08b} => {} <=", flags, string);

            if string.contains(FLAGS_DISPLAY_FALLBACK) || string == "" {
                TestResult::failed()
            } else {
                TestResult::passed()
            }
        }

        #[test]
        fn test_suits_combined_with_rank_does_not_use_the_fallback() {
            let flags = rank_pattern!(0) | CLUBS_FLAGS;

            assert!(!no_flags_resort_to_the_fallback(flags).is_failure());
        }

        #[test]
        fn test_pairs_of_cards_with_the_same_colour_and_rank_produce_the_expected_results() {
            assert_eq!(
                flags_string(0b01_00_00_01),
                <Cow<str>>::from("the black aces")
            );
            assert_eq!(
                flags_string(0b10_00_00_10),
                <Cow<str>>::from("the black twos")
            );
            assert_eq!(
                flags_string(0b00_01_01_00),
                <Cow<str>>::from("the red aces")
            );
            assert_eq!(
                flags_string(0b00_10_10_00),
                <Cow<str>>::from("the red twos")
            );
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
