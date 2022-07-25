use platform_types::{State, StateParams};

pub fn run<S: State + 'static>(state: S) {
    
}

pub fn get_state_params() -> StateParams {
    StateParams::default()
}
