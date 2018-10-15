use common::*;
use game_state::{
    can_play, get_status_text, in_game, CardFlags, CardFlagsChoiceState, Choice, Chosen, GameState,
    Status, RULE_TYPES,
};
use platform_types::{log, Button, Input, Logger, Speaker};
use std::cmp::min;

//This is needed because we want to use it in scopes where other parts of the state are borrowwed.
macro_rules! cancel_rule_selection {
    ($state:expr) => {
        $state.choice = Choice::NoChoice;
        $state.status = Status::RuleSelection;
    };
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
        let text = b"choose a suit for the card";

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

pub fn choose_in_game_changes(state: &mut GameState) -> in_game::ChoiceState {
    match state.choice {
        Choice::NoChoice => {
            state.choice = Choice::OfInGameChanges(Default::default());
            Default::default()
        }
        Choice::Already(Chosen::InGameChanges(_)) => {
            if let Choice::Already(Chosen::InGameChanges(choice_state)) = state.choice.take() {
                choice_state
            } else {
                invariant_violation!(
                    { Default::default() },
                    "Somehow we're multi-threaded or somthing?!"
                )
            }
        }
        _ => Default::default(),
    }
}

#[inline]
pub fn do_in_game_changes_choice(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
    speaker: &mut Speaker,
) {
    let logger = state.get_logger();
    let mut chosen = None;
    let mut cancel = CancelRuleChoice::No;

    if let Choice::OfInGameChanges(ref mut choice_state) = state.choice {
        match choice_state.layer {
            in_game::Layer::Card => {
                let mut card_sub_choice = in_game::ChoiceStateAndRules {
                    choice_state,
                    rules: &state.rules,
                };

                cancel = do_card_sub_choice(
                    framebuffer,
                    &mut state.context,
                    input,
                    speaker,
                    &mut card_sub_choice,
                    logger,
                );
            }
            in_game::Layer::Changes => {
                in_game_changes_choose_changes(
                    framebuffer,
                    &mut state.context,
                    input,
                    speaker,
                    choice_state,
                    logger,
                );

                match choice_state.layer {
                    in_game::Layer::Changes => {}
                    in_game::Layer::Done => {
                        chosen = Some(Choice::Already(Chosen::InGameChanges(choice_state.clone())));
                    }
                    in_game::Layer::Card => {}
                }
            }
            in_game::Layer::Done => {
                framebuffer.center_half_window();
            }
        }
    } else {
        invariant_violation!(
            { state.choice = Choice::NoChoice },
            "`do_in_game_changes_choice` was called with the wrong choice type!"
        )
    }

    //This could be done in the above match with non-lexical lifetimes
    if let Some(chosen) = chosen {
        state.choice = chosen
    }

    //possibly this could be avoided with NLL too.
    if let CancelRuleChoice::Yes = cancel {
        cancel_rule_selection!(state);
    }
}

fn print_choice_header(framebuffer: &mut Framebuffer, text: &[u8]) -> u8 {
    let mut max_heading_y = heading_y(-1);

    let reflowed = bytes_reflow(text, NINE_SLICE_MAX_INTERIOR_WIDTH_IN_CHARS as _);

    for (i, line) in bytes_lines(&reflowed).enumerate() {
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

    max_heading_y
}

fn in_game_changes_choose_changes(
    framebuffer: &mut Framebuffer,
    context: &mut UIContext,
    input: Input,
    speaker: &mut Speaker,
    choice_state: &mut in_game::ChoiceState,
    logger: Logger,
) {
    framebuffer.full_window();

    let text = &[
        b"choose what will happen when ",
        get_card_string(choice_state.card).as_bytes(),
        b" is played.",
    ]
        .concat();

    let max_heading_y = print_choice_header(framebuffer, text);

    let w = SPRITE_SIZE * 5;
    let h = SPRITE_SIZE * 3;

    {
        let y = SPRITE_SIZE * 12;

        let spec = ButtonSpec {
            x: SCREEN_WIDTH as u8 - (w + SPRITE_SIZE),
            y,
            w,
            h,
            id: 3,
            text: "done".to_owned(),
        };

        if do_button(framebuffer, context, input, speaker, &spec) {
            choice_state.layer = in_game::Layer::Done;
        }
    }
    const FIRST_SCROLL_ID: UIId = 4;

    let min_scroll_y = max_heading_y + SPRITE_SIZE * 2;

    const SCROLL_ROW_COUNT: u8 = 7;

    let w = SPRITE_SIZE * 6;
    let x = SPRITE_SIZE;

    let id_range = FIRST_SCROLL_ID..FIRST_SCROLL_ID + SCROLL_ROW_COUNT;

    for id in id_range.clone() {
        let i = id - FIRST_SCROLL_ID;

        if let Some(change) = in_game::Change::all_values().get((choice_state.scroll + i) as usize)
        {
            let text = change.to_string();

            let spec = RowSpec {
                x,
                y: min_scroll_y + SPRITE_SIZE * i,
                w,
                id,
                text,
            };

            if do_pressable_row(framebuffer, context, input, speaker, &spec) {
                llog!(logger, change.to_string());
            }
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
            let scroll = &mut choice_state.scroll;
            *scroll = handle_scroll_movement(
                context,
                input,
                id_range,
                ModOffset {
                    modulus: nu8!(DECK_SIZE),
                    current: *scroll,
                    ..Default::default()
                },
            );
        }
    }
}

pub fn choose_can_play_graph(state: &mut GameState) -> Vec<can_play::Change> {
    match state.choice {
        Choice::NoChoice => {
            state.choice = Choice::OfCanPlayGraph(Default::default());
            Vec::new()
        }
        Choice::Already(Chosen::CanPlayGraph(_)) => {
            if let Choice::Already(Chosen::CanPlayGraph(changes)) = state.choice.take() {
                changes
            } else {
                invariant_violation!({ Vec::new() }, "Somehow we're multi-threaded or somthing?!")
            }
        }
        _ => Vec::new(),
    }
}

enum CancelRuleChoice {
    No,
    Yes,
}

fn do_card_sub_choice<C: CardSubChoice>(
    framebuffer: &mut Framebuffer,
    context: &mut UIContext,
    input: Input,
    speaker: &mut Speaker,
    choice_state: &mut C,
    logger: Logger,
) -> CancelRuleChoice {
    let mut output = CancelRuleChoice::No;

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
            //TODO add confirm dialog
            choice_state.reset();
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
            output = CancelRuleChoice::Yes;
        }
    }

    if choice_state.should_show_done_button() {
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
            choice_state.mark_done();
        }
    }

    {
        let x = SPRITE_SIZE * 11;
        let y = SPRITE_SIZE * 13;

        let current_highlighted_card =
            (*choice_state.borrow_mut() + context.hot.wrapping_sub(FIRST_SCROLL_ID)) % DECK_SIZE;

        let lines = choice_state.get_status_lines(current_highlighted_card);

        framebuffer.print_line(&lines[0], x, y, WHITE_INDEX);
        framebuffer.print_line(&lines[1], x, y + FONT_SIZE, WHITE_INDEX);
    }

    let w = SPRITE_SIZE * 10;
    let h = SPRITE_SIZE * 3;
    let x = SPRITE_SIZE;

    const FIRST_SCROLL_ID: UIId = 4;

    const SCROLL_BUTTON_COUNT: u8 = 4;
    let id_range = FIRST_SCROLL_ID..FIRST_SCROLL_ID + SCROLL_BUTTON_COUNT;
    for id in id_range.clone() {
        let i = id - FIRST_SCROLL_ID;
        let card = nth_next_card(*choice_state.borrow_mut(), i);
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
            *choice_state.borrow_mut() = card;
            choice_state.next_layer();
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
            let card = choice_state.borrow_mut();
            *card = handle_scroll_movement(
                context,
                input,
                id_range,
                ModOffset {
                    modulus: nu8!(DECK_SIZE),
                    current: *card,
                    ..Default::default()
                },
            );
        }
    }

    output
}

use std::ops::Range;

fn handle_scroll_movement(
    context: &mut UIContext,
    input: Input,
    Range { start, end }: Range<UIId>,
    mod_offset: ModOffset,
) -> u8 {
    let mut output = mod_offset.current;

    let column_count = mod_offset.offset;

    let mut unoffset = context.hot - start;
    let visible_rows = end - start;

    invariant_assert_eq!(
        (mod_offset.modulus / column_count) * column_count,
        mod_offset.modulus,
    );

    if input.pressed_this_frame(Button::Up) {
        if unoffset < column_count {
            output = previous_mod(mod_offset);
        } else {
            unoffset -= column_count;
        }
    } else if input.pressed_this_frame(Button::Down) {
        if unoffset / column_count >= visible_rows - 1 {
            output = next_mod(mod_offset);
        } else {
            unoffset = next_mod(ModOffset {
                current: unoffset,
                ..mod_offset
            });
        }
    }

    context.set_next_hot(unoffset + start);

    output
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
    _logger: Logger,
) {
    framebuffer.full_window();

    let text = &[
        b"choose the cards the ",
        get_card_string(choice_state.card).as_bytes(),
        b" can be played on.",
    ]
        .concat();

    let max_heading_y = print_choice_header(framebuffer, text);

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
            choice_state
                .changes
                .push(can_play::Change::new(choice_state.edges, choice_state.card));
            choice_state.layer = Default::default();
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
            choice_state.layer = Default::default();
        }
    }

    const FIRST_CHECKBOX_ID: UIId = 3;

    do_scrolling_card_checkbox(
        framebuffer,
        context,
        input,
        speaker,
        &mut choice_state.scroll_card,
        &mut choice_state.edges,
        FIRST_CHECKBOX_ID,
        max_heading_y,
    );
}

fn do_scrolling_card_checkbox(
    framebuffer: &mut Framebuffer,
    context: &mut UIContext,
    input: Input,
    speaker: &mut Speaker,
    scroll_card: &mut Card,
    card_flags: &mut CardFlags,
    first_checkbox_id: UIId,
    max_heading_y: u8,
) {
    const SCROLL_ROWS_COUNT: u8 = 10;
    const SCROLL_COLS_COUNT: u8 = 2;

    invariant_assert_eq!(
        (DECK_SIZE / SCROLL_COLS_COUNT) * SCROLL_COLS_COUNT,
        DECK_SIZE,
    );
    let current_scroll_card = nth_next_card(*scroll_card, 0);

    for y in 0..SCROLL_ROWS_COUNT {
        for x in 0..SCROLL_COLS_COUNT {
            let i = x + y * SCROLL_COLS_COUNT;
            let id = i as UIId + first_checkbox_id;

            let card = nth_next_card(current_scroll_card, i);
            let text = get_suit_rank_pair(card);

            let spec = CheckboxSpec {
                x: SPRITE_SIZE
                    + (SPRITE_SIZE * 2 + RANK_SUIT_PAIR_WITH_IN_CHARS * FONT_ADVANCE) * x,
                y: max_heading_y + SPRITE_SIZE * (y + 1) as u8 + (SPRITE_SIZE as u8 / 2),
                id,
                text,
                checked: card_flags.has_card(card),
            };

            if do_checkbox(framebuffer, context, input, speaker, &spec) {
                card_flags.toggle_card(card);
            }
        }
    }

    if context.hot < first_checkbox_id as _ {
        if context.hot == 0 {
            context.set_next_hot(1);
        } else if input.pressed_this_frame(Button::Up) {
            let next = dice_mod(context.hot - 1, first_checkbox_id - 1);
            context.set_next_hot(next);
        } else if input.pressed_this_frame(Button::Down) {
            let next = dice_mod(context.hot + 1, first_checkbox_id - 1);
            context.set_next_hot(next);
        } else if input.pressed_this_frame(Button::Right) {
            if context.hot == 1 {
                context.set_next_hot(first_checkbox_id);
            } else {
                context.set_next_hot(first_checkbox_id + 3 * SCROLL_COLS_COUNT);
            }
        } else if input.pressed_this_frame(Button::Left) {
            if context.hot == 1 {
                context.set_next_hot(first_checkbox_id + 1);
            } else {
                context.set_next_hot(first_checkbox_id + 3 * SCROLL_COLS_COUNT + 1);
            }
        }
    } else {
        if input.pressed_this_frame(Button::Left) {
            if context.hot & 1 == 1 {
                if context.hot > first_checkbox_id + 3 * SCROLL_COLS_COUNT {
                    context.set_next_hot(first_checkbox_id - 1);
                } else {
                    context.set_next_hot(first_checkbox_id - 2);
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
                if context.hot > first_checkbox_id + 3 * SCROLL_COLS_COUNT {
                    context.set_next_hot(first_checkbox_id - 1);
                } else {
                    context.set_next_hot(first_checkbox_id - 2);
                }
            }
        } else {
            *scroll_card = handle_scroll_movement(
                context,
                input,
                first_checkbox_id..first_checkbox_id + SCROLL_ROWS_COUNT,
                ModOffset {
                    modulus: nu8!(DECK_SIZE),
                    current: *scroll_card,
                    offset: SCROLL_COLS_COUNT,
                },
            );
        }
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
    let mut chosen = None;
    let mut cancel = CancelRuleChoice::No;

    if let Choice::OfCanPlayGraph(ref mut choice_state) = state.choice {
        match choice_state.layer {
            can_play::Layer::Card => {
                cancel = do_card_sub_choice(
                    framebuffer,
                    &mut state.context,
                    input,
                    speaker,
                    choice_state,
                    logger,
                );

                match choice_state.layer {
                    can_play::Layer::Edges => {
                        let can_play_graph = &state.rules.can_play_graph;

                        choice_state.edges = choice_state
                            .changes
                            .iter()
                            .rev()
                            .find(|c| c.card() == choice_state.card)
                            .map(|c| c.edges())
                            .unwrap_or_else(|| can_play_graph.get_edges(choice_state.card));
                    }
                    can_play::Layer::Done => {
                        chosen = Some(Choice::Already(Chosen::CanPlayGraph(
                            choice_state.changes.clone(),
                        )));
                    }
                    can_play::Layer::Card => {}
                }
            }
            can_play::Layer::Edges => can_play_graph_choose_edges(
                framebuffer,
                &mut state.context,
                input,
                speaker,
                choice_state,
                logger,
            ),
            can_play::Layer::Done => {
                framebuffer.center_half_window();
            }
        }
    } else {
        invariant_violation!(
            { state.choice = Choice::NoChoice },
            "`do_can_play_graph_choice` was called with the wrong choice type!"
        )
    }

    //This could be done in the above match with non-lexical lifetimes
    if let Some(chosen) = chosen {
        state.choice = chosen;
    }

    //possibly this could be avoided with NLL too.
    if let CancelRuleChoice::Yes = cancel {
        cancel_rule_selection!(state);
    }
}

pub fn choose_rule(state: &mut GameState) -> Option<Status> {
    match state.choice {
        Choice::NoChoice => {
            state.choice = Choice::OfStatus;
            None
        }
        Choice::Already(Chosen::Status(status)) => {
            state.choice = Choice::NoChoice;
            Some(status)
        }
        _ => None,
    }
}

#[inline]
pub fn do_card_flags_choice(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
    speaker: &mut Speaker,
) {
    let mut chosen = None;
    let mut cancel = CancelRuleChoice::No;

    if let Choice::OfCardFlags(ref mut card_flags_state) = state.choice {
        let context = &mut state.context;

        framebuffer.full_window();
        {
            let text = b"select which cards are wild";

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
                text: "ok".to_owned(),
            };

            if do_button(framebuffer, context, input, speaker, &spec) {
                chosen = Some(card_flags_state.flags);
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
                cancel = CancelRuleChoice::Yes;
            }
        }

        const FIRST_CHECKBOX_ID: UIId = 3;

        do_scrolling_card_checkbox(
            framebuffer,
            context,
            input,
            speaker,
            &mut card_flags_state.card,
            &mut card_flags_state.flags,
            FIRST_CHECKBOX_ID,
            SPRITE_SIZE * 3,
        );
    } else {
        invariant_violation!(
            { state.choice = Choice::NoChoice },
            "`do_card_flags_choice` was called with the wrong choice type!"
        )
    }

    if let Some(chosen) = chosen {
        state.choice = Choice::Already(Chosen::CardFlags(chosen));
    }

    if let CancelRuleChoice::Yes = cancel {
        cancel_rule_selection!(state);
    }
}

pub fn choose_wild_flags(state: &mut GameState) -> Option<CardFlags> {
    match state.choice {
        Choice::NoChoice => {
            state.choice = Choice::OfCardFlags(CardFlagsChoiceState {
                flags: state.rules.wild,
                card: Default::default(),
            });
            None
        }
        Choice::Already(Chosen::CardFlags(flags)) => {
            state.choice = Choice::NoChoice;
            Some(flags)
        }
        _ => None,
    }
}

#[inline]
pub fn do_status_choice(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
    speaker: &mut Speaker,
) {
    framebuffer.full_window();
    {
        let text = b"choose a type of rule";

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

    for (i, status) in RULE_TYPES.iter().cloned().enumerate() {
        let i = (i + 1) as u8;

        let mut text = get_status_text(status).to_string();

        let spec = ButtonSpec {
            x,
            y: h * i,
            w,
            h,
            id: i,
            text,
        };

        if do_button(framebuffer, &mut state.context, input, speaker, &spec) {
            state.choice = Choice::Already(Chosen::Status(status));
        }
    }

    #[allow(non_snake_case)]
    let MAX_ID = RULE_TYPES.len() as UIId;

    if state.context.hot == 0 || state.context.hot > MAX_ID {
        state.context.set_next_hot(1);
    } else if input.pressed_this_frame(Button::Up) {
        let next = dice_mod(state.context.hot - 1, MAX_ID);
        state.context.set_next_hot(next);
    } else if input.pressed_this_frame(Button::Down) {
        let next = dice_mod(state.context.hot + 1, MAX_ID);
        state.context.set_next_hot(next);
    }
}

pub fn do_choices(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
    speaker: &mut Speaker,
) {
    match state.choice {
        Choice::OfInGameChanges(_) => do_in_game_changes_choice(framebuffer, state, input, speaker),
        Choice::OfCanPlayGraph(_) => do_can_play_graph_choice(framebuffer, state, input, speaker),
        Choice::OfCardFlags(_) => do_card_flags_choice(framebuffer, state, input, speaker),
        Choice::OfStatus => do_status_choice(framebuffer, state, input, speaker),
        Choice::OfSuit => do_suit_choice(framebuffer, state, input, speaker),
        Choice::OfBool => do_bool_choice(framebuffer, state, input, speaker),
        Choice::OfUnit => do_unit_choice(framebuffer, state, input, speaker),
        Choice::NoChoice => {
            //TODO should we unify Status and Choice to avoid this code?
            if let Status::InGame = state.status {
            } else {
                framebuffer.full_window();
            }
        }
        Choice::Already(_) => {}
    }
}
