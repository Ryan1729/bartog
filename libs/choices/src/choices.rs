use common::*;
use game_state::{can_play, Choice, Chosen, GameState};
use platform_types::{log, Button, Input, Logger, Speaker, SFX};
use std::cmp::min;

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
            state.choice = Choice::OfCanPlayGraph(Default::default());
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

fn can_play_graph_choose_card(
    framebuffer: &mut Framebuffer,
    context: &mut UIContext,
    input: Input,
    speaker: &mut Speaker,
    choice_state: &mut can_play::ChoiceState,
    logger: Logger,
) {
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

        if do_button(framebuffer, context, input, speaker, &spec) {
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

        if do_button(framebuffer, context, input, speaker, &spec) {
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

        if do_button(framebuffer, context, input, speaker, &spec) {
            log(logger, "done");
        }
    }

    let w = SPRITE_SIZE * 10;
    let h = SPRITE_SIZE * 3;
    let x = SPRITE_SIZE;

    const FIRST_SCROLL_ID: UIId = 4;

    const SCROLL_BUTTON_COUNT: u8 = 4;
    for i in 0..SCROLL_BUTTON_COUNT {
        let id = i as UIId + FIRST_SCROLL_ID;
        let card = nth_next_card(choice_state.card, i);
        let text = get_card_string(card);

        let spec = ButtonSpec {
            x,
            y: h * (i + 1) as u8 + (SPRITE_SIZE as u8 / 2),
            w,
            h,
            id,
            text,
        };

        if do_button(framebuffer, context, input, speaker, &spec) {
            choice_state.card = card;
            choice_state.layer = can_play::Layer::Edges;
        }
    }

    if context.hot < FIRST_SCROLL_ID as _ {
        if context.hot == 0 {
            context.set_next_hot(1);
        } else if input.pressed_this_frame(Button::Up) {
            let next = dice_mod(context.hot - 1, 3);
            context.set_next_hot(next);
        } else if input.pressed_this_frame(Button::Down) {
            let next = dice_mod(context.hot + 1, 3);
            context.set_next_hot(next);
        } else if input.pressed_this_frame(Button::Right) || input.pressed_this_frame(Button::Left)
        {
            let next = (FIRST_SCROLL_ID - 1) + context.hot;
            context.set_next_hot(next);
        }
    } else {
        if input.pressed_this_frame(Button::Right) || input.pressed_this_frame(Button::Left) {
            let next = min(
                context.hot.saturating_sub(FIRST_SCROLL_ID) + 1,
                FIRST_SCROLL_ID - 1,
            );
            context.set_next_hot(next);
        } else {
            let mut unoffset = context.hot - FIRST_SCROLL_ID;

            if input.pressed_this_frame(Button::Up) {
                if unoffset == 0 {
                    choice_state.card = nth_next_card(choice_state.card, DECK_SIZE - 1) as _;
                } else {
                    unoffset -= 1;
                }
            } else if input.pressed_this_frame(Button::Down) {
                if unoffset == SCROLL_BUTTON_COUNT - 1 {
                    choice_state.card = nth_next_card(choice_state.card, 1) as _;
                } else {
                    unoffset = nth_next_card(unoffset, 1);
                }
            }

            context.set_next_hot(unoffset + FIRST_SCROLL_ID);
        }
    }
}

fn heading_y(i: i8) -> u8 {
    (SPRITE_SIZE as i8 * 2 + FONT_SIZE as i8 * i) as u8
}

fn can_play_graph_choose_edges(
    framebuffer: &mut Framebuffer,
    context: &mut UIContext,
    input: Input,
    speaker: &mut Speaker,
    choice_state: &mut can_play::ChoiceState,
    logger: Logger,
) {
    framebuffer.full_window();

    let mut max_heading_y = heading_y(-1);

    {
        let text = &[
            b"choose the cards the ",
            get_card_string(choice_state.card).as_bytes(),
            b" can be played on.",
        ]
            .concat();

        let reflowed = bytes_reflow(text, NINE_SLICE_MAX_INTERIOR_WIDTH_IN_CHARS as _);
        let lines = bytes_lines(&reflowed);

        for (i, line) in lines.enumerate() {
            let (x, _) = center_line_in_rect(
                line.len() as u8,
                (
                    (SPRITE_SIZE, SPRITE_SIZE),
                    (NINE_SLICE_MAX_INTERIOR_SIZE, NINE_SLICE_MAX_INTERIOR_SIZE),
                ),
            );

            max_heading_y = heading_y(i as i8);

            framebuffer.print(line, x, max_heading_y, WHITE_INDEX);
        }
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
            text: "ok".to_owned(),
        };

        if do_button(framebuffer, context, input, speaker, &spec) {
            log(logger, "ok");
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

        if do_button(framebuffer, context, input, speaker, &spec) {
            log(logger, "cancel");
        }
    }

    const FIRST_CHECKBOX_ID: UIId = 3;

    const SCROLL_ROWS_COUNT: u8 = 10;
    const SCROLL_COLS_COUNT: u8 = 2;

    invariant_assert_eq!(
        (DECK_SIZE / SCROLL_COLS_COUNT) * SCROLL_COLS_COUNT,
        DECK_SIZE,
    );
    let scroll_card = nth_next_card(choice_state.scroll_card, 0);

    for y in 0..SCROLL_ROWS_COUNT {
        for x in 0..SCROLL_COLS_COUNT {
            let i = x + y * SCROLL_COLS_COUNT;
            let id = i as UIId + FIRST_CHECKBOX_ID;

            let card = nth_next_card(scroll_card, i);
            let text = get_suit_rank_pair(card);

            let spec = CheckboxSpec {
                x: SPRITE_SIZE
                    + (SPRITE_SIZE * 2 + RANK_SUIT_PAIR_WITH_IN_CHARS * FONT_ADVANCE) * x,
                y: max_heading_y + SPRITE_SIZE * (y + 1) as u8 + (SPRITE_SIZE as u8 / 2),
                id,
                text,
                checked: choice_state.edges.has_card(card),
            };

            if do_checkbox(framebuffer, context, input, speaker, &spec) {
                choice_state.edges.toggle_card(card);
            }
        }
    }

    if context.hot < FIRST_CHECKBOX_ID as _ {
        if context.hot == 0 {
            context.set_next_hot(1);
        } else if input.pressed_this_frame(Button::Up) {
            let next = dice_mod(context.hot - 1, 2);
            context.set_next_hot(next);
        } else if input.pressed_this_frame(Button::Down) {
            let next = dice_mod(context.hot + 1, 2);
            context.set_next_hot(next);
        } else if input.pressed_this_frame(Button::Right) {
            if context.hot == 1 {
                context.set_next_hot(FIRST_CHECKBOX_ID);
            } else {
                context.set_next_hot(FIRST_CHECKBOX_ID + 3 * SCROLL_COLS_COUNT);
            }
        } else if input.pressed_this_frame(Button::Left) {
            if context.hot == 1 {
                context.set_next_hot(FIRST_CHECKBOX_ID + 1);
            } else {
                context.set_next_hot(FIRST_CHECKBOX_ID + 3 * SCROLL_COLS_COUNT + 1);
            }
        }
    } else {
        if input.pressed_this_frame(Button::Left) {
            if context.hot & 1 == 1 {
                if context.hot > FIRST_CHECKBOX_ID + 3 * SCROLL_COLS_COUNT {
                    context.set_next_hot(FIRST_CHECKBOX_ID - 1);
                } else {
                    context.set_next_hot(FIRST_CHECKBOX_ID - 2);
                }
            } else {
                let next = context.hot - 1;
                context.set_next_hot(next);
            }
        } else if input.pressed_this_frame(Button::Right) {
            if context.hot & 1 == 1 {
                let next = context.hot + 1;
                context.set_next_hot(next);
            } else {
                if context.hot > FIRST_CHECKBOX_ID + 3 * SCROLL_COLS_COUNT {
                    context.set_next_hot(FIRST_CHECKBOX_ID - 1);
                } else {
                    context.set_next_hot(FIRST_CHECKBOX_ID - 2);
                }
            }
        } else {
            let mut unoffset = context.hot - FIRST_CHECKBOX_ID;

            if input.pressed_this_frame(Button::Up) {
                if unoffset < 2 {
                    choice_state.scroll_card =
                        nth_next_card(choice_state.scroll_card, DECK_SIZE - 2) as _;
                } else {
                    unoffset -= 2;
                }
            } else if input.pressed_this_frame(Button::Down) {
                if unoffset / SCROLL_COLS_COUNT >= SCROLL_ROWS_COUNT - 1 {
                    choice_state.scroll_card = nth_next_card(choice_state.scroll_card, 2) as _;
                } else {
                    unoffset = nth_next_card(unoffset, 2);
                }
            }

            context.set_next_hot(unoffset + FIRST_CHECKBOX_ID);
        }
    }

    llog!(logger, context);
}

#[inline]
pub fn do_can_play_graph_choice(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
    speaker: &mut Speaker,
) {
    let logger = state.get_logger();
    if let Choice::OfCanPlayGraph(ref mut choice_state) = state.choice {
        match choice_state.layer {
            can_play::Layer::Card => can_play_graph_choose_card(
                framebuffer,
                &mut state.context,
                input,
                speaker,
                choice_state,
                logger,
            ),
            can_play::Layer::Edges => can_play_graph_choose_edges(
                framebuffer,
                &mut state.context,
                input,
                speaker,
                choice_state,
                logger,
            ),
        }
    } else {
        invariant_violation!(
            { state.choice = Choice::NoChoice },
            "`do_can_play_graph_choice` was called with the wrong choice type!"
        )
    }
}
