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

    const RANK_COUNT: usize = 13;

    let mut macros = String::new();

    macro_rules! p {
        ($s:expr) => {
            macros.push_str($s)
        };
    }

    macro_rules! pf {
        ($($stuff:tt)*) => {p!(&format!($($stuff)*))};
    }

    p!("// generated file!\n\n");
    p!("macro_rules! consecutive_ranks {");

    macro_rules! four_times {
        ($fmt_str:expr) => {
            for start in 0..RANK_COUNT {
                for end in (start + 1)..RANK_COUNT {
                    let range = start..=end;

                    macro_rules! in_range {
                        ($e:expr) => {
                            if range.contains(&$e) {
                                1
                            } else {
                                0
                            }
                        };
                    }

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
                }
            }
        };
    }

    four_times!("({}-{} clubs) => {{ 0b{}_{}{}{}{}_{}{}{}{}_{}{}{}{} }};");
    four_times!("({}-{} diamonds) => {{ 0b{}_{}{}{}{}_{}{}{}{}_{}{}{}{}__0_0000_0000_0000 }};");
    four_times!("({}-{} hearts) => {{ 0b{}_{}{}{}{}_{}{}{}{}_{}{}{}{}__0_0000_0000_0000__0_0000_0000_0000 }};");
    four_times!("({}-{} spades) => {{ 0b{}_{}{}{}{}_{}{}{}{}_{}{}{}{}__0_0000_0000_0000__0_0000_0000_0000__0_0000_0000_0000 }};");

    p!("}\n\n");

    for suit in ["clubs", "diamonds", "hearts", "spades"].iter() {
        for start in 0..RANK_COUNT {
            for end in (start + 1)..RANK_COUNT {
                pf!(
                    "consecutive_ranks!({0}-{1} {2}) => consecutive_ranks_result!({0}=>{1}, {2}),\n",
                    start,
                    end,
                    suit
                );
            }
        }
    }

    match file.write_all(macros.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
