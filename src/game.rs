use common::*;

#[inline]
pub fn update_and_render(framebuffer: &mut Framebuffer, state: &mut GameState, input: Input) {
    draw_winning_screen(framebuffer);
}
