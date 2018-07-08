use common::*;

#[inline]
pub fn update_and_render(framebuffer: &mut Framebuffer, state: &mut GameState, input: Input) {
    framebuffer.clear();

    let mut x = card::X_EMPTY_SPACE;
    let mut y = card::Y_EMPTY_SPACE;
    for card in 0..state.count {
        framebuffer.draw_card(card, x, y);

        x += card::HAND_OFFSET;
        if x as usize + card::WIDTH as usize > SCREEN_WIDTH {
            x = card::X_EMPTY_SPACE;
            y += card::HEIGHT_PLUS_SPACE;
        }
    }

    if input.gamepad.contains(Button::Right) {
        if state.count < 52 {
            state.count += 1;
        }
    } else if input.gamepad.contains(Button::Left) {
        state.count = state.count.saturating_sub(1);
    }
}
