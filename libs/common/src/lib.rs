#![allow(non_snake_case)]

#[macro_export]
macro_rules! d {
    () => {
        Default::default()
    };
}

#[macro_export]
macro_rules! nu8 {
    ($byte:expr) => {{
        use std::num::NonZeroU8;
        NonZeroU8::new($byte).unwrap()
    }};
}

#[macro_export]
macro_rules! implement {
    (BorrowMut<$borrowed:ty> for $implementing:ty: $that:ident, $ref_expr:expr) => {
        use std::borrow::Borrow;
        impl Borrow<$borrowed> for $implementing {
            fn borrow(&self) -> &$borrowed {
                let $that = self;
                &$ref_expr
            }
        }

        use std::borrow::BorrowMut;
        impl BorrowMut<$borrowed> for $implementing {
            fn borrow_mut(&mut self) -> &mut $borrowed {
                let $that = self;
                &mut $ref_expr
            }
        }
    };
    (<$a:lifetime> BorrowMut<$borrowed:ty> for $implementing:ty: $that:ident, $ref_expr:expr) => {
        use std::borrow::Borrow;
        impl<$a> Borrow<$borrowed> for $implementing {
            fn borrow(&self) -> &$borrowed {
                let $that = self;
                &$ref_expr
            }
        }

        use std::borrow::BorrowMut;
        impl<$a> BorrowMut<$borrowed> for $implementing {
            fn borrow_mut(&mut self) -> &mut $borrowed {
                let $that = self;
                &mut $ref_expr
            }
        }
    };
    (BorrowPairMut<$borrowed1:ty, $borrowed2:ty> for $implementing:ty: $that:ident, ($ref1_expr:expr, $ref2_expr:expr)) => {
        use common::BorrowPair;
        impl BorrowPair<$borrowed1, $borrowed2> for $implementing {
            fn borrow_pair(&self) -> (&$borrowed1, &$borrowed2) {
                let $that = self;
                (&$ref1_expr, &$ref2_expr)
            }
        }

        use common::BorrowPairMut;
        impl BorrowPairMut<$borrowed1, $borrowed2> for $implementing {
            fn borrow_pair_mut(&mut self) -> (&mut $borrowed1, &mut $borrowed2) {
                let $that = self;
                (&mut $ref1_expr, &mut $ref2_expr)
            }
        }
    };
    (<$a:lifetime> BorrowPairMut<$borrowed1:ty, $borrowed2:ty> for $implementing:ty: $that:ident, ($ref1_expr:expr, $ref2_expr:expr)) => {
        use common::BorrowPair;
        impl<$a> BorrowPair<$borrowed1, $borrowed2> for $implementing {
            fn borrow_pair(&self) -> (&$borrowed1, &$borrowed2) {
                let $that = self;
                (&$ref1_expr, &$ref2_expr)
            }
        }

        use common::BorrowPairMut;
        impl<$a> BorrowPairMut<$borrowed1, $borrowed2> for $implementing {
            fn borrow_pair_mut(&mut self) -> (&mut $borrowed1, &mut $borrowed2) {
                let $that = self;
                (&mut $ref1_expr, &mut $ref2_expr)
            }
        }
    };
    (from_rng for $type:ty, by picking from $all:expr) => {
        impl $type {
            pub fn from_rng(rng: &mut Xs) -> $type {
                let all = $all;
                let i = xs_range(rng, 0..all.len() as _) as usize;
                all[i]
            }
        }
    };
}

#[cfg(test)]
extern crate quickcheck;

extern crate platform_types;

extern crate inner_common;
pub use inner_common::*;

extern crate card_flags;
pub use card_flags::*;

extern crate features;
pub use features::*;

mod rendering;
pub use self::rendering::*;

mod card_animation;
pub use self::card_animation::*;

mod text;
pub use self::text::*;

mod gui;
pub use self::gui::*;

mod hand;
pub use self::hand::*;

mod traits;
pub use self::traits::*;

