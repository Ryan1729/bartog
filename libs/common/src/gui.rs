use inner_common::*;
use platform_types::{Button, Input, Speaker, SFX};
use rendering::{center_line_in_rect, Framebuffer};
use std::cmp::min;

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

fn button_press(context: &mut UIContext, input: Input, speaker: &mut Speaker, id: UIId) -> bool {
    let mut result = false;

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

    result
}

// Add the extra bit to `y` because the current graphics looks better that way.
// In particular, it centers the character vertically within the tile.
const TEXT_HEIGHT_OFFSET: u8 = (FONT_SIZE / 4);

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
    let id = spec.id;

    let result = button_press(context, input, speaker, id);

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
    framebuffer.print(text, x, y + TEXT_HEIGHT_OFFSET, WHITE_INDEX);

    result
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
    let id = spec.id;

    let result = button_press(context, input, speaker, id);

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

    result
}

pub struct RowSpec {
    pub text: String,
    pub x: u8,
    pub y: u8,
    pub w: u8,
    pub id: UIId,
}

//calling this once will swallow multiple presses on the row. We could either
//pass in and return the number of presses to fix that, or this could simply be
//called multiple times per frame (once for each click).
pub fn do_pressable_row(
    framebuffer: &mut Framebuffer,
    context: &mut UIContext,
    input: Input,
    speaker: &mut Speaker,
    spec: &RowSpec,
) -> bool {
    let id = spec.id;

    let result = button_press(context, input, speaker, id);

    if context.active == id && input.gamepad.contains(Button::A) {
        framebuffer.row_pressed(spec.x, spec.y, spec.w);
    } else if context.hot == id {
        framebuffer.row_hot(spec.x, spec.y, spec.w);
    } else {
        framebuffer.row(spec.x, spec.y, spec.w);
    }

    // TODO make an elipisis character and draw it instead of the last character of text.
    // Seems like the best way would be to center on the length + 1 and then draw the
    // first `len` characters then figure out where the elipisis should go and draw that.
    let full_text = spec.text.as_bytes();

    let text = &full_text[..min(full_text.len(), (spec.w / FONT_ADVANCE) as usize)];

    let (x, y) = center_line_in_rect(text.len() as u8, ((spec.x, spec.y), (spec.w, 1)));

    //The row is meant to be only one ... row ... high. So don't bother reflowing.
    framebuffer.print(text, x, y + 3 * TEXT_HEIGHT_OFFSET, WHITE_INDEX);

    result
}
