fn main() {
    run();
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(start)]
    pub fn run() {
        super::run();
    }
}

fn run() {
    let params = platform::get_state_params();
    let state = game::BartogState::new(params);
    platform::run(state);
}