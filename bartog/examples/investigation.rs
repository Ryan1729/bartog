use platform_types::{Button, Command, SFX, State, StateParams};

struct StateWrapper {
    state: game::BartogState,
    frame_count: u64,
    stashed: Vec<Command>,
}

impl State for StateWrapper {
    fn frame(&mut self) -> (&[Command], &[SFX]) {
        self.frame_count += 1;
        if self.stashed.len() > 0 {
            (&self.stashed, &[])
        } else if self.frame_count < 10 {
            do_frame(&mut self.state, self.frame_count)
        } else {
            let (commands, _) = do_frame(&mut self.state, self.frame_count);
            self.stashed.extend_from_slice(commands);
            (&self.stashed, &[])
        }
    }

    fn press(&mut self, button: Button) {
        self.state.press(button)
    }

    fn release(&mut self, button: Button) {
        self.state.release(button)
    }
}

impl StateWrapper {
    fn new(params: StateParams) -> Self {
        Self {
            state: game::BartogState::new(params),
            frame_count: 0,
            stashed: Vec::new(),
        }
    }
}

fn do_frame(state: &mut game::BartogState, frame_count: u64) -> (&[Command], &[SFX]) {
    if frame_count == 2 {
        state.press(Button::A);
    } else if frame_count == 3 {
        state.release(Button::A);
    }

    state.frame()
}


fn main() {
    let seed = [10, 56, 42, 75, 1, 190, 216, 65, 6, 119, 65, 160, 129, 177, 4, 62];
    let state = StateWrapper::new((
        seed,
        None,
        None,
    ));

    platform::run(state);
}