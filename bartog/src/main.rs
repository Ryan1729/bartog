use web;

use game;

fn main() {
    real_main();
}

#[cfg(feature = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    pub fn run() {
        super::real_main();
    }
}

fn real_main() {
    let params = web::get_state_params();
    let state = game::BartogState::new(params);
    web::run(state);
}