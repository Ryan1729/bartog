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

const COMMANDS_8: [Command; 1] = [
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
    ];

const COMMANDS_9: [Command; 1] = [
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
];

const COMMANDS_10: [Command; 1] = [
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
];