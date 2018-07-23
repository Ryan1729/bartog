use common::*;

use std::cmp::min;

#[allow(dead_code)]
fn draw_hand(framebuffer: &mut Framebuffer, hand: &Hand) {
    let len = hand.len() as u8;
    if len == 0 {
        return;
    }

    let left_edge = card::X_EMPTY_SPACE;
    let right_edge = SCREEN_WIDTH as u8 - card::X_EMPTY_SPACE;
    let full_width = right_edge.saturating_sub(left_edge);
    let usable_width = full_width.saturating_sub(card::WIDTH);
    let offset = min(usable_width / len, card::WIDTH);
    let mut x = left_edge;
    for &card in hand.iter() {
        framebuffer.draw_card(card, x, PLAYER_HAND_HEIGHT);

        x += offset;
    }
}

fn draw_hand_with_cursor(framebuffer: &mut Framebuffer, hand: &Hand, index: usize) {
    let len = hand.len() as u8;
    if len == 0 {
        return;
    }

    let left_edge = card::X_EMPTY_SPACE;
    let right_edge = SCREEN_WIDTH as u8 - card::X_EMPTY_SPACE;
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
        framebuffer.draw_card(card, x, PLAYER_HAND_HEIGHT);

        x += offset;
    }

    if let Some((card, cursor_offset)) = selected_card_and_offset {
        framebuffer.draw_highlighted_card(card, cursor_offset, PLAYER_HAND_HEIGHT);
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

    draw_hand_with_cursor(framebuffer, &state.hand, state.hand_index as usize);
}
