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
            self.state.frame()
        } else {
            let (commands, _) = self.state.frame();
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

fn main() {
    let seed = [10, 56, 42, 75, 1, 190, 216, 65, 6, 119, 65, 160, 129, 177, 4, 62];
    let mut state = StateWrapper::new((
        seed,
        None,
        None,
    ));

    state.frame();
    state.press(Button::A);
    state.frame();
    state.release(Button::A);

    platform::run(state);
}