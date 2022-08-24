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
    let params = platform::get_state_params();
    let state = StateWrapper::new(params);
    platform::run(state);
}