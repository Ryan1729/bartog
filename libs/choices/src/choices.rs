use common::*;
use game_state::{can_play, Choice, Chosen, GameState};
use platform_types::{log, Button, Input, Speaker, SFX};

//calling this once will swallow multiple presses on the button. We could either
//pass in and return the number of presses to fix that, or this could simply be
//called multiple times per frame (once for each click).
fn do_button(
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

pub fn choose_play_again(state: &mut GameState) -> Option<()> {
    match state.choice {
        Choice::NoChoice => {
            state.choice = Choice::OfUnit;
            None
        }
        Choice::Already(Chosen::Unit(unit)) => {
            state.choice = Choice::NoChoice;
            Some(unit)
        }
        _ => None,
    }
}

#[inline]
pub fn do_unit_choice(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
    speaker: &mut Speaker,
) {
    framebuffer.full_window();

    {
        let winner_text = reflow(
            &state.get_winner_text(),
            NINE_SLICE_MAX_INTERIOR_WIDTH_IN_CHARS as usize,
        );

        let dimensions = get_text_dimensions(winner_text.as_bytes());

        let (x, _) = center_rect_in_rect(
            dimensions,
            (
                (SPRITE_SIZE, SPRITE_SIZE),
                (NINE_SLICE_MAX_INTERIOR_SIZE, NINE_SLICE_MAX_INTERIOR_SIZE),
            ),
        );

        framebuffer.print(winner_text.as_bytes(), x, SPRITE_SIZE, 6);
    }

    {
        let question = b"would you like to play again?";

        let (x, y) = center_line_in_rect(
            question.len() as u8,
            (
                (SPRITE_SIZE, SPRITE_SIZE),
                (NINE_SLICE_MAX_INTERIOR_SIZE, NINE_SLICE_MAX_INTERIOR_SIZE),
            ),
        );

        framebuffer.print(question, x, y, WHITE_INDEX);
    }

    let w = SPRITE_SIZE * 5;
    let h = SPRITE_SIZE * 3;
    let y = SCREEN_HEIGHT as u8 - (h + SPRITE_SIZE);

    let (x, _) = center_rect_in_rect((w, h), ((0, y), (SCREEN_WIDTH as u8, h)));

    let text = "yes".to_owned();

    let spec1 = ButtonSpec {
        x,
        y,
        w,
        h,
        id: 1,
        text,
    };

    if do_button(framebuffer, &mut state.context, input, speaker, &spec1) {
        state.choice = Choice::Already(Chosen::Unit(()));
    }

    if state.context.hot != 1 {
        state.context.set_next_hot(1);
    }
}

#[inline]
pub fn do_bool_choice(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
    speaker: &mut Speaker,
) {
    framebuffer.full_window();

    {
        let question = b"Close this window?";

        let (x, y) = center_line_in_rect(
            question.len() as u8,
            (
                (SPRITE_SIZE, SPRITE_SIZE),
                (NINE_SLICE_MAX_INTERIOR_SIZE, NINE_SLICE_MAX_INTERIOR_SIZE),
            ),
        );

        framebuffer.print(question, x, y, WHITE_INDEX);
    }

    let w = SPRITE_SIZE * 5;
    let h = SPRITE_SIZE * 3;
    let y = SCREEN_HEIGHT as u8 - (h + SPRITE_SIZE);

    let spec1 = ButtonSpec {
        x: SPRITE_SIZE,
        y,
        w,
        h,
        id: 1,
        text: "yes".to_owned(),
    };

    if do_button(framebuffer, &mut state.context, input, speaker, &spec1) {
        state.choice = Choice::Already(Chosen::Bool(true));
    }

    let spec2 = ButtonSpec {
        x: SCREEN_WIDTH as u8 - (w + SPRITE_SIZE),
        y,
        w,
        h,
        id: 2,
        text: "no".to_owned(),
    };

    if do_button(framebuffer, &mut state.context, input, speaker, &spec2) {
        state.choice = Choice::Already(Chosen::Bool(false));
    }

    if state.context.hot != 1 && state.context.hot != 2 {
        state.context.set_next_hot(1);
    } else if input.pressed_this_frame(Button::Left) || input.pressed_this_frame(Button::Right) {
        if state.context.hot == 1 {
            state.context.set_next_hot(2);
        } else {
            state.context.set_next_hot(1);
        }
    }
}

pub fn choose_suit(state: &mut GameState) -> Option<Suit> {
    match state.choice {
        Choice::NoChoice => {
            state.choice = Choice::OfSuit;
            None
        }
        Choice::Already(Chosen::Suit(suit)) => {
            state.choice = Choice::NoChoice;
            Some(suit)
        }
        _ => None,
    }
}

#[inline]
pub fn do_suit_choice(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
    speaker: &mut Speaker,
) {
    framebuffer.full_window();
    {
        let text = b"choose a suit for the 8 to be";

        let (x, _) = center_line_in_rect(
            text.len() as u8,
            (
                (SPRITE_SIZE, SPRITE_SIZE),
                (NINE_SLICE_MAX_INTERIOR_SIZE, NINE_SLICE_MAX_INTERIOR_SIZE),
            ),
        );

        framebuffer.print(text, x, SPRITE_SIZE * 2, WHITE_INDEX);
    }

    let w = NINE_SLICE_MAX_INTERIOR_SIZE;
    let h = SPRITE_SIZE * 3;
    let x = SPRITE_SIZE;

    for (i, suit) in Suits::ALL.iter().cloned().enumerate() {
        let i = (i + 1) as u8;

        let (_, suit_char) = get_suit_colour_and_char(suit);

        let mut text = String::with_capacity(1);
        text.push(char::from(suit_char));

        let spec = ButtonSpec {
            x,
            y: h * i,
            w,
            h,
            id: i,
            text,
        };

        if do_button(framebuffer, &mut state.context, input, speaker, &spec) {
            state.choice = Choice::Already(Chosen::Suit(suit));
        }
    }

    if state.context.hot == 0 || state.context.hot > 4 {
        state.context.set_next_hot(1);
    } else if input.pressed_this_frame(Button::Up) {
        let next = dice_mod(state.context.hot - 1, 4);
        state.context.set_next_hot(next);
    } else if input.pressed_this_frame(Button::Down) {
        let next = dice_mod(state.context.hot + 1, 4);
        state.context.set_next_hot(next);
    }
}

fn dice_mod(x: u8, m: u8) -> u8 {
    if x == 0 {
        m
    } else {
        (x.saturating_sub(1) % m).saturating_add(1)
    }
}

use std::mem;

pub fn choose_can_play_graph(state: &mut GameState) -> Vec<can_play::Change> {
    match state.choice {
        Choice::NoChoice => {
            state.choice = Choice::OfCanPlayGraph(Vec::new());
            Vec::new()
        }
        Choice::Already(Chosen::CanPlayGraph(_)) => {
            if let Choice::Already(Chosen::CanPlayGraph(changes)) =
                mem::replace(&mut state.choice, Choice::NoChoice)
            {
                changes
            } else {
                invariant_violation!({ Vec::new() }, "Somehow we're multi-threaded or somthing?!")
            }
        }
        _ => Vec::new(),
    }
}

#[inline]
pub fn do_can_play_graph_choice(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
    speaker: &mut Speaker,
) {
    let logger = state.get_logger();
    if let Choice::OfCanPlayGraph(ref mut changes) = state.choice {
        framebuffer.full_window();

        {
            let text = b"choose a card to change.";

            let (x, _) = center_line_in_rect(
                text.len() as u8,
                (
                    (SPRITE_SIZE, SPRITE_SIZE),
                    (NINE_SLICE_MAX_INTERIOR_SIZE, NINE_SLICE_MAX_INTERIOR_SIZE),
                ),
            );

            framebuffer.print(text, x, SPRITE_SIZE * 2, WHITE_INDEX);
        }

        let w = SPRITE_SIZE * 5;
        let h = SPRITE_SIZE * 3;

        {
            let y = SPRITE_SIZE * 4;

            let spec = ButtonSpec {
                x: SCREEN_WIDTH as u8 - (w + SPRITE_SIZE),
                y,
                w,
                h,
                id: 1,
                text: "reset".to_owned(),
            };

            if do_button(framebuffer, &mut state.context, input, speaker, &spec) {
                log(logger, "reset");
            }
        }

        {
            let y = SPRITE_SIZE * 7;

            let spec = ButtonSpec {
                x: SCREEN_WIDTH as u8 - (w + SPRITE_SIZE),
                y,
                w,
                h,
                id: 2,
                text: "cancel".to_owned(),
            };

            if do_button(framebuffer, &mut state.context, input, speaker, &spec) {
                log(logger, "cancel");
            }
        }

        {
            let y = SPRITE_SIZE * 10;

            let spec = ButtonSpec {
                x: SCREEN_WIDTH as u8 - (w + SPRITE_SIZE),
                y,
                w,
                h,
                id: 3,
                text: "done".to_owned(),
            };

            if do_button(framebuffer, &mut state.context, input, speaker, &spec) {
                log(logger, "done");
            }
        }

        let w = SPRITE_SIZE * 10;
        let h = SPRITE_SIZE * 3;
        let x = SPRITE_SIZE;

        const ID_OFFSET: UIId = 4;

        for (i, card) in (0..DECK_SIZE)
            .skip(state.context.hot.saturating_sub(4) as usize)
            .take(4)
            .enumerate()
        {
            let id = card + ID_OFFSET;

            let text = get_card_string(card);

            let spec = ButtonSpec {
                x,
                y: h * (i + 1) as u8 + (SPRITE_SIZE as u8 / 2),
                w,
                h,
                id,
                text,
            };

            if do_button(framebuffer, &mut state.context, input, speaker, &spec) {
                log(logger, &spec.text);
            }
        }

        if state.context.hot < ID_OFFSET {
            if state.context.hot == 0 {
                state.context.set_next_hot(1);
            } else if input.pressed_this_frame(Button::Up) {
                let next = dice_mod(state.context.hot - 1, 3);
                state.context.set_next_hot(next);
            } else if input.pressed_this_frame(Button::Down) {
                let next = dice_mod(state.context.hot + 1, 3);
                state.context.set_next_hot(next);
            } else if input.pressed_this_frame(Button::Right)
                || input.pressed_this_frame(Button::Left)
            {
                state.context.set_next_hot(ID_OFFSET);
            }
        } else {
            if input.pressed_this_frame(Button::Right) || input.pressed_this_frame(Button::Left) {
                state.context.set_next_hot(1);
            } else {
                let mut unoffset = state.context.hot - ID_OFFSET;

                if input.pressed_this_frame(Button::Up) {
                    unoffset = unoffset.saturating_sub(1);
                } else if input.pressed_this_frame(Button::Down) {
                    unoffset = (unoffset + 1) % DECK_SIZE;
                }

                state.context.set_next_hot(unoffset + ID_OFFSET);
            }
        }
    } else {
        invariant_violation!(
            { state.choice = Choice::NoChoice },
            "`do_can_play_graph_choice` was called with the wrong choice type!"
        )
    }

    glog!(state, state.context);
}
