use crate::text::bytes_lines;
use inner_common::*;
use std::cmp::max;

pub struct Framebuffer {
    pub buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new() -> Framebuffer {
        Framebuffer::default()
    }
}

type PaletteIndex = u8;

pub enum Kind {
    Gfx((u8, u8)),
    Font((u8, u8), PaletteIndex),
    Colour(PaletteIndex),
}

pub struct Command {
    pub rect: Rect,
    pub kind: Kind,
}

pub fn blit(
    buffer: &mut Vec<u32>,
    Command {
        kind,
        rect: Rect {
            x: display_x,
            y: display_y,
            w,
            h,
        },
    }: Command,
) {
    const D_WIDTH: usize = SCREEN_WIDTH as usize;
    let w = w as usize;
    let h = h as usize;

    let d_x = display_x as usize;
    let d_y = display_y as usize;

    let d_x_max = d_x + w;
    let d_y_max = d_y + h;

    match kind {
        Kind::Gfx((sprite_x, sprite_y)) => {
            let mut current_s_y = sprite_y as usize;
            for y in d_y..d_y_max {
                let mut current_s_x = sprite_x as usize;
                for x in d_x..d_x_max {
                    let colour = GFX[
                        current_s_x + current_s_y * GFX_WIDTH
                    ] as usize;
                    //make purple transparent
                    if colour != 4 {
                        let index = x + y * D_WIDTH;
                        if index < buffer.len() {
                            buffer[index] = PALETTE[colour];
                        }
                    }
                    current_s_x += 1;
                }
                current_s_y += 1;
            }
        },
        Kind::Font((sprite_x, sprite_y), colour) => {
            let mut current_s_y = sprite_y as usize;
            for y in d_y..d_y_max {
                let mut current_s_x = sprite_x as usize;
                for x in d_x..d_x_max {
                    let foxt_pixel_colour = FONT[
                        current_s_x + current_s_y * FONT_WIDTH
                    ] as usize;
                    //make black transparent
                    if foxt_pixel_colour != 0 {
                        let index = x + y * D_WIDTH;
                        if index < buffer.len() {
                            buffer[index] = PALETTE[colour as usize & 15];
                        }
                    }
                    current_s_x += 1;
                }
                current_s_y += 1;
            }
        },
        Kind::Colour(colour) => {
            for y in d_y..d_y_max {
                for x in d_x..d_x_max {
                    let index = x + y * D_WIDTH;
                    if index < buffer.len() {
                        buffer[index] = PALETTE[colour as usize & 15];
                    }
                }
            }
        }
    };
}

impl Framebuffer {
    pub fn sspr(
        &mut self,
        sprite_x: u8,
        sprite_y: u8,
        w: u8,
        h: u8,
        display_x: u8,
        display_y: u8,
    ) {
        blit(
            &mut self.buffer,
            Command {
                kind: Kind::Gfx((sprite_x, sprite_y)),
                rect: Rect {
                    x: display_x,
                    y: display_y,
                    w, 
                    h,
                },
            }
        )
    }

    fn print_char_raw(
        &mut self,
        sprite_x: u8,
        sprite_y: u8,
        w: u8,
        h: u8,
        display_x: u8,
        display_y: u8,
        colour: u8,
    ) {
        blit(
            &mut self.buffer,
            Command {
                kind: Kind::Font((sprite_x, sprite_y), colour),
                rect: Rect {
                    x: display_x,
                    y: display_y,
                    w, 
                    h,
                },
            }
        )
    }

    pub fn clearTo(&mut self, colour: PaletteIndex) {
        blit(
            &mut self.buffer,
            Command {
                kind: Kind::Colour(colour),
                rect: Rect {
                    x: 0,
                    y: 0,
                    w: SCREEN_WIDTH,
                    h: SCREEN_HEIGHT,
                },
            }
        )
    }

    fn spr(&mut self, sprite_number: u8, x: u8, y: u8) {
        let (sprite_x, sprite_y) = get_sprite_xy(sprite_number);
        self.sspr(sprite_x, sprite_y, SPRITE_SIZE, SPRITE_SIZE, x, y);
    }

    pub fn print(&mut self, bytes: &[u8], x: u8, mut y: u8, colour: u8) {
        for line in bytes_lines(bytes) {
            self.print_line(line, x, y, colour);
            y = y.saturating_add(FONT_SIZE);
        }
    }

    pub fn print_line(&mut self, bytes: &[u8], mut x: u8, y: u8, colour: u8) {
        let mut bytes_iter = bytes.iter();

        while let Some(&c) = bytes_iter.next() {
            if c == RANK_SUIT_PAIR_LAYOUT_CHAR {
                if let Some(&rank) = bytes_iter.next() {
                    let (sprite_x, sprite_y) = get_char_xy(rank);

                    if rank == TEN_CHAR {
                        x = x.saturating_add(FONT_ADVANCE / 4);
                        self.print_char_raw(sprite_x, sprite_y, FONT_SIZE, FONT_SIZE, x, y, colour);
                        x = x.saturating_add(FONT_ADVANCE * 3 / 4);
                    } else {
                        x = x.saturating_add(FONT_ADVANCE);
                        self.print_char_raw(sprite_x, sprite_y, FONT_SIZE, FONT_SIZE, x, y, colour);
                    }

                    x = x.saturating_add(FONT_ADVANCE);

                    if let Some(&suit) = bytes_iter.next() {
                        let (sprite_x, sprite_y) = get_char_xy(suit);

                        x = x.saturating_add(FONT_ADVANCE / 4);
                        self.print_char_raw(sprite_x, sprite_y, FONT_SIZE, FONT_SIZE, x, y, colour);
                        x = x.saturating_add(FONT_ADVANCE * 3 / 4);
                    } else {
                        x = x.saturating_add(FONT_ADVANCE);
                    }

                    x = x.saturating_add(FONT_ADVANCE);
                }
                //Need 4 chars of room
                bytes_iter.next();

                continue;
            }

            let (sprite_x, sprite_y) = get_char_xy(c);
            self.print_char_raw(sprite_x, sprite_y, FONT_SIZE, FONT_SIZE, x, y, colour);
            x = x.saturating_add(FONT_ADVANCE);
        }
    }

    fn print_line_raw(&mut self, bytes: &[u8], mut x: u8, y: u8, colour: u8) {
        for &c in bytes {
            let (sprite_x, sprite_y) = get_char_xy(c);
            self.print_char_raw(sprite_x, sprite_y, FONT_SIZE, FONT_SIZE, x, y, colour);
            x = x.saturating_add(FONT_ADVANCE);
        }
    }

    pub fn print_single_line_number(&mut self, number: usize, x: u8, y :u8, colour: u8) {
        self.print_line_raw(number.to_string().as_bytes(), x, y, colour);
    }

    pub fn print_char(&mut self, character: u8, x: u8, y: u8, colour: u8) {
        let (sprite_x, sprite_y) = get_char_xy(character);
        self.print_char_raw(sprite_x, sprite_y, FONT_SIZE, FONT_SIZE, x, y, colour);
    }

    pub fn draw_card(&mut self, card: Card, x: u8, y: u8) {
        self.sspr(
            card::FRONT_SPRITE_X,
            card::FRONT_SPRITE_Y,
            card::WIDTH,
            card::HEIGHT,
            x,
            y,
        );

        let (colour, suit_char) = get_suit_colour_and_char(get_suit(card));

        let rank_char = get_rank_char(card);

        self.print_char(
            rank_char,
            x + card::LEFT_RANK_X,
            y + card::LEFT_RANK_Y,
            colour,
        );
        self.print_char(
            suit_char,
            x + card::LEFT_SUIT_X,
            y + card::LEFT_SUIT_Y,
            colour,
        );

        self.print_char(
            rank_char | FONT_FLIP,
            x + card::RIGHT_RANK_X,
            y + card::RIGHT_RANK_Y,
            colour,
        );
        self.print_char(
            suit_char | FONT_FLIP,
            x + card::RIGHT_SUIT_X,
            y + card::RIGHT_SUIT_Y,
            colour,
        );
    }

    pub fn draw_highlighted_card(&mut self, card: Card, x: u8, y: u8) {
        self.draw_card(card, x, y);

        self.sspr(
            cursor::SPRITE_X,
            cursor::SPRITE_Y,
            cursor::WIDTH,
            cursor::HEIGHT,
            x.wrapping_sub(1),
            y.wrapping_sub(1),
        );
    }

    pub fn draw_card_back(&mut self, x: u8, y: u8) {
        self.sspr(
            card::BACK_SPRITE_X,
            card::BACK_SPRITE_Y,
            card::WIDTH,
            card::HEIGHT,
            x,
            y,
        );
    }

    pub fn full_window(&mut self) {
        self.window(0, 0, SCREEN_WIDTH as u8, SCREEN_HEIGHT as u8);
    }

    pub fn center_half_window(&mut self) {
        self.window(
            SCREEN_WIDTH / 4,
            SCREEN_HEIGHT / 4,
            SCREEN_WIDTH / 2,
            SCREEN_HEIGHT / 2,
        );
    }

    pub fn window(&mut self, x: u8, y: u8, w: u8, h: u8) {
        self.nine_slice(WINDOW_TOP_LEFT, x, y, w, h);
    }

    pub fn button(&mut self, x: u8, y: u8, w: u8, h: u8) {
        self.nine_slice(BUTTON_TOP_LEFT, x, y, w, h);
    }

    pub fn button_hot(&mut self, x: u8, y: u8, w: u8, h: u8) {
        self.nine_slice(BUTTON_HOT_TOP_LEFT, x, y, w, h);
    }

    pub fn button_pressed(&mut self, x: u8, y: u8, w: u8, h: u8) {
        self.nine_slice(BUTTON_PRESSED_TOP_LEFT, x, y, w, h);
    }

    pub fn nine_slice(&mut self, top_left: u8, x: u8, y: u8, w: u8, h: u8) {
        let TOP_LEFT: u8 = top_left;
        let TOP: u8 = TOP_LEFT + 1;
        let TOP_RIGHT: u8 = TOP + 1;

        let MIDDLE_LEFT: u8 = TOP_LEFT + SPRITES_PER_ROW;
        let MIDDLE: u8 = TOP + SPRITES_PER_ROW;
        let MIDDLE_RIGHT: u8 = TOP_RIGHT + SPRITES_PER_ROW;

        let BOTTOM_LEFT: u8 = MIDDLE_LEFT + SPRITES_PER_ROW;
        let BOTTOM: u8 = MIDDLE + SPRITES_PER_ROW;
        let BOTTOM_RIGHT: u8 = MIDDLE_RIGHT + SPRITES_PER_ROW;

        let after_left_corner = x.saturating_add(SPRITE_SIZE);
        let before_right_corner = x.saturating_add(w).saturating_sub(SPRITE_SIZE);

        let below_top_corner = y.saturating_add(SPRITE_SIZE);
        let above_bottom_corner = y.saturating_add(h).saturating_sub(SPRITE_SIZE);

        for fill_y in (below_top_corner..above_bottom_corner).step_by(SPRITE_SIZE as _) {
            for fill_x in (after_left_corner..before_right_corner).step_by(SPRITE_SIZE as _) {
                self.spr(MIDDLE, fill_x, fill_y);
            }
        }

        for fill_x in (after_left_corner..before_right_corner).step_by(SPRITE_SIZE as _) {
            self.spr(TOP, fill_x, y);
            self.spr(BOTTOM, fill_x, above_bottom_corner);
        }

        for fill_y in (below_top_corner..above_bottom_corner).step_by(SPRITE_SIZE as _) {
            self.spr(MIDDLE_LEFT, x, fill_y);
            self.spr(MIDDLE_RIGHT, before_right_corner, fill_y);
        }

        self.spr(TOP_LEFT, x, y);
        self.spr(TOP_RIGHT, before_right_corner, y);
        self.spr(BOTTOM_LEFT, x, above_bottom_corner);
        self.spr(BOTTOM_RIGHT, before_right_corner, above_bottom_corner);
    }

    pub fn bottom_six_slice(&mut self, top_left: u8, x: u8, y: u8, w: u8, h: u8) {
        let TOP_LEFT: u8 = top_left;
        let TOP: u8 = TOP_LEFT + 1;
        let TOP_RIGHT: u8 = TOP + 1;

        let MIDDLE_LEFT: u8 = TOP_LEFT + SPRITES_PER_ROW;
        let MIDDLE: u8 = TOP + SPRITES_PER_ROW;
        let MIDDLE_RIGHT: u8 = TOP_RIGHT + SPRITES_PER_ROW;

        let BOTTOM_LEFT: u8 = MIDDLE_LEFT + SPRITES_PER_ROW;
        let BOTTOM: u8 = MIDDLE + SPRITES_PER_ROW;
        let BOTTOM_RIGHT: u8 = MIDDLE_RIGHT + SPRITES_PER_ROW;

        let after_left_corner = x.saturating_add(SPRITE_SIZE);
        let before_right_corner = x.saturating_add(w).saturating_sub(SPRITE_SIZE);

        let below_top_corner = y.saturating_add(SPRITE_SIZE);
        let above_bottom_corner = y.saturating_add(h).saturating_sub(SPRITE_SIZE);

        for fill_y in (below_top_corner..above_bottom_corner).step_by(SPRITE_SIZE as _) {
            for fill_x in (after_left_corner..before_right_corner).step_by(SPRITE_SIZE as _) {
                self.spr(MIDDLE, fill_x, fill_y);
            }
        }

        for fill_x in (after_left_corner..before_right_corner).step_by(SPRITE_SIZE as _) {
            self.spr(MIDDLE, fill_x, y);
            self.spr(BOTTOM, fill_x, above_bottom_corner);
        }

        for fill_y in (below_top_corner..above_bottom_corner).step_by(SPRITE_SIZE as _) {
            self.spr(MIDDLE_LEFT, x, fill_y);
            self.spr(MIDDLE_RIGHT, before_right_corner, fill_y);
        }

        self.spr(MIDDLE_LEFT, x, y);
        self.spr(MIDDLE_RIGHT, before_right_corner, y);
        self.spr(BOTTOM_LEFT, x, above_bottom_corner);
        self.spr(BOTTOM_RIGHT, before_right_corner, above_bottom_corner);
    }

    fn three_slice(&mut self, left_edge: u8, x: u8, y: u8, w: u8) {
        let LEFT: u8 = left_edge;
        let MIDDLE: u8 = LEFT + 1;
        let RIGHT: u8 = MIDDLE + 1;

        let after_left_corner = x.saturating_add(SPRITE_SIZE);
        let before_right_corner = x.saturating_add(w).saturating_sub(SPRITE_SIZE);

        self.spr(LEFT, x, y);

        for fill_x in (after_left_corner..before_right_corner).step_by(SPRITE_SIZE as _) {
            self.spr(MIDDLE, fill_x, y);
        }

        self.spr(RIGHT, before_right_corner, y);
    }

    pub fn row(&mut self, x: u8, y: u8, w: u8) {
        self.three_slice(ROW_LEFT_EDGE, x, y, w);
    }

    pub fn row_hot(&mut self, x: u8, y: u8, w: u8) {
        self.three_slice(ROW_HOT_LEFT_EDGE, x, y, w);
    }

    pub fn row_pressed(&mut self, x: u8, y: u8, w: u8) {
        self.three_slice(ROW_PRESSED_LEFT_EDGE, x, y, w);
    }

    pub fn row_marker(&mut self, x: u8, y: u8, w: u8) {
        self.three_slice(ROW_MARKER_LEFT_EDGE, x, y, w);
    }

    pub fn checkbox(&mut self, x: u8, y: u8, checked: bool) {
        self.spr(
            if checked {
                checkbox::CHECKED
            } else {
                checkbox::UNCHECKED
            },
            x,
            y,
        );
    }

    pub fn checkbox_hot(&mut self, x: u8, y: u8, checked: bool) {
        self.spr(
            if checked {
                checkbox::HOT_CHECKED
            } else {
                checkbox::HOT_UNCHECKED
            },
            x,
            y,
        );
    }

    pub fn checkbox_pressed(&mut self, x: u8, y: u8, checked: bool) {
        self.spr(
            if checked {
                checkbox::PRESSED_CHECKED
            } else {
                checkbox::PRESSED_UNCHECKED
            },
            x,
            y,
        );
    }

    fn xy_to_i(x: usize, y: usize) -> usize {
        y.saturating_mul(usize::from(SCREEN_WIDTH)).saturating_add(x)
    }
}

pub fn get_sprite_xy(sprite_number: u8) -> (u8, u8) {
    (
        (sprite_number % SPRITES_PER_ROW) * SPRITE_SIZE,
        (sprite_number / SPRITES_PER_ROW) * SPRITE_SIZE,
    )
}

pub fn get_char_xy(sprite_number: u8) -> (u8, u8) {
    const SPRITES_PER_ROW: u8 = FONT_WIDTH as u8 / FONT_SIZE;

    (
        (sprite_number % SPRITES_PER_ROW) * FONT_SIZE,
        (sprite_number / SPRITES_PER_ROW) * FONT_SIZE,
    )
}

impl Default for Framebuffer {
    fn default() -> Self {
        let mut buffer = Vec::new();
        buffer.resize(
            usize::from(SCREEN_WIDTH) * usize::from(SCREEN_HEIGHT),
            PALETTE[0]
        );

        Framebuffer { buffer }
    }
}

use std::cmp::min;

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: u8,
    pub y: u8,
    pub w: u8,
    pub h: u8,
}

impl From<((u8, u8, u8, u8))> for Rect {
    #[inline]
    fn from((x, y, w, h): (u8, u8, u8, u8)) -> Self {
        Rect { x, y, w, h }
    }
}

impl From<Rect> for (u8, u8, u8, u8) {
    #[inline]
    fn from(Rect { x, y, w, h }: Rect) -> Self {
        (x, y, w, h)
    }
}

impl From<((u8, u8), (u8, u8))> for Rect {
    #[inline]
    fn from(((x, y), (w, h)): ((u8, u8), (u8, u8))) -> Self {
        Rect { x, y, w, h }
    }
}

impl From<Rect> for ((u8, u8), (u8, u8)) {
    #[inline]
    fn from(Rect { x, y, w, h }: Rect) -> Self {
        ((x, y), (w, h))
    }
}

pub fn get_text_dimensions(bytes: &[u8]) -> (u8, u8) {
    let mut width: u8 = 0;
    let mut height: u8 = 0;
    for line in bytes_lines(bytes) {
        height = height.saturating_add(1);
        width = max(width, line.len() as u8);
    }

    width = width.saturating_mul(FONT_ADVANCE);
    height = height.saturating_mul(FONT_SIZE);

    (width, height)
}

pub fn center_line_in_rect<R: Into<Rect>>(text_length: u8, r: R) -> (u8, u8) {
    let Rect { x, y, w, h } = r.into();
    let middle_x = x + (w / 2);
    let middle_y = y + (h / 2);

    let text_x =
        (middle_x as usize).saturating_sub(text_length as usize * FONT_ADVANCE as usize / 2) as u8;
    let text_y = (middle_y as usize).saturating_sub(FONT_SIZE as usize / 2) as u8;

    (text_x, text_y)
}

pub fn center_rect_in_rect<R: Into<Rect>>((width, height): (u8, u8), r: R) -> (u8, u8) {
    let Rect { x, y, w, h } = r.into();
    let middle_x = x + (w / 2);
    let middle_y = y + (h / 2);

    let left_x = middle_x.saturating_sub(width / 2);
    let top_y = middle_y.saturating_sub(height / 2);

    (left_x, top_y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::*;

    #[test]
    fn test_get_text_dimensions_then_center_rect_in_rect_matches_center_line_in_rect_for_a_single_line(
    ) {
        quickcheck(
                    get_text_dimensions_then_center_rect_in_rect_matches_center_line_in_rect_for_a_single_line
                        as fn(u8, (u8, u8, u8, u8)) -> TestResult,
                )
    }
    fn get_text_dimensions_then_center_rect_in_rect_matches_center_line_in_rect_for_a_single_line(
        char_count: u8,
        r: (u8, u8, u8, u8),
    ) -> TestResult {
        if char_count as usize * FONT_ADVANCE as usize > 255 {
            return TestResult::discard();
        }

        let rect: Rect = r.into();

        let line_point = center_line_in_rect(char_count, rect);

        let text = vec![b'A'; char_count as usize];

        let text_point = center_rect_in_rect(get_text_dimensions(&text), rect);
        assert_eq!(text_point, line_point);
        TestResult::from_bool(text_point == line_point)
    }

    #[test]
    fn test_center_rect_in_rect_actually_centers_when_possible() {
        quickcheck(
            center_rect_in_rect_actually_centers_when_possible
                as fn(((u8, u8), (u8, u8, u8, u8))) -> TestResult,
        )
    }
    fn center_rect_in_rect_actually_centers_when_possible(
        ((w, h), r): ((u8, u8), (u8, u8, u8, u8)),
    ) -> TestResult {
        let rect: Rect = r.into();

        if rect.w & 1 == 1 || w & 1 == 1 {
            return TestResult::discard();
        }

        let (x, _y) = center_rect_in_rect((w, h), rect);
        let left_side = x.saturating_sub(rect.x);
        let right_side = (rect.x + rect.w).saturating_sub(x + w);

        assert_eq!(left_side, right_side);
        TestResult::from_bool(left_side == right_side)
    }

    #[test]
    fn test_center_line_in_rect_actually_centers_when_possible() {
        assert!(FONT_ADVANCE & 1 == 0);
        quickcheck(
            center_line_in_rect_actually_centers_when_possible
                as fn((u8, (u8, u8, u8, u8))) -> TestResult,
        )
    }
    fn center_line_in_rect_actually_centers_when_possible(
        (length, r): (u8, (u8, u8, u8, u8)),
    ) -> TestResult {
        let rect: Rect = r.into();

        if rect.w & 1 == 1 || rect.w < FONT_ADVANCE || length >= (256 / FONT_ADVANCE as usize) as u8
        {
            return TestResult::discard();
        }
        let w = length * FONT_ADVANCE;

        let (x, _y) = center_line_in_rect(length, rect);
        let left_side = (x as usize).saturating_sub(rect.x as usize);
        let right_side =
            (rect.x as usize + rect.w as usize).saturating_sub(x as usize + w as usize);

        assert_eq!(left_side, right_side);
        TestResult::from_bool(left_side == right_side)
    }
}
