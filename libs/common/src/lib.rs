#![allow(non_snake_case)]

#[cfg(feature = "invariant-checking")]
#[macro_export]
macro_rules! invariant_violation {
    () => ({
        console!(error, "invariant was violated!", &format!("{}:{}", file!(), line!()));
        panic!("invariant was violated!")
    });
    ($code:block, $($rest:tt)*) => {
        invariant_violation!($($rest)*)
    };
    ($msg:expr) => ({
        console!(error, $msg, &format!("{}:{}", file!(), line!()));
        panic!($msg)
    });
    ($msg:expr,) => (
        invariant_violation!($msg)
    );
    ($fmt:expr, $($arg:tt)+) => ({
        console!(error, $fmt, $($arg)*, &format!("{}:{}", file!(), line!()));
        panic!($fmt, $($arg)*)
    });
}

#[cfg(not(feature = "invariant-checking"))]
#[macro_export]
macro_rules! invariant_violation {
    ($code:block, $($rest:tt)*) => {
        $code
    };
    ($($whatever:tt)*) => {};
}

#[cfg(feature = "invariant-checking")]
#[macro_export]
macro_rules! invariant_assert {
    ($($arg:tt)+) => ({
        assert!($($arg)*)
    });
}

#[cfg(not(feature = "invariant-checking"))]
#[macro_export]
macro_rules! invariant_assert {
    ($($whatever:tt)*) => {};
}

#[cfg(feature = "invariant-checking")]
#[macro_export]
macro_rules! invariant_assert_eq {
    ($($arg:tt)+) => ({
        assert_eq!($($arg)*)
    });
}

#[cfg(not(feature = "invariant-checking"))]
#[macro_export]
macro_rules! invariant_assert_eq {
    ($($whatever:tt)*) => {};
}

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

extern crate platform_types;

extern crate rand;

pub mod rendering;
pub use rendering::Framebuffer;

pub mod inner_common;
pub use inner_common::*;

pub mod game_state;
pub use game_state::*;

pub mod animation;
pub use animation::*;

pub mod text;
pub use text::*;

pub type UIId = u8;

#[derive(Debug)]
pub struct UIContext {
    pub hot: UIId,
    pub active: UIId,
    pub next_hot: UIId,
}

impl UIContext {
    pub fn new() -> Self {
        UIContext {
            hot: 0,
            active: 0,
            next_hot: 0,
        }
    }

    pub fn set_not_active(&mut self) {
        self.active = 0;
    }
    pub fn set_active(&mut self, id: UIId) {
        self.active = id;
    }
    pub fn set_next_hot(&mut self, id: UIId) {
        self.next_hot = id;
    }
    pub fn set_not_hot(&mut self) {
        self.hot = 0;
    }
    pub fn frame_init(&mut self) {
        if self.active == 0 {
            self.hot = self.next_hot;
        }
        self.next_hot = 0;
    }
}

pub struct ButtonSpec {
    pub text: String,
    pub x: u8,
    pub y: u8,
    pub w: u8,
    pub h: u8,
    pub id: UIId,
}
