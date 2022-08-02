use web;

use game;

fn main() {
    fn logger(s: &str) {
        println!("{}", s);
    }

    fn error_logger(s: &str) {
        eprintln!("{}", s);
    }

    // TODO actual random seed.
    let seed = <_>::default();

    let params: game::StateParams = (
        seed,
        Some(logger),
        Some(error_logger),
    );

    run(params);
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(start)]
    pub fn run() {
        super::run(web::get_state_params());
    }
}

fn run(params: game::StateParams) {
    let state = game::BartogState::new(params);
    web::run(state);
}