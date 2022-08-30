use common::*;
use game_state::{
    can_play, get_status_text, in_game, CardFlagsChoiceState, Choice, Chosen, GameState, Status,
    RULE_TYPES,
};
use platform_types::{Button, Input, Speaker};
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

enum UnitChoiceScreen {
    Rules,
    Winners,
}

#[inline]
fn do_unit_choice(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
    speaker: &mut Speaker,
    screen: UnitChoiceScreen,
) {
    framebuffer.full_window();

    match screen {
        UnitChoiceScreen::Winners => {
            {
                let winner_text = reflow(
                    &state.in_game.get_winner_text(),
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
        }
        UnitChoiceScreen::Rules => {
            print_choice_header(
                framebuffer,
                b"use z, x, enter, shift and the arrow keys to play. press shift to show this menu again. press enter to show the event log. z and x quickly scroll through the log. use arrows and z to navigate menus and play cards. press x to draw a card. ready to play?",
            );
        }
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
        match screen {
            UnitChoiceScreen::Winners => state.choice = Choice::Already(Chosen::Unit(())),
            UnitChoiceScreen::Rules => state.show_rules = false,
        }
    }

    if state.context.hot != 1 {
        state.context.set_next_hot(1);
    }
}

pub fn show_rules_screen(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
    speaker: &mut Speaker,
) {
    do_unit_choice(framebuffer, state, input, speaker, UnitChoiceScreen::Rules)
}

#[inline]
fn do_bool_choice(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
    speaker: &mut Speaker,
) {
    framebuffer.full_window();

    {
        let question = b"close this window?";

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
    } else if input.pressed_this_frame(Button::LEFT) || input.pressed_this_frame(Button::RIGHT) {
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
    if input.gamepad.contains(Button::B) {
        return;
    }

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

    for (i, suit) in suits::ALL.iter().cloned().enumerate() {
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
    } else if input.pressed_this_frame(Button::UP) {
        let next = dice_mod(state.context.hot - 1, 4);
        state.context.set_next_hot(next);
    } else if input.pressed_this_frame(Button::DOWN) {
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
            state.choice = Choice::OfInGameChanges(d!());
            d!()
        }
        Choice::Already(Chosen::InGameChanges(_)) => {
            if let Choice::Already(Chosen::InGameChanges(choice_state)) = state.choice.take() {
                choice_state
            } else {
                invariant_violation!({ d!() }, "Somehow we're multi-threaded or somthing?!")
            }
        }
        _ => d!(),
    }
}

#[inline]
pub fn do_in_game_changes_choice(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
    speaker: &mut Speaker,
) {
    if let Choice::OfInGameChanges(ref mut choice_state) = state.choice {
        match choice_state.layer {
            in_game::Layer::Card => {
                let mut sub_choice = in_game::ChoiceStateAndRules {
                    choice_state,
                    rules: &state.rules,
                };

                let cancel = do_card_flags_sub_choice(
                    framebuffer,
                    &mut state.context,
                    input,
                    speaker,
                    &mut sub_choice,
                    b"choose a set of cards.",
                );

                if let CancelRuleChoice::Yes = cancel {
                    cancel_rule_selection!(state);
                } else {
                    match choice_state.layer {
                        in_game::Layer::Changes => {
                            let card_changes = state
                                .rules
                                .when_played
                                .get_card_flags_changes(choice_state.card_set);
                            choice_state.changes.clear();
                            for change in card_changes {
                                choice_state.changes.push(change);
                            }
                        }
                        in_game::Layer::Done | in_game::Layer::Card => {}
                    }
                }
            }
            in_game::Layer::Changes => {
                in_game_changes_choose_changes(
                    framebuffer,
                    &mut state.context,
                    input,
                    speaker,
                    choice_state,
                );

                match choice_state.layer {
                    in_game::Layer::Changes => {}
                    in_game::Layer::Done => {
                        state.choice = Choice::Already(Chosen::InGameChanges(choice_state.clone()));
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
}

pub fn print_choice_header(framebuffer: &mut Framebuffer, text: &[u8]) -> u8 {
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

fn set_from_change(s: &mut Vec<u8>, change: &in_game::Change) {
    let description = change.to_string();
    s.clear();
    for &b in description.as_bytes() {
        s.push(b);
    }
    bytes_reflow_in_place(s, 18);
}

fn in_game_changes_choose_changes(
    framebuffer: &mut Framebuffer,
    context: &mut UIContext,
    input: Input,
    speaker: &mut Speaker,
    choice_state: &mut in_game::ChoiceState,
) {
    framebuffer.full_window();

    let size = choice_state.card_set.size();
    let verb_to_be: &[u8] = if size == 1 || size == DECK_SIZE as u32 {
        b" is "
    } else {
        b" are "
    };

    let card_set_string = format!("{:64}", choice_state.card_set);

    let text = bytes_concat!(
        b"choose what will happen when ",
        card_set_string.as_bytes(),
        verb_to_be,
        b"played.",
    );

    let max_heading_y = print_choice_header(framebuffer, text);

    {
        let w = SPRITE_SIZE * 5;
        let spec = ButtonSpec {
            x: SCREEN_WIDTH as u8 - (w + SPRITE_SIZE),
            y: SPRITE_SIZE * 12,
            w,
            h: SPRITE_SIZE * 3,
            id: 1,
            text: "done".to_owned(),
        };

        if do_button(framebuffer, context, input, speaker, &spec) {
            choice_state.layer = in_game::Layer::Done;
        }
    }
    const FIRST_SCROLL_START_ID: UIId = 2;

    const SCROLL_ROW_COUNT: u8 = 7;

    const SECOND_SCROLL_START_ID: UIId = FIRST_SCROLL_START_ID + SCROLL_ROW_COUNT;

    let min_scroll_y = max_heading_y + SPRITE_SIZE * 2;
    let max_scroll_y = min_scroll_y + SPRITE_SIZE * SCROLL_ROW_COUNT;

    let left_id_range = FIRST_SCROLL_START_ID..FIRST_SCROLL_START_ID + SCROLL_ROW_COUNT;

    //The + 1 is to leave a gap to mark the loop point, and keep the modulus non-zero.
    let left_modulus = in_game::ALL_CHANGES.len() + 1;
    {
        let mut addition = None;
        for id in left_id_range.clone() {
            let i = id - FIRST_SCROLL_START_ID;

            let x = SPRITE_SIZE;
            let y = min_scroll_y + SPRITE_SIZE * i - SPRITE_SIZE / 2;

            let index = (choice_state.left_scroll as usize + i as usize) % left_modulus as usize;
            if let Some(change) = in_game::ALL_CHANGES.get(index) {
                let label = change.row_label();

                if id == context.hot {
                    set_from_change(&mut choice_state.description, change);
                }

                let spec = RowSpec { x, y, id, label };

                if do_pressable_row(framebuffer, context, input, speaker, &spec) {
                    addition = Some(index);
                }
            } else {
                let spec = RowSpec {
                    x,
                    y,
                    id,
                    label: Default::default(),
                };

                do_pressable_row(framebuffer, context, input, speaker, &spec);
            }
        }

        if let Some(index) = addition {
            if let Some(&change) = in_game::ALL_CHANGES.get(index) {
                let i = min(
                    choice_state.right_scroll as usize + choice_state.marker_y as usize,
                    choice_state.changes.len(),
                );
                choice_state.changes.insert(i, change);
            }
        }
    }

    let right_id_range = SECOND_SCROLL_START_ID..SECOND_SCROLL_START_ID + SCROLL_ROW_COUNT;
    //leave a gap to mark the loop point, and keep the modulus non-zero.
    let right_modulus = choice_state.changes.len() + 1;
    {
        let mut removal = None;
        let x = SPRITE_SIZE + ROW_WIDTH + SPRITE_SIZE;
        for id in right_id_range.clone() {
            let i = id - SECOND_SCROLL_START_ID;

            let y = min_scroll_y + SPRITE_SIZE * i - SPRITE_SIZE / 2;

            let index = (choice_state.right_scroll as usize + i as usize) % right_modulus;
            if let Some(change) = choice_state.changes.get(index) {
                let label = change.row_label();

                if id == context.hot {
                    set_from_change(&mut choice_state.description, change);
                }

                let spec = RowSpec { x, y, id, label };

                if do_pressable_row(framebuffer, context, input, speaker, &spec) {
                    removal = Some(index);
                }
            } else {
                let spec = RowSpec {
                    x,
                    y,
                    id,
                    label: Default::default(),
                };

                do_pressable_row(framebuffer, context, input, speaker, &spec);
            }
        }

        let y = min_scroll_y + SPRITE_SIZE * choice_state.marker_y - SPRITE_SIZE;

        framebuffer.row_marker(x, y, ROW_WIDTH);

        if let Some(index) = removal {
            choice_state.changes.remove(index);
        }
    }

    let mut y = max_scroll_y;
    let max_y = SCREEN_HEIGHT as u8 - SPRITE_SIZE;
    //TODO allow scrolling?
    for line in bytes_lines(&choice_state.description) {
        framebuffer.print_line(line, SPRITE_SIZE, y, WHITE_INDEX);

        y += FONT_SIZE;
        if y >= max_y {
            break;
        }
    }

    if context.hot < FIRST_SCROLL_START_ID as _ {
        if context.hot == 0 {
            context.set_next_hot(1);
        } else if input.pressed_this_frame(Button::UP) {
            context.set_next_hot(SECOND_SCROLL_START_ID + SCROLL_ROW_COUNT - 1);
        } else if input.pressed_this_frame(Button::DOWN) {
            context.set_next_hot(SECOND_SCROLL_START_ID);
        } else if input.pressed_this_frame(Button::RIGHT) || input.pressed_this_frame(Button::LEFT)
        {
            context.set_next_hot(FIRST_SCROLL_START_ID + SCROLL_ROW_COUNT - 1);
        }
    } else if input.pressed_this_frame(Button::RIGHT) {
        let next = if context.hot < FIRST_SCROLL_START_ID + SCROLL_ROW_COUNT {
            context.hot + SCROLL_ROW_COUNT
        } else {
            1
        };
        context.set_next_hot(next);
    } else if input.pressed_this_frame(Button::LEFT) {
        let next = if context.hot < FIRST_SCROLL_START_ID + SCROLL_ROW_COUNT {
            1
        } else {
            context.hot - SCROLL_ROW_COUNT
        };
        context.set_next_hot(next);
    } else if context.hot < FIRST_SCROLL_START_ID + SCROLL_ROW_COUNT {
        let left_scroll = &mut choice_state.left_scroll;
        *left_scroll = handle_scroll_movement(
            context,
            input,
            left_id_range,
            ModOffset {
                modulus: left_modulus,
                current: *left_scroll,
                ..d!()
            },
        );
    } else {
        if input.pressed_this_frame(Button::UP) {
            choice_state.marker_y = choice_state.marker_y.saturating_sub(1);
        } else if input.pressed_this_frame(Button::DOWN) {
            choice_state.marker_y = min(choice_state.marker_y + 1, SCROLL_ROW_COUNT);
        }

        let right_scroll = &mut choice_state.right_scroll;
        *right_scroll = handle_scroll_movement(
            context,
            input,
            right_id_range.clone(),
            ModOffset {
                modulus: right_modulus,
                current: *right_scroll,
                ..d!()
            },
        );
    }

    if outside_range(&right_id_range, context.hot)
        && inside_range(&right_id_range, context.next_hot)
    {
        choice_state.marker_y = context.next_hot - right_id_range.start + 1;
    }
}

fn inside_range<Idx>(range: &Range<Idx>, x: Idx) -> bool
where
    Idx: PartialOrd<Idx>,
{
    x >= range.start && x < range.end
}

fn outside_range<Idx>(range: &Range<Idx>, x: Idx) -> bool
where
    Idx: PartialOrd<Idx>,
{
    x < range.start || x >= range.end
}

pub fn choose_can_play_graph(state: &mut GameState) -> Vec<can_play::Change> {
    match state.choice {
        Choice::NoChoice => {
            state.choice = Choice::OfCanPlayGraph(d!());
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

fn do_card_flags_sub_choice<C: CardFlagsSubChoice>(
    framebuffer: &mut Framebuffer,
    context: &mut UIContext,
    input: Input,
    speaker: &mut Speaker,
    choice_state: &mut C,
    text: &[u8],
) -> CancelRuleChoice {
    let mut output = CancelRuleChoice::No;

    framebuffer.full_window();

    let max_heading_y = print_choice_header(framebuffer, text);

    let w = SPRITE_SIZE * 5;
    let h = SPRITE_SIZE * 3;
    let x = SCREEN_WIDTH as u8 - (w + SPRITE_SIZE);

    let upward_offset = SPRITE_SIZE * 3 / 4;

    {
        let y = SPRITE_SIZE * 4 - upward_offset;

        let spec = ButtonSpec {
            x,
            y,
            w,
            h,
            id: 1,
            text: "reset".to_owned(),
        };

        if do_button(framebuffer, context, input, speaker, &spec) {
            choice_state.reset();
        }
    }

    {
        let y = SPRITE_SIZE * 7 - upward_offset * 2;

        let spec = ButtonSpec {
            x,
            y,
            w,
            h,
            id: 2,
            text: "ok".to_owned(),
        };

        if do_button(framebuffer, context, input, speaker, &spec) {
            choice_state.mark_done();
        }
    }

    {
        let y = SPRITE_SIZE * 10 - upward_offset * 3;

        let spec = ButtonSpec {
            x,
            y,
            w,
            h,
            id: 3,
            text: "cancel".to_owned(),
        };

        if do_button(framebuffer, context, input, speaker, &spec) {
            output = CancelRuleChoice::Yes;
        }
    }

    let (scroll_card, flags) = choice_state.borrow_pair_mut();

    {
        let y = SPRITE_SIZE * 13 - upward_offset * 4;

        let spec = ButtonSpec {
            x,
            y,
            w,
            h,
            id: 4,
            text: "invert".to_owned(),
        };

        if do_button(framebuffer, context, input, speaker, &spec) {
            flags.invert();
        }
    }

    const FIRST_CHECKBOX_ID: UIId = 5;

    do_scrolling_card_checkbox(
        (
            framebuffer,
            context,
            speaker,
            scroll_card,
            flags,
        ),
        input,
        FIRST_CHECKBOX_ID,
        max_heading_y,
    );

    {
        let x = SPRITE_SIZE * 11;
        let y = SPRITE_SIZE * 13;

        let lines = choice_state.get_status_lines();

        framebuffer.print_line(&lines[0], x, y, WHITE_INDEX);
        framebuffer.print_line(&lines[1], x, y + FONT_SIZE, WHITE_INDEX);
    }

    output
}

fn do_card_sub_choice<C: CardSubChoice>(
    framebuffer: &mut Framebuffer,
    context: &mut UIContext,
    input: Input,
    speaker: &mut Speaker,
    choice_state: &mut C,
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
        } else if input.pressed_this_frame(Button::UP) {
            let next = dice_mod(context.hot - 1, 3);
            context.set_next_hot(next);
        } else if input.pressed_this_frame(Button::DOWN) {
            let next = dice_mod(context.hot + 1, 3);
            context.set_next_hot(next);
        } else if input.pressed_this_frame(Button::RIGHT) || input.pressed_this_frame(Button::LEFT)
        {
            let next = (FIRST_SCROLL_ID - 1) + context.hot;
            context.set_next_hot(next);
        }
    } else if input.pressed_this_frame(Button::RIGHT) || input.pressed_this_frame(Button::LEFT) {
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
                modulus: DECK_SIZE,
                current: *card,
                ..d!()
            },
        );
    }

    output
}

use std::fmt;
use std::ops::{Add, Range, Rem, Sub};

fn handle_scroll_movement<T>(
    context: &mut UIContext,
    input: Input,
    Range { start, end }: Range<UIId>,
    mod_offset: ModOffset<T>,
) -> T
where
    T: fmt::Debug
        + From<u8>
        + Add<T, Output = T>
        + Sub<T, Output = T>
        + Rem<T, Output = T>
        + Ord
        + Copy,
{
    let mut output = mod_offset.current;

    let column_count = mod_offset.offset;

    let mut unoffset = context.hot - start;
    let visible_rows = end - start;
    if visible_rows == 0 {
        invariant_violation!(
            {
                return output;
            },
            "`visible_rows == 0`"
        );
    }

    invariant_assert!(column_count != 0);
    invariant_assert_eq!(mod_offset.modulus % column_count.into(), 0u8.into());

    if input.pressed_this_frame(Button::UP) {
        if unoffset < column_count {
            output = previous_mod(mod_offset);
        } else {
            unoffset -= column_count;
        }
    } else if input.pressed_this_frame(Button::DOWN) {
        if unoffset / column_count >= (visible_rows / column_count) - 1 {
            output = next_mod(mod_offset);
        } else {
            let unoffset_mod: ModOffset<UIId> = ModOffset {
                modulus: visible_rows,
                current: unoffset,
                offset: mod_offset.offset,
            };
            unoffset = next_mod(unoffset_mod);
        }
    }

    context.set_next_hot(unoffset + start);

    output
}

fn heading_y(i: i8) -> u8 {
    (SPRITE_SIZE as i8 * 2 + FONT_SIZE as i8 * i) as u8
}

fn do_scrolling_card_checkbox(
    (
        framebuffer,
        context,
        speaker,
        scroll_card,
        card_flags,
    ) : (
        &mut Framebuffer,
        &mut UIContext,
        &mut Speaker,
        &mut Card,
        &mut CardFlags,
    ),
    input: Input,
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
        } else if input.pressed_this_frame(Button::UP) {
            let next = dice_mod(context.hot - 1, first_checkbox_id - 1);
            context.set_next_hot(next);
        } else if input.pressed_this_frame(Button::DOWN) {
            let next = dice_mod(context.hot + 1, first_checkbox_id - 1);
            context.set_next_hot(next);
        } else if input.pressed_this_frame(Button::RIGHT) {
            if context.hot == 1 {
                context.set_next_hot(first_checkbox_id);
            } else {
                context.set_next_hot(first_checkbox_id + 3 * SCROLL_COLS_COUNT);
            }
        } else if input.pressed_this_frame(Button::LEFT) {
            if context.hot == 1 {
                context.set_next_hot(first_checkbox_id + 1);
            } else {
                context.set_next_hot(first_checkbox_id + 3 * SCROLL_COLS_COUNT + 1);
            }
        }
    } else if input.pressed_this_frame(Button::LEFT) {
        if context.hot & 1 == first_checkbox_id & 1 {
            if context.hot > first_checkbox_id + 3 * SCROLL_COLS_COUNT {
                context.set_next_hot(first_checkbox_id - 1);
            } else {
                context.set_next_hot(first_checkbox_id - 2);
            }
        } else {
            let next = context.hot - 1;
            context.set_next_hot(next);
        }
    } else if input.pressed_this_frame(Button::RIGHT) {
        context.set_next_hot(if context.hot & 1 == first_checkbox_id & 1 {
            context.hot + 1
        } else if context.hot > first_checkbox_id + 3 * SCROLL_COLS_COUNT {
            first_checkbox_id - 1
        } else {
            first_checkbox_id - 2
        });
    } else {
        *scroll_card = handle_scroll_movement(
            context,
            input,
            first_checkbox_id..first_checkbox_id + (SCROLL_ROWS_COUNT * SCROLL_COLS_COUNT),
            ModOffset {
                modulus: DECK_SIZE,
                current: *scroll_card,
                offset: SCROLL_COLS_COUNT,
            },
        );
    }
}

#[inline]
pub fn do_can_play_graph_choice(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
    speaker: &mut Speaker,
) {
    if let Choice::OfCanPlayGraph(ref mut choice_state) = state.choice {
        let mut cancel = CancelRuleChoice::No;

        match choice_state.layer {
            can_play::Layer::Card => {
                cancel = do_card_sub_choice(
                    framebuffer,
                    &mut state.context,
                    input,
                    speaker,
                    choice_state,
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
                        choice_state.reset_edges = choice_state.edges;
                    }
                    can_play::Layer::Done => {
                        state.choice =
                            Choice::Already(Chosen::CanPlayGraph(choice_state.changes.clone()));
                    }
                    can_play::Layer::Card => {}
                }
            }
            can_play::Layer::Edges => {
                let text = bytes_concat!(
                    b"choose the cards the ",
                    get_card_string(choice_state.card).as_bytes(),
                    b" can be played on.",
                );

                cancel = do_card_flags_sub_choice(
                    framebuffer,
                    &mut state.context,
                    input,
                    speaker,
                    choice_state,
                    text,
                );

                match choice_state.layer {
                    can_play::Layer::Done => {}
                    can_play::Layer::Edges => {}
                    can_play::Layer::Card => {
                        choice_state
                            .changes
                            .push(can_play::Change::new(choice_state.edges, choice_state.card));
                    }
                }
            }
            can_play::Layer::Done => {
                framebuffer.center_half_window();
            }
        }

        if let CancelRuleChoice::Yes = cancel {
            cancel_rule_selection!(state);
        }
    } else {
        invariant_violation!(
            { state.choice = Choice::NoChoice },
            "`do_can_play_graph_choice` was called with the wrong choice type!"
        )
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
    if let Choice::OfCardFlags(ref mut card_flags_state) = state.choice {
        let cancel = do_card_flags_sub_choice(
            framebuffer,
            &mut state.context,
            input,
            speaker,
            card_flags_state,
            b"select which cards are wild",
        );

        if let CancelRuleChoice::Yes = cancel {
            cancel_rule_selection!(state);
        } else if let Some(c) = card_flags_state.get_chosen() {
            state.choice = Choice::Already(Chosen::CardFlags(c));
        }
    } else {
        invariant_violation!(
            { state.choice = Choice::NoChoice },
            "`do_card_flags_choice` was called with the wrong choice type!"
        )
    }
}

pub fn choose_wild_flags(state: &mut GameState) -> Option<CardFlags> {
    match state.choice {
        Choice::NoChoice => {
            state.choice = Choice::OfCardFlags(CardFlagsChoiceState::new(state.rules.wild));
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

        let text = get_status_text(status).to_string();

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
    } else if input.pressed_this_frame(Button::UP) {
        let next = dice_mod(state.context.hot - 1, MAX_ID);
        state.context.set_next_hot(next);
    } else if input.pressed_this_frame(Button::DOWN) {
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
        Choice::OfUnit => do_unit_choice(
            framebuffer,
            state,
            input,
            speaker,
            UnitChoiceScreen::Winners,
        ),
        Choice::NoChoice => {
            if let Status::InGame = state.status {
            } else {
                framebuffer.full_window();
            }
        }
        Choice::Already(_) => {}
    }
}
