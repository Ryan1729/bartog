use web;

use game;

fn main() {
    real_main();
}

#[cfg(feature = "web_sys_version")]
mod wasm {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    pub fn run() {
        //console_log::init_with_level(log::Level::Debug).expect("error initializing logger");

        super::real_main();
    }
}

fn real_main() {
    let params = web::get_state_params();
    let state = game::BartogState::new(params);
    web::run(state);
}