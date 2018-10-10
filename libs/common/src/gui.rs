use inner_common::*;
use platform_types::{Button, Input, Speaker, SFX};
use rendering::{center_line_in_rect, Framebuffer};

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

//calling this once will swallow multiple presses on the button. We could either
//pass in and return the number of presses to fix that, or this could simply be
//called multiple times per frame (once for each click).
pub fn do_button(
    framebuffer: &mut Framebuffer,
    context: &mut UIContext,
    input: Input,
    speaker: &mut Speaker,
    spec: &ButtonSpec,
) -> bool {
    let mut result = false;

    let id = spec.id;

    if context.active == id {
        if input.released_this_frame(Button::A) {
            result = context.hot == id;

            context.set_not_active();
        }
        context.set_next_hot(id);
    } else if context.hot == id {
        if input.pressed_this_frame(Button::A) {
            context.set_active(id);
            speaker.request_sfx(SFX::ButtonPress);
        }
        context.set_next_hot(id);
    }

    if context.active == id && input.gamepad.contains(Button::A) {
        framebuffer.button_pressed(spec.x, spec.y, spec.w, spec.h);
    } else if context.hot == id {
        framebuffer.button_hot(spec.x, spec.y, spec.w, spec.h);
    } else {
        framebuffer.button(spec.x, spec.y, spec.w, spec.h);
    }

    let text = spec.text.as_bytes();

    let (x, y) = center_line_in_rect(text.len() as u8, ((spec.x, spec.y), (spec.w, spec.h)));

    //Long labels aren't great UX anyway, I think, so don't bother reflowing.
    //Add the extra bit to `y` because the current graphics looks better that way.
    framebuffer.print(spec.text.as_bytes(), x, y + (FONT_SIZE / 4), WHITE_INDEX);

    return result;
}

pub struct CheckboxSpec {
    pub text: String,
    pub x: u8,
    pub y: u8,
    pub id: UIId,
    pub checked: bool,
}

//This returns whetrher the checkbox was toggled, not the next state of the box.
//see also: note on do_button above.
pub fn do_checkbox(
    framebuffer: &mut Framebuffer,
    context: &mut UIContext,
    input: Input,
    speaker: &mut Speaker,
    spec: &CheckboxSpec,
) -> bool {
    let mut result = false;

    let id = spec.id;

    if context.active == id {
        if input.released_this_frame(Button::A) {
            result = context.hot == id;

            context.set_not_active();
        }
        context.set_next_hot(id);
    } else if context.hot == id {
        if input.pressed_this_frame(Button::A) {
            context.set_active(id);
            speaker.request_sfx(SFX::ButtonPress);
        }
        context.set_next_hot(id);
    }

    if context.active == id && input.gamepad.contains(Button::A) {
        framebuffer.checkbox_pressed(spec.x, spec.y, spec.checked);
    } else if context.hot == id {
        framebuffer.checkbox_hot(spec.x, spec.y, spec.checked);
    } else {
        framebuffer.checkbox(spec.x, spec.y, spec.checked);
    }

    //Long labels aren't great UX anyway, I think, so don't bother reflowing.
    //Add the extra bit to `y` because the current graphics looks better that way.
    framebuffer.print(
        spec.text.as_bytes(),
        spec.x.saturating_add(SPRITE_SIZE),
        spec.y + (FONT_SIZE / 4),
        WHITE_INDEX,
    );

    return result;
}
