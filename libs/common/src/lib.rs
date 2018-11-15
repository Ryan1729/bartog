#![allow(non_snake_case)]

//This is useful since I can only use println! in non browser exectutions,
//(it crashes otherwise) and this makes it easy to check that the only
//instances of println are in these macros.
#[allow(unused_macros)]
#[macro_export]
macro_rules! test_println {
    ($($arg:tt)*) => ({
        if cfg!(test) {
            println!($($arg)*);
        }
    })
}

#[macro_export]
macro_rules! test_log {
    ($e:expr) => {{
        if cfg!(test) {
            println!(concat!(stringify!($e), ": {:#?}"), $e);
        }
    }};
}

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
    (Distribution<$type:ty> for Standard by picking from $all:expr) => {
        impl Distribution<$type> for Standard {
            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $type {
                let all = $all;
                let i = rng.gen_range(0, all.len());
                all[i]
            }
        }
    };
}

#[cfg(test)]
extern crate quickcheck;

extern crate platform_types;

extern crate features;
pub use features::*;

extern crate rand;

mod rendering;
pub use rendering::*;

mod inner_common;
pub use inner_common::*;

mod card_animation;
pub use card_animation::*;

mod text;
pub use text::*;

mod gui;
pub use gui::*;

mod hand;
pub use hand::*;

mod traits;
pub use traits::*;

mod english;
pub use english::*;

mod card_flags;
pub use card_flags::*;
