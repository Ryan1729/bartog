#![allow(dead_code)]

use crate::xs::{self, Xs};

//in pixels
pub const SCREEN_WIDTH: u8 = screen::WIDTH;
pub const SCREEN_HEIGHT: u8 = screen::HEIGHT;
pub const SCREEN_LENGTH: usize = screen::LENGTH;

#[macro_export]
macro_rules! red {
    ($colour:expr) => {
        ($colour & R_FLAG) >> R_SHIFT
    };
}

#[macro_export]
macro_rules! green {
    ($colour:expr) => {
        ($colour & G_FLAG) >> G_SHIFT
    };
}

#[macro_export]
macro_rules! blue {
    ($colour:expr) => {
        ($colour & B_FLAG) >> B_SHIFT
    };
}

#[macro_export]
macro_rules! alpha {
    ($colour:expr) => {
        ($colour & 0xFF_00_00_00) >> 24
    };
}

#[macro_export]
macro_rules! colour {
    ($red:expr, $green:expr, $blue:expr, $alpha:expr) => {
        ($red << R_SHIFT) | ($green << G_SHIFT) | ($blue << B_SHIFT) | $alpha << 24
    };
}

#[macro_export]
macro_rules! set_alpha {
    ($colour:expr, $alpha:expr) => {
        ($colour & 0x00_FF_FF_FF) | $alpha << 24
    };
}

pub const BLUE_INDEX: u8 = 0;
pub const GREEN_INDEX: u8 = 1;
pub const RED_INDEX: u8 = 2;
pub const YELLOW_INDEX: u8 = 3;
pub const PURPLE_INDEX: u8 = 4;
pub const GREY_INDEX: u8 = 5;
pub const WHITE_INDEX: u8 = 6;
pub const BLACK_INDEX: u8 = 7;

pub mod card {
    use super::*;

    pub const X_EMPTY_SPACE: u8 = 2;
    pub const Y_EMPTY_SPACE: u8 = 1;

    pub const HAND_OFFSET: u8 = 8;

    pub const WIDTH: u8 = 20;
    pub const HEIGHT: u8 = 30;

    pub const WIDTH_PLUS_SPACE: u8 = WIDTH + X_EMPTY_SPACE + X_EMPTY_SPACE;
    pub const HEIGHT_PLUS_SPACE: u8 = HEIGHT + Y_EMPTY_SPACE + Y_EMPTY_SPACE;

    pub const FRONT_SPRITE_X: u8 = 2;
    pub const FRONT_SPRITE_Y: u8 = 1;

    pub const BACK_SPRITE_X: u8 = 26;
    pub const BACK_SPRITE_Y: u8 = 1;

    pub const LEFT_RANK_X: u8 = 3;
    pub const LEFT_RANK_Y: u8 = 3;

    pub const LEFT_SUIT_X: u8 = 1;
    pub const LEFT_SUIT_Y: u8 = 10;

    pub const RIGHT_RANK_X: u8 = WIDTH - (LEFT_RANK_X + FONT_SIZE);
    pub const RIGHT_RANK_Y: u8 = HEIGHT - (LEFT_RANK_Y + FONT_SIZE);

    pub const RIGHT_SUIT_X: u8 = WIDTH - (LEFT_SUIT_X + FONT_SIZE);
    pub const RIGHT_SUIT_Y: u8 = HEIGHT - (LEFT_SUIT_Y + FONT_SIZE);
}

pub mod cursor {
    pub const X_EMPTY_SPACE: u8 = 1;
    pub const Y_EMPTY_SPACE: u8 = 0;

    pub const WIDTH: u8 = 24;
    pub const HEIGHT: u8 = 32;

    pub const WIDTH_PLUS_SPACE: u8 = WIDTH + X_EMPTY_SPACE + X_EMPTY_SPACE;
    pub const HEIGHT_PLUS_SPACE: u8 = HEIGHT + Y_EMPTY_SPACE + Y_EMPTY_SPACE;

    pub const SPRITE_X: u8 = 49;
    pub const SPRITE_Y: u8 = 0;

    pub const ALT_SPRITE_X: u8 = 73;
    pub const ALT_SPRITE_Y: u8 = 0;
}

pub mod checkbox {
    pub const UNCHECKED: u8 = 15;
    pub const CHECKED: u8 = UNCHECKED + 16;
    pub const HOT_UNCHECKED: u8 = UNCHECKED + 16 * 2;
    pub const HOT_CHECKED: u8 = UNCHECKED + 16 * 3;
    pub const PRESSED_CHECKED: u8 = UNCHECKED + 16 * 4;
    pub const PRESSED_UNCHECKED: u8 = UNCHECKED + 16 * 5;
}

pub const RANK_SUIT_PAIR_LAYOUT_CHAR: u8 = 26;

pub const TEN_CHAR: u8 = 27;

pub const CLUB_CHAR: u8 = 31;
pub const DIAMOND_CHAR: u8 = 29;
pub const HEART_CHAR: u8 = 30;
pub const SPADE_CHAR: u8 = 28;

pub const RANK_COUNT: u8 = 13;
pub const SUIT_COUNT: u8 = 4;
pub const DECK_SIZE: u8 = RANK_COUNT * SUIT_COUNT;

pub type Card = u8;

pub fn gen_cards(rng: &mut Xs, count: usize) -> Vec<Card> {
    let mut cards = Vec::with_capacity(count);
    for _ in 0..count {
        cards.push(gen_card(rng));
    }
    cards
}

pub fn gen_card(rng: &mut Xs) -> Card {
    xs::range(rng, 0..DECK_SIZE as _) as Card
}

pub fn get_card_string(card: Card) -> String {
    format!("{} of {}", get_rank_str(card), get_suit_str(get_suit(card)))
}

pub const RANK_SUIT_PAIR_WITH_IN_CHARS: u8 = 4;

pub fn get_suit_rank_pair(card: Card) -> String {
    let mut output = String::with_capacity(RANK_SUIT_PAIR_WITH_IN_CHARS as usize);

    let (colour, suit_char) = get_suit_colour_and_char(get_suit(card));

    output.push(RANK_SUIT_PAIR_LAYOUT_CHAR as char);
    output.push(get_rank_char(card) as char);
    output.push(suit_char as char);
    output.push(colour as char);

    output
}

pub fn get_short_card_string_and_colour(card: Card) -> (String, u8) {
    let (colour, ch) = get_suit_colour_and_char(get_suit(card));

    let mut output = String::with_capacity(2);

    output.push(get_rank_char(card) as char);
    output.push(ch as char);

    (output, colour)
}

pub struct ModOffset<T> {
    pub modulus: T,
    pub current: T,
    pub offset: u8,
}

use std::ops::{Add, Rem, Sub};

use features::invariant_violation;

#[inline]
pub fn next_mod<T>(
    ModOffset {
        modulus,
        current,
        offset,
    }: ModOffset<T>,
) -> T
where
    T: From<u8> + Add<T, Output = T> + Rem<T, Output = T> + PartialEq<T>,
{
    if modulus == 0u8.into() {
        invariant_violation!({ 0u8.into() }, "`modulus == 0` in `next_mod`")
    } else {
        (current + offset.into()) % modulus
    }
}

#[inline]
pub fn previous_mod<T>(
    ModOffset {
        modulus,
        current,
        offset,
    }: ModOffset<T>,
) -> T
where
    T: From<u8>
        + Add<T, Output = T>
        + Sub<T, Output = T>
        + Rem<T, Output = T>
        + PartialEq<T>
        + PartialOrd
        + Copy,
{
    if modulus == 0u8.into() || T::from(offset) > modulus {
        invariant_violation!(
            { 0u8.into() },
            "`modulus == 0 || offset > modulus` in `previous_mod`"
        )
    } else {
        (current + (modulus - offset.into())) % modulus
    }
}

impl<T> Default for ModOffset<T>
where
    T: From<u8>,
{
    fn default() -> Self {
        ModOffset {
            modulus: 1u8.into(),
            current: 0u8.into(),
            offset: 1,
        }
    }
}

#[inline]
pub fn nth_next_card(current: Card, offset: u8) -> Card {
    next_mod(ModOffset {
        modulus: DECK_SIZE,
        current,
        offset,
    })
}

#[inline]
pub fn nth_previous_card(current: Card, offset: u8) -> Card {
    previous_mod(ModOffset {
        modulus: DECK_SIZE,
        current,
        offset,
    })
}

pub type Suit = u8;

pub mod suits {
    use super::*;

    pub const CLUBS: Suit = 0;
    pub const DIAMONDS: Suit = 1;
    pub const HEARTS: Suit = 2;
    pub const SPADES: Suit = 3;

    pub const ALL: [Suit; SUIT_COUNT as usize] = [CLUBS, DIAMONDS, HEARTS, SPADES];
}

pub fn get_suit(card: Card) -> Suit {
    card / RANK_COUNT
}

pub fn get_suit_colour_and_char(suit: Suit) -> (u8, u8) {
    match suit {
        suits::CLUBS => (BLACK_INDEX, CLUB_CHAR),
        suits::DIAMONDS => (RED_INDEX, DIAMOND_CHAR),
        suits::HEARTS => (RED_INDEX, HEART_CHAR),
        suits::SPADES => (BLACK_INDEX, SPADE_CHAR),
        _ => (PURPLE_INDEX, 33), //purple "!"
    }
}

pub fn get_suit_str(suit: Suit) -> &'static str {
    match suit {
        suits::CLUBS => "clubs",
        suits::DIAMONDS => "diamonds",
        suits::HEARTS => "hearts",
        suits::SPADES => "spades",
        _ => "unknown",
    }
}

pub type Rank = u8;

pub mod ranks {
    use super::*;

    pub const ACE: Rank = 0;
    pub const TWO: Rank = 1;
    pub const THREE: Rank = 2;
    pub const FOUR: Rank = 3;
    pub const FIVE: Rank = 4;
    pub const SIX: Rank = 5;
    pub const SEVEN: Rank = 6;
    pub const EIGHT: Rank = 7;
    pub const NINE: Rank = 8;
    pub const TEN: Rank = 9;
    pub const JACK: Rank = 10;
    pub const QUEEN: Rank = 11;
    pub const KING: Rank = 12;

    pub const ALL: [Rank; RANK_COUNT as usize] = [
        ACE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, TEN, JACK, QUEEN, KING,
    ];
}

pub fn get_rank(card: Card) -> Rank {
    card % RANK_COUNT
}

pub fn get_rank_char(card: Card) -> u8 {
    get_rank_char_from_rank(get_rank(card))
}

pub fn get_rank_char_from_rank(rank: Rank) -> u8 {
    match rank {
        0 => b'a',
        1 => b'2',
        2 => b'3',
        3 => b'4',
        4 => b'5',
        5 => b'6',
        6 => b'7',
        7 => b'8',
        8 => b'9',
        9 => TEN_CHAR,
        10 => b'j',
        11 => b'q',
        12 => b'k',
        _ => b'!',
    }
}

pub fn get_rank_str(card: Card) -> &'static str {
    match get_rank(card) {
        0 => "ace",
        1 => "2",
        2 => "3",
        3 => "4",
        4 => "5",
        5 => "6",
        6 => "7",
        7 => "8",
        8 => "9",
        9 => "10",
        10 => "jack",
        11 => "queen",
        12 => "king",
        _ => "unknown rank",
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct PositionedCard {
    pub card: Card,
    pub x: u8,
    pub y: u8,
}

pub type PlayerID = u8;

pub const MAX_PLAYER_ID: PlayerID = 3;
pub const PLAYER_ID_COUNT: usize = (MAX_PLAYER_ID + 1) as _;
pub const PLAYER_ID: PlayerID = MAX_PLAYER_ID;

pub fn all_player_ids() -> [PlayerID; PLAYER_ID_COUNT] {
    let mut output = [0; PLAYER_ID_COUNT];
    for i in 0..=MAX_PLAYER_ID {
        output[i as usize] = i;
    }
    output
}

#[inline]
pub fn is_cpu_player(player_id: PlayerID) -> bool {
    player_id < PLAYER_ID
}

// Having invalid ids treated as player ids curruntly works fine most places, and it's nice if
// `is_cpu_player` and `is_player` cover all the cases. we can make `is_strictly_player` or
// something if we need to.
#[inline]
pub fn is_player(player_id: PlayerID) -> bool {
    player_id >= PLAYER_ID
}

pub fn player_name(player_id: PlayerID) -> String {
    if is_cpu_player(player_id) {
        format!("cpu {}", player_id)
    } else if player_id == MAX_PLAYER_ID {
        "you".to_owned()
    } else {
        "???".to_owned()
    }
}

pub fn player_1_char_name(player_id: PlayerID) -> String {
    if is_cpu_player(player_id) {
        format!("{}", player_id)
    } else if player_id == MAX_PLAYER_ID {
        "u".to_owned()
    } else {
        "?".to_owned()
    }
}

pub fn get_pronoun(player_id: PlayerID) -> String {
    if player_id == MAX_PLAYER_ID {
        "you".to_string()
    } else {
        "they".to_string()
    }
}

pub const PLAYER_HAND_HEIGHT: u8 = (SCREEN_HEIGHT as u16 - (card::HEIGHT * 5 / 9) as u16) as u8;
pub const MIDDLE_CPU_HAND_HEIGHT: u8 = card::Y_EMPTY_SPACE;
pub const LEFT_CPU_HAND_X: u8 = card::X_EMPTY_SPACE;
pub const RIGHT_CPU_HAND_X: u8 =
    (SCREEN_WIDTH as u16 - (card::WIDTH as u16 + card::X_EMPTY_SPACE as u16)) as u8;

pub const LEFT_AND_RIGHT_HAND_EDGES: (u8, u8) = (
    card::HEIGHT + (card::Y_EMPTY_SPACE * 2),
    SCREEN_HEIGHT - (card::HEIGHT + card::Y_EMPTY_SPACE),
);

pub const TOP_AND_BOTTOM_HAND_EDGES: (u8, u8) = (
    card::X_EMPTY_SPACE,
    SCREEN_WIDTH - card::X_EMPTY_SPACE,
);

pub const DECK_X: u8 = 40;
pub const DECK_Y: u8 = 32;
pub const DECK_XY: (u8, u8) = (DECK_X, DECK_Y);

pub const DISCARD_X: u8 = DECK_X + card::WIDTH + card::WIDTH / 2;
pub const DISCARD_Y: u8 = DECK_Y;
pub const DISCARD_XY: (u8, u8) = (DISCARD_X, DISCARD_Y);

pub const NINE_SLICE_MAX_INTERIOR_SIZE: u8 = (SCREEN_WIDTH - 2 * SPRITE_SIZE) as u8;

pub const NINE_SLICE_MAX_INTERIOR_WIDTH_IN_CHARS: u8 =
    (NINE_SLICE_MAX_INTERIOR_SIZE / FONT_ADVANCE) as u8;
pub const NINE_SLICE_MAX_INTERIOR_HEIGHT_IN_CHARS: u8 =
    (NINE_SLICE_MAX_INTERIOR_SIZE / FONT_SIZE) as u8;

pub const WINDOW_TOP_LEFT: u8 = 64;
pub const BUTTON_TOP_LEFT: u8 = 67;
pub const BUTTON_HOT_TOP_LEFT: u8 = 70;
pub const BUTTON_PRESSED_TOP_LEFT: u8 = 73;

pub const ROW_LEFT_EDGE: u8 = 12;
pub const ROW_HOT_LEFT_EDGE: u8 = 28;
pub const ROW_PRESSED_LEFT_EDGE: u8 = 44;
pub const ROW_MARKER_LEFT_EDGE: u8 = 60;

pub const SPRITE_SIZE: u8 = 8;
pub const SPRITES_PER_ROW: u8 = (platform_types::GFX_WIDTH / SPRITE_SIZE as usize) as u8;

pub const FONT_SIZE: u8 = 8;
pub const FONT_ADVANCE: u8 = 4;
pub const FONT_FLIP: u8 = 128;