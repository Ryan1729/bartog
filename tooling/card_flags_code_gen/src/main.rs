#![feature(range_contains)]

use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn get_rank_char(rank: usize) -> &'static str {
    match rank {
        0 => "A",
        1 => "2",
        2 => "3",
        3 => "4",
        4 => "5",
        5 => "6",
        6 => "7",
        7 => "8",
        8 => "9",
        9 => "10",
        10 => "J",
        11 => "Q",
        12 => "K",
        _ => "!",
    }
}

fn main() {
    let path = Path::new("./code_flags_macros.rs");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    const RANK_COUNT: u8 = 13;

    let mut macros = String::new();

    macro_rules! p {
        ($s:expr) => {
            macros.push_str($s)
        };
    }

    macro_rules! pf {
        ($($stuff:tt)*) => {p!(&format!($($stuff)*))};
    }

    let sorted_extrema = get_sorted_extrema(RANK_COUNT);

    p!("// generated file!\n\n");
    p!("macro_rules! consecutive_ranks {");

    for (start, end) in sorted_extrema.iter() {
        let range = start..=end;

        macro_rules! in_range {
            ($e:expr) => {{
                let x: u8 = $e;
                if range.contains(&&x) {
                    1
                } else {
                    0
                }
            }};
        }

        macro_rules! thirteen_times {
            ($fmt_str:expr) => {
                pf!(
                    $fmt_str,
                    start,
                    end,
                    in_range!(12),
                    in_range!(11),
                    in_range!(10),
                    in_range!(9),
                    in_range!(8),
                    in_range!(7),
                    in_range!(6),
                    in_range!(5),
                    in_range!(4),
                    in_range!(3),
                    in_range!(2),
                    in_range!(1),
                    in_range!(0),
                );
            };
        }

        thirteen_times!("({}-{} black) => {{ 0b{2}_{3}{4}{5}{6}_{7}{8}{9}{10}_{11}{12}{13}{14}__0_0000_0000_0000__0_0000_0000_0000__{2}_{3}{4}{5}{6}_{7}{8}{9}{10}_{11}{12}{13}{14} }};");
        thirteen_times!("({}-{} red) => {{ 0b{2}_{3}{4}{5}{6}_{7}{8}{9}{10}_{11}{12}{13}{14}__{2}_{3}{4}{5}{6}_{7}{8}{9}{10}_{11}{12}{13}{14}__0_0000_0000_0000 }};");
        thirteen_times!("({}-{} clubs) => {{ 0b{}_{}{}{}{}_{}{}{}{}_{}{}{}{} }};");
        thirteen_times!("({}-{} diamonds) => {{ 0b{}_{}{}{}{}_{}{}{}{}_{}{}{}{}__0_0000_0000_0000 }};");
        thirteen_times!("({}-{} hearts) => {{ 0b{}_{}{}{}{}_{}{}{}{}_{}{}{}{}__0_0000_0000_0000__0_0000_0000_0000 }};");
        thirteen_times!("({}-{} spades) => {{ 0b{}_{}{}{}{}_{}{}{}{}_{}{}{}{}__0_0000_0000_0000__0_0000_0000_0000__0_0000_0000_0000 }};");
    }
    p!("}\n\n");

    let suits = ["black", "red", "clubs", "diamonds", "hearts", "spades"];

    for (start, end) in sorted_extrema.iter() {
        for suit in suits.iter() {
            pf!("consecutive_ranks!({0}-{1} {2}),\n", start, end, suit);
        }
    }

    p!("\n\n");

    for (start, end) in sorted_extrema.iter() {
        for suit in suits.iter() {
            pf!(
                "consecutive_ranks!({0}-{1} {2}) => consecutive_ranks_result!({0}=>{1}, {2}),\n",
                start,
                end,
                suit
            );
        }
    }

    const DECK_SIZE: u8 = 52;

    p!("\n\n");
    p!("macro_rules! card_pattern {");
    for i in 0..DECK_SIZE {
        pf!("({}) => {{ {:052b} }};", i, 1u64 << i);
    }
    p!("}");

    p!("\n\n");
    for i in 0..DECK_SIZE {
        pf!("card_pattern!({}),\n", i);
    }

    p!("\n\noptimal_toy_flags:\n");

    let optimal_toy_flags = get_optimal_toy_flags();

    for flags in optimal_toy_flags {
        p!("&[");

        p!(&special_flags_to_string(flags));

        p!("],\n");
    }

    match file.write_all(macros.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn get_sorted_extrema(span: u8) -> Vec<(u8, u8)> {
    let mut output = Vec::new();

    for end in 1..(span - 1) {
        output.push((0, end));
    }

    for start in 1..span {
        for end in (start + 1)..span {
            output.push((start, end));
        }
    }

    output.sort_by_key(|(start, end)| 255 - (end - start));

    output
}

#[test]
fn test_get_sorted_extrema_on_small_input() {
    let actual_extrema = get_sorted_extrema(4);

    let actual_flags: Vec<u8> = actual_extrema
        .into_iter()
        .map(|(start, end)| {
            let range = start..=end;

            macro_rules! in_range_shifted {
                ($e:expr) => {
                    if range.contains(&$e) {
                        1u8 << $e
                    } else {
                        0u8
                    }
                };
            }

            in_range_shifted!(3)
                | in_range_shifted!(2)
                | in_range_shifted!(1)
                | in_range_shifted!(0)
        }).collect();

    let expected_flags = vec![0b1110, 0b0111, 0b1100, 0b0110, 0b0011];

    let actual: Vec<u8> = actual_flags
        .into_iter()
        .map(|f: u8| f.count_ones() as u8)
        .collect();
    let expected: Vec<u8> = expected_flags
        .into_iter()
        .map(|f: u8| f.count_ones() as u8)
        .collect();

    assert_eq!(expected, actual);
}

type ToyFlags = u8;

pub const RANK_COUNT: ToyFlags = 4;

pub const CLUBS_FLAGS: ToyFlags = 0b0000_1111;
pub const DIAMONDS_FLAGS: ToyFlags = CLUBS_FLAGS << RANK_COUNT;

macro_rules! rank_pattern {
    (0) => {
        0b0001_0001
    };
    (1) => {
        0b0010_0010
    };
    (2) => {
        0b0100_0100
    };
    (3) => {
        0b1000_1000
    };
}

macro_rules! consecutive_ranks {
    (0-1 clubs) => {
        0b0000_0011
    };
    (0-2 clubs) => {
        0b0000_0111
    };
    (1-2 clubs) => {
        0b0000_0110
    };
    (1-3 clubs) => {
        0b0000_1110
    };
    (2-3 clubs) => {
        0b0000_1100
    };
    (0-1 diamonds) => {
        0b0011_0000
    };
    (0-2 diamonds) => {
        0b0111_0000
    };
    (1-2 diamonds) => {
        0b0110_0000
    };
    (1-3 diamonds) => {
        0b1110_0000
    };
    (2-3 diamonds) => {
        0b1100_0000
    };
}

const SPECIAL_FLAGS: [ToyFlags; 25] = [
    0b1111_1111,
    DIAMONDS_FLAGS,
    CLUBS_FLAGS,
    rank_pattern!(0),
    rank_pattern!(1),
    rank_pattern!(2),
    rank_pattern!(3),
    consecutive_ranks!(0-1 clubs),
    consecutive_ranks!(0-2 clubs),
    consecutive_ranks!(1-2 clubs),
    consecutive_ranks!(1-3 clubs),
    consecutive_ranks!(2-3 clubs),
    consecutive_ranks!(0-1 diamonds),
    consecutive_ranks!(0-2 diamonds),
    consecutive_ranks!(1-2 diamonds),
    consecutive_ranks!(1-3 diamonds),
    consecutive_ranks!(2-3 diamonds),
    0b1000_0000,
    0b0100_0000,
    0b0010_0000,
    0b0001_0000,
    0b0000_1000,
    0b0000_0100,
    0b0000_0010,
    0b0000_0001,
];

fn special_flags_to_str(flags: ToyFlags) -> &'static str {
    match flags {
        0b1111_1111 => stringify!(0b1111_1111),
        DIAMONDS_FLAGS => stringify!(DIAMONDS_FLAGS),
        CLUBS_FLAGS => stringify!(CLUBS_FLAGS),
        rank_pattern!(0) => stringify!(rank_pattern!(0)),
        rank_pattern!(1) => stringify!(rank_pattern!(1)),
        rank_pattern!(2) => stringify!(rank_pattern!(2)),
        rank_pattern!(3) => stringify!(rank_pattern!(3)),
        consecutive_ranks!(0-1 clubs) => stringify!(consecutive_ranks!(0-1 clubs)),
        consecutive_ranks!(0-2 clubs) => stringify!(consecutive_ranks!(0-2 clubs)),
        consecutive_ranks!(1-2 clubs) => stringify!(consecutive_ranks!(1-2 clubs)),
        consecutive_ranks!(1-3 clubs) => stringify!(consecutive_ranks!(1-3 clubs)),
        consecutive_ranks!(2-3 clubs) => stringify!(consecutive_ranks!(2-3 clubs)),
        consecutive_ranks!(0-1 diamonds) => stringify!(consecutive_ranks!(0-1 diamonds)),
        consecutive_ranks!(0-2 diamonds) => stringify!(consecutive_ranks!(0-2 diamonds)),
        consecutive_ranks!(1-2 diamonds) => stringify!(consecutive_ranks!(1-2 diamonds)),
        consecutive_ranks!(1-3 diamonds) => stringify!(consecutive_ranks!(1-3 diamonds)),
        consecutive_ranks!(2-3 diamonds) => stringify!(consecutive_ranks!(2-3 diamonds)),
        0b1000_0000 => stringify!(0b1000_0000),
        0b0100_0000 => stringify!(0b0100_0000),
        0b0010_0000 => stringify!(0b0010_0000),
        0b0001_0000 => stringify!(0b0001_0000),
        0b0000_1000 => stringify!(0b0000_1000),
        0b0000_0100 => stringify!(0b0000_0100),
        0b0000_0010 => stringify!(0b0000_0010),
        0b0000_0001 => stringify!(0b0000_0001),
        _ => {unimplemented!("special_flags_to_str")}
    }
}

fn special_flags_to_string(flags: Vec<ToyFlags>) -> String {
    let mut output = String::new();

    let mut sep = "";
    for flag in flags {
        output.push_str(sep);
        output.push_str(special_flags_to_str(flag));
        sep = ", ";
    }

    output
}


fn get_optimal_toy_flags() -> Vec<Vec<ToyFlags>> {
    fn possibility_helper(possibilities: &mut Vec<Vec<ToyFlags>>, prev_tracking_flags: &mut ToyFlags, prev_possibility: &mut Vec<ToyFlags>, relevant_flags: &[ToyFlags]) {
        if *prev_tracking_flags == 0 {
            possibilities.push(prev_possibility.clone());
            return;
        }

        for i in 0..relevant_flags.len() {
            let flag = relevant_flags[i];
            if flag & *prev_tracking_flags != 0 {
                let mut tracking_flags = *prev_tracking_flags;
                tracking_flags &= !flag;
                let mut possibility = prev_possibility.clone();
                possibility.push(flag);

                possibility_helper(possibilities, &mut tracking_flags, &mut possibility, &relevant_flags[i + 1..]);
            }
        }
    }

    let mut output = Vec::with_capacity(256);

    for i in 0..256 {
        let i = i as ToyFlags;
        //let's start super naive and just genreate all possibilities.
        let mut possibilities: Vec<Vec<ToyFlags>> = Vec::new();

        let relevant_flags: Vec<_> = SPECIAL_FLAGS.iter().filter(|&f| f & !i == 0).map(|&x| x).collect();

        for &starting_flag in relevant_flags.iter() {
            let mut possibility: Vec<ToyFlags> = Vec::new();
            possibility.push(starting_flag);

            let mut tracking_flags = i & !starting_flag;

            //we assume that the special flags can always cover the flags
            let other_relevant_flags: Vec<ToyFlags> = relevant_flags.iter().filter(|&&f| f != starting_flag).map(|&x| x).collect();
            possibility_helper(&mut possibilities, &mut tracking_flags, &mut possibility, &other_relevant_flags);
        }

        let flags = possibilities.into_iter().min_by_key(|v| v.len()).unwrap_or_default();

        output.push(flags);
    }

    output
}
