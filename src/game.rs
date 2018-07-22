use common::*;

use std::cmp::min;

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

#[inline]
pub fn update_and_render(framebuffer: &mut Framebuffer, state: &mut GameState, input: Input) {
    framebuffer.clear();

    draw_hand(framebuffer, &state.hand);

    if input.gamepad.contains(Button::Right) {
        state.hand.draw_from(&mut state.deck);
    } else if input.gamepad.contains(Button::Left) {
        state
            .hand
            .discard_randomly_to(&mut state.deck, &mut state.rng);
    }
}
