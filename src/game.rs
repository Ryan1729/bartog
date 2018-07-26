use common::*;

use std::cmp::min;

enum Face {
    Up,
    Down,
}

fn draw_hand_ltr(
    framebuffer: &mut Framebuffer,
    hand: &Hand,
    (left_edge, right_edge): (u8, u8),
    y: u8,
    face: Face,
) {
    let len = hand.len() as u8;
    if len == 0 {
        return;
    }

    let full_width = right_edge.saturating_sub(left_edge);
    let usable_width = full_width.saturating_sub(card::WIDTH);
    let offset = min(usable_width / len, card::WIDTH);
    let mut x = left_edge;

    match face {
        Face::Up => {
            for &card in hand.iter() {
                framebuffer.draw_card(card, x, y);

                x += offset;
            }
        }
        Face::Down => {
            for &card in hand.iter() {
                framebuffer.draw_card_back(x, y);

                x += offset;
            }
        }
    }
}

fn draw_hand_ttb(
    framebuffer: &mut Framebuffer,
    hand: &Hand,
    (top_edge, bottom_edge): (u8, u8),
    x: u8,
    face: Face,
) {
    let len = hand.len() as u8;
    if len == 0 {
        return;
    }

    let full_height = bottom_edge.saturating_sub(top_edge);
    let usable_height = full_height.saturating_sub(card::HEIGHT);
    let offset = min(usable_height / len, card::HEIGHT);
    let mut y = top_edge;

    match face {
        Face::Up => {
            for &card in hand.iter() {
                framebuffer.draw_card(card, x, y);

                y += offset;
            }
        }
        Face::Down => {
            for &card in hand.iter() {
                framebuffer.draw_card_back(x, y);

                y += offset;
            }
        }
    }
}

fn draw_hand_with_cursor(
    framebuffer: &mut Framebuffer,
    hand: &Hand,
    (left_edge, right_edge): (u8, u8),
    y: u8,
    index: usize,
) {
    let len = hand.len() as u8;
    if len == 0 {
        return;
    }

    let full_width = right_edge.saturating_sub(left_edge);
    let usable_width = full_width.saturating_sub(card::WIDTH);
    let offset = min(usable_width / len, card::WIDTH);

    let mut x = left_edge;
    let mut selected_card_and_offset = None;
    for (i, &card) in hand.iter().enumerate() {
        if i == index {
            selected_card_and_offset = Some((card, x));
            x += offset;

            continue;
        }
        framebuffer.draw_card(card, x, y);

        x += offset;
    }

    if let Some((card, cursor_offset)) = selected_card_and_offset {
        framebuffer.draw_highlighted_card(card, cursor_offset, y);
    }
}

fn update(state: &mut GameState, input: Input) {
    if input.pressed_this_frame(Button::Right) {
        if (state.hand_index as usize) < state.hand.len() - 1 {
            state.hand_index = state.hand_index.saturating_add(1);
        }
    } else if input.pressed_this_frame(Button::Left) {
        state.hand_index = state.hand_index.saturating_sub(1);
    } else if input.pressed_this_frame(Button::A) {
        state
            .hand
            .discard_to(&mut state.discard, state.hand_index as usize);
    } else if input.pressed_this_frame(Button::Down) {
        state.hand.draw_from(&mut state.deck);
    } else if input.pressed_this_frame(Button::Up) {
        state
            .hand
            .discard_randomly_to(&mut state.deck, &mut state.rng);
    }
}

#[inline]
pub fn update_and_render(framebuffer: &mut Framebuffer, state: &mut GameState, input: Input) {
    update(state, input);

    framebuffer.clearTo(GREEN);

    draw_hand_ttb(
        framebuffer,
        &state.cpu_hands[0],
        LEFT_AND_RIGHT_HAND_EDGES,
        LEFT_CPU_HAND_X,
        Face::Down,
    );

    draw_hand_ltr(
        framebuffer,
        &state.cpu_hands[1],
        TOP_AND_BOTTOM_HAND_EDGES,
        MIDDLE_CPU_HAND_HEIGHT,
        Face::Down,
    );

    draw_hand_ttb(
        framebuffer,
        &state.cpu_hands[2],
        LEFT_AND_RIGHT_HAND_EDGES,
        RIGHT_CPU_HAND_X,
        Face::Down,
    );

    state
        .deck
        .iter()
        .last()
        .map(|&c| framebuffer.draw_card(c, 40, 32));
    state
        .discard
        .iter()
        .last()
        .map(|&c| framebuffer.draw_card(c, 40 + card::WIDTH + card::WIDTH / 2, 32));

    draw_hand_with_cursor(
        framebuffer,
        &state.hand,
        TOP_AND_BOTTOM_HAND_EDGES,
        PLAYER_HAND_HEIGHT,
        state.hand_index as usize,
    );
}
