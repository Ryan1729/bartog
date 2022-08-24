use platform_types::{Button, Command, SFX, State, Rect, Kind::*};

struct StateWrapper {
    frame_count: u64,
}

impl State for StateWrapper {
    fn frame(&mut self) -> (&[Command], &[SFX]) {
        self.frame_count += 1;
        if self.frame_count <= 1 {
            (&COMMANDS_8, &[])
        } else if self.frame_count == 2 {
            (&COMMANDS_9, &[])
        } else  {
            (&COMMANDS_10, &[])
        }
    }

    fn press(&mut self, _: Button) {
    }

    fn release(&mut self, _: Button) {
    }
}

fn main() {
    platform::run(StateWrapper{
        frame_count: 0,
    });
}

const COMMANDS_8: [Command; 50] = [
        Command {
            rect: Rect {
                x: 0,
                y: 0,
                w: 128,
                h: 128,
            },
            kind: Colour(
                1,
            ),
        },
        Command {
            rect: Rect {
                x: 2,
                y: 32,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    26,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 2,
                y: 39,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    26,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 2,
                y: 46,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    26,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 2,
                y: 53,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    26,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 2,
                y: 60,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    26,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 2,
                y: 1,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    26,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 22,
                y: 1,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    26,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 42,
                y: 1,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    26,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 62,
                y: 1,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    26,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 82,
                y: 1,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    26,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 106,
                y: 32,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    26,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 106,
                y: 40,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    26,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 106,
                y: 48,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    26,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 106,
                y: 56,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    26,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 40,
                y: 32,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    26,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 46,
                y: 66,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    24,
                    24,
                ),
                7,
            ),
        },
        Command {
            rect: Rect {
                x: 50,
                y: 66,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    16,
                    24,
                ),
                7,
            ),
        },
        Command {
            rect: Rect {
                x: 76,
                y: 66,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    0,
                    24,
                ),
                7,
            ),
        },
        Command {
            rect: Rect {
                x: 22,
                y: 112,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    2,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 25,
                y: 115,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    88,
                    8,
                ),
                7,
            ),
        },
        Command {
            rect: Rect {
                x: 23,
                y: 122,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    96,
                    8,
                ),
                7,
            ),
        },
        Command {
            rect: Rect {
                x: 31,
                y: 131,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    88,
                    72,
                ),
                7,
            ),
        },
        Command {
            rect: Rect {
                x: 33,
                y: 124,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    96,
                    72,
                ),
                7,
            ),
        },
        Command {
            rect: Rect {
                x: 42,
                y: 112,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    2,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 45,
                y: 115,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    8,
                    56,
                ),
                7,
            ),
        },
        Command {
            rect: Rect {
                x: 43,
                y: 122,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    120,
                    8,
                ),
                7,
            ),
        },
        Command {
            rect: Rect {
                x: 51,
                y: 131,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    8,
                    120,
                ),
                7,
            ),
        },
        Command {
            rect: Rect {
                x: 53,
                y: 124,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    120,
                    72,
                ),
                7,
            ),
        },
        Command {
            rect: Rect {
                x: 62,
                y: 112,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    2,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 65,
                y: 115,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    8,
                    56,
                ),
                2,
            ),
        },
        Command {
            rect: Rect {
                x: 63,
                y: 122,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    112,
                    8,
                ),
                2,
            ),
        },
        Command {
            rect: Rect {
                x: 71,
                y: 131,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    8,
                    120,
                ),
                2,
            ),
        },
        Command {
            rect: Rect {
                x: 73,
                y: 124,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    112,
                    72,
                ),
                2,
            ),
        },
        Command {
            rect: Rect {
                x: 82,
                y: 112,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    2,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 85,
                y: 115,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    72,
                    24,
                ),
                7,
            ),
        },
        Command {
            rect: Rect {
                x: 83,
                y: 122,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    96,
                    8,
                ),
                7,
            ),
        },
        Command {
            rect: Rect {
                x: 91,
                y: 131,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    72,
                    88,
                ),
                7,
            ),
        },
        Command {
            rect: Rect {
                x: 93,
                y: 124,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    96,
                    72,
                ),
                7,
            ),
        },
        Command {
            rect: Rect {
                x: 2,
                y: 112,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    2,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 5,
                y: 115,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    56,
                    24,
                ),
                2,
            ),
        },
        Command {
            rect: Rect {
                x: 3,
                y: 122,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    112,
                    8,
                ),
                2,
            ),
        },
        Command {
            rect: Rect {
                x: 11,
                y: 131,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    56,
                    88,
                ),
                2,
            ),
        },
        Command {
            rect: Rect {
                x: 13,
                y: 124,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    112,
                    72,
                ),
                2,
            ),
        },
        Command {
            rect: Rect {
                x: 1,
                y: 111,
                w: 24,
                h: 32,
            },
            kind: Gfx(
                (
                    49,
                    0,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 98,
                y: 56,
                w: 20,
                h: 30,
            },
            kind: Gfx(
                (
                    2,
                    1,
                ),
            ),
        },
        Command {
            rect: Rect {
                x: 101,
                y: 59,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    8,
                    56,
                ),
                7,
            ),
        },
        Command {
            rect: Rect {
                x: 99,
                y: 66,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    96,
                    8,
                ),
                7,
            ),
        },
        Command {
            rect: Rect {
                x: 107,
                y: 75,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    8,
                    120,
                ),
                7,
            ),
        },
        Command {
            rect: Rect {
                x: 109,
                y: 68,
                w: 8,
                h: 8,
            },
            kind: Font(
                (
                    96,
                    72,
                ),
                7,
            ),
        },
    ];

const COMMANDS_9: [Command; 50] = [
    Command {
        rect: Rect {
            x: 0,
            y: 0,
            w: 128,
            h: 128,
        },
        kind: Colour(
            1,
        ),
    },
    Command {
        rect: Rect {
            x: 2,
            y: 32,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                26,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 2,
            y: 39,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                26,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 2,
            y: 46,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                26,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 2,
            y: 53,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                26,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 2,
            y: 60,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                26,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 2,
            y: 1,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                26,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 22,
            y: 1,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                26,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 42,
            y: 1,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                26,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 62,
            y: 1,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                26,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 82,
            y: 1,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                26,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 106,
            y: 32,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                26,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 106,
            y: 40,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                26,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 106,
            y: 48,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                26,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 106,
            y: 56,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                26,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 40,
            y: 32,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                26,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 46,
            y: 66,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                24,
                24,
            ),
            7,
        ),
    },
    Command {
        rect: Rect {
            x: 50,
            y: 66,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                16,
                24,
            ),
            7,
        ),
    },
    Command {
        rect: Rect {
            x: 76,
            y: 66,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                0,
                24,
            ),
            7,
        ),
    },
    Command {
        rect: Rect {
            x: 22,
            y: 112,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                2,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 25,
            y: 115,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                88,
                8,
            ),
            7,
        ),
    },
    Command {
        rect: Rect {
            x: 23,
            y: 122,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                96,
                8,
            ),
            7,
        ),
    },
    Command {
        rect: Rect {
            x: 31,
            y: 131,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                88,
                72,
            ),
            7,
        ),
    },
    Command {
        rect: Rect {
            x: 33,
            y: 124,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                96,
                72,
            ),
            7,
        ),
    },
    Command {
        rect: Rect {
            x: 42,
            y: 112,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                2,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 45,
            y: 115,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                8,
                56,
            ),
            7,
        ),
    },
    Command {
        rect: Rect {
            x: 43,
            y: 122,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                120,
                8,
            ),
            7,
        ),
    },
    Command {
        rect: Rect {
            x: 51,
            y: 131,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                8,
                120,
            ),
            7,
        ),
    },
    Command {
        rect: Rect {
            x: 53,
            y: 124,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                120,
                72,
            ),
            7,
        ),
    },
    Command {
        rect: Rect {
            x: 62,
            y: 112,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                2,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 65,
            y: 115,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                8,
                56,
            ),
            2,
        ),
    },
    Command {
        rect: Rect {
            x: 63,
            y: 122,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                112,
                8,
            ),
            2,
        ),
    },
    Command {
        rect: Rect {
            x: 71,
            y: 131,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                8,
                120,
            ),
            2,
        ),
    },
    Command {
        rect: Rect {
            x: 73,
            y: 124,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                112,
                72,
            ),
            2,
        ),
    },
    Command {
        rect: Rect {
            x: 82,
            y: 112,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                2,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 85,
            y: 115,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                72,
                24,
            ),
            7,
        ),
    },
    Command {
        rect: Rect {
            x: 83,
            y: 122,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                96,
                8,
            ),
            7,
        ),
    },
    Command {
        rect: Rect {
            x: 91,
            y: 131,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                72,
                88,
            ),
            7,
        ),
    },
    Command {
        rect: Rect {
            x: 93,
            y: 124,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                96,
                72,
            ),
            7,
        ),
    },
    Command {
        rect: Rect {
            x: 2,
            y: 112,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                2,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 5,
            y: 115,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                56,
                24,
            ),
            2,
        ),
    },
    Command {
        rect: Rect {
            x: 3,
            y: 122,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                112,
                8,
            ),
            2,
        ),
    },
    Command {
        rect: Rect {
            x: 11,
            y: 131,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                56,
                88,
            ),
            2,
        ),
    },
    Command {
        rect: Rect {
            x: 13,
            y: 124,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                112,
                72,
            ),
            2,
        ),
    },
    Command {
        rect: Rect {
            x: 1,
            y: 111,
            w: 24,
            h: 32,
        },
        kind: Gfx(
            (
                49,
                0,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 96,
            y: 55,
            w: 20,
            h: 30,
        },
        kind: Gfx(
            (
                2,
                1,
            ),
        ),
    },
    Command {
        rect: Rect {
            x: 99,
            y: 58,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                8,
                56,
            ),
            7,
        ),
    },
    Command {
        rect: Rect {
            x: 97,
            y: 65,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                96,
                8,
            ),
            7,
        ),
    },
    Command {
        rect: Rect {
            x: 105,
            y: 74,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                8,
                120,
            ),
            7,
        ),
    },
    Command {
        rect: Rect {
            x: 107,
            y: 67,
            w: 8,
            h: 8,
        },
        kind: Font(
            (
                96,
                72,
            ),
            7,
        ),
    },
];

const COMMANDS_10: [Command; 50] = [
            Command {
                rect: Rect {
                    x: 0,
                    y: 0,
                    w: 128,
                    h: 128,
                },
                kind: Colour(
                    1,
                ),
            },
            Command {
                rect: Rect {
                    x: 2,
                    y: 32,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 2,
                    y: 39,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 2,
                    y: 46,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 2,
                    y: 53,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 2,
                    y: 60,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 2,
                    y: 1,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 22,
                    y: 1,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 42,
                    y: 1,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 62,
                    y: 1,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 82,
                    y: 1,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 106,
                    y: 32,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 106,
                    y: 40,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 106,
                    y: 48,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 106,
                    y: 56,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 40,
                    y: 32,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 46,
                    y: 66,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        24,
                        24,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 50,
                    y: 66,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        16,
                        24,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 76,
                    y: 66,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        0,
                        24,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 22,
                    y: 112,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        2,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 25,
                    y: 115,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        88,
                        8,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 23,
                    y: 122,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        96,
                        8,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 31,
                    y: 131,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        88,
                        72,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 33,
                    y: 124,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        96,
                        72,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 42,
                    y: 112,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        2,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 45,
                    y: 115,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        8,
                        56,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 43,
                    y: 122,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        120,
                        8,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 51,
                    y: 131,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        8,
                        120,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 53,
                    y: 124,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        120,
                        72,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 62,
                    y: 112,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        2,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 65,
                    y: 115,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        8,
                        56,
                    ),
                    2,
                ),
            },
            Command {
                rect: Rect {
                    x: 63,
                    y: 122,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        112,
                        8,
                    ),
                    2,
                ),
            },
            Command {
                rect: Rect {
                    x: 71,
                    y: 131,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        8,
                        120,
                    ),
                    2,
                ),
            },
            Command {
                rect: Rect {
                    x: 73,
                    y: 124,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        112,
                        72,
                    ),
                    2,
                ),
            },
            Command {
                rect: Rect {
                    x: 82,
                    y: 112,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        2,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 85,
                    y: 115,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        72,
                        24,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 83,
                    y: 122,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        96,
                        8,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 91,
                    y: 131,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        72,
                        88,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 93,
                    y: 124,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        96,
                        72,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 2,
                    y: 112,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        2,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 5,
                    y: 115,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        56,
                        24,
                    ),
                    2,
                ),
            },
            Command {
                rect: Rect {
                    x: 3,
                    y: 122,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        112,
                        8,
                    ),
                    2,
                ),
            },
            Command {
                rect: Rect {
                    x: 11,
                    y: 131,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        56,
                        88,
                    ),
                    2,
                ),
            },
            Command {
                rect: Rect {
                    x: 13,
                    y: 124,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        112,
                        72,
                    ),
                    2,
                ),
            },
            Command {
                rect: Rect {
                    x: 1,
                    y: 111,
                    w: 24,
                    h: 32,
                },
                kind: Gfx(
                    (
                        49,
                        0,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 94,
                    y: 54,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        2,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 97,
                    y: 57,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        8,
                        56,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 95,
                    y: 64,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        96,
                        8,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 103,
                    y: 73,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        8,
                        120,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 105,
                    y: 66,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        96,
                        72,
                    ),
                    7,
                ),
            },
        ];