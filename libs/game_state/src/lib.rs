#![allow(non_snake_case)]
#[macro_use]
extern crate common;

extern crate platform_types;

extern crate rand;

mod game_state;
pub use game_state::*;

mod card_flags;
pub use card_flags::*;
