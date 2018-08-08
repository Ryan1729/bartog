#[cfg(feature = "invariant-checking")]
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
macro_rules! invariant_violation {
    ($code:block, $($rest:tt)*) => {
        $code
    };
    ($($whatever:tt)*) => {};
}

#[cfg(feature = "invariant-checking")]
macro_rules! invariant_assert {
    ($($arg:tt)+) => ({
        assert!($($arg)*)
    });
}

#[cfg(not(feature = "invariant-checking"))]
macro_rules! invariant_assert {
    ($($whatever:tt)*) => {};
}

pub mod rendering;
pub use rendering::draw_winning_screen;
pub use rendering::Framebuffer;

pub mod inner_common;
pub use inner_common::*;

pub mod game_state;
pub use game_state::*;

#[derive(Clone, Copy, Default, Debug)]
pub struct Input {
    pub gamepad: Button::Ty,
    pub previous_gamepad: Button::Ty,
}

impl Input {
    pub fn new() -> Self {
        Input {
            gamepad: Button::Ty::empty(),
            previous_gamepad: Button::Ty::empty(),
        }
    }

    pub fn pressed_this_frame(&self, buttons: Button::Ty) -> bool {
        !self.previous_gamepad.contains(buttons) && self.gamepad.contains(buttons)
    }
}

// These values are deliberately picked to be the same as the ones in NES' input registers.
pub mod Button {
    bitflags! {
        #[derive(Default)]
        pub flags Ty: u8 {
            const A          = 1 << 0,
            const B          = 1 << 1,
            const Select     = 1 << 2,
            const Start      = 1 << 3,
            const Up         = 1 << 4,
            const Down       = 1 << 5,
            const Left       = 1 << 6,
            const Right      = 1 << 7
        }
    }
}

pub type Logger = Option<fn(&str) -> ()>;

pub fn log(logger: Logger, s: &str) {
    if let Some(l) = logger {
        l(s);
    }
}
