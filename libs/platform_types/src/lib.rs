#[macro_use]
extern crate bitflags;

#[derive(Clone, Copy, Default, Debug)]
pub struct Input {
    pub gamepad: Button,
    pub previous_gamepad: Button,
}

impl Input {
    pub fn new() -> Self {
        Input {
            gamepad: Button::empty(),
            previous_gamepad: Button::empty(),
        }
    }

    pub fn pressed_this_frame(&self, buttons: Button) -> bool {
        !self.previous_gamepad.contains(buttons) && self.gamepad.contains(buttons)
    }

    pub fn released_this_frame(&self, buttons: Button) -> bool {
        self.previous_gamepad.contains(buttons) && !self.gamepad.contains(buttons)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SFX {
    CardPlace,
    CardSlide,
    ButtonPress,
}

impl SFX {
    pub fn to_sound_key(&self) -> &'static str {
        match *self {
            SFX::CardPlace => "cardPlace",
            SFX::CardSlide => "cardSlide",
            SFX::ButtonPress => "buttonPress",
        }
    }
}

pub struct Speaker {
    requests: Vec<SFX>,
}

impl Speaker {
    pub fn new() -> Self {
        Speaker {
            requests: Vec::with_capacity(8),
        }
    }

    pub fn clear(&mut self) {
        self.requests.clear();
    }

    pub fn request_sfx(&mut self, sfx: SFX) {
        self.requests.push(sfx);
    }

    pub fn slice(&self) -> &[SFX] {
        &self.requests
    }
}

// These values are deliberately picked to be the same as the ones in NES' input registers.
bitflags! {
    #[derive(Default)]
    pub struct Button: u8 {
        const A          = 1 << 0;
        const B          = 1 << 1;
        const SELECT     = 1 << 2;
        const START      = 1 << 3;
        const UP         = 1 << 4;
        const DOWN       = 1 << 5;
        const LEFT       = 1 << 6;
        const RIGHT      = 1 << 7;
    }
}

pub type Logger = Option<fn(&str) -> ()>;

pub type StateParams = ([u8; 16], Logger, Logger);

pub trait State {
    fn frame(&mut self) -> (&[u32], &[SFX]);

    fn press(&mut self, button: Button);

    fn release(&mut self, button: Button);
}
