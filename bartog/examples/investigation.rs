use platform_types::{Button, Command, SFX, State, StateParams};

struct StateWrapper {
    state: game::BartogState,
}

impl State for StateWrapper {
    fn frame(&mut self) -> (&[Command], &[SFX]) {
        self.state.frame()
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
        }
    }
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