#![allow(non_snake_case)]
extern crate common;

extern crate platform_types;

extern crate rand;

extern crate lazy_static;

#[cfg(test)]
extern crate quickcheck;

mod game_state;
pub use game_state::*;

mod card_flags;
pub use card_flags::*;

pub mod can_play;

pub mod in_game;
