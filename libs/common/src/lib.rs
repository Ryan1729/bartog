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

#[macro_export]
//GameState log
macro_rules! glog {
    ($state:expr, $e:expr) => {{
        $state.log(&format!(concat!(stringify!($e), ": {:#?}"), $e));
    }};
}

#[macro_export]
//Logger log
macro_rules! llog {
    ($logger:expr, $e:expr) => {{
        log($logger, &format!(concat!(stringify!($e), ": {:#?}"), $e));
    }};
}

#[cfg(test)]
extern crate quickcheck;

extern crate platform_types;

extern crate rand;

mod rendering;
pub use rendering::*;

mod inner_common;
pub use inner_common::*;

mod animation;
pub use animation::*;

mod text;
pub use text::*;

mod gui;
pub use gui::*;
