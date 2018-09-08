extern crate web;

extern crate game;

fn main() {
    let (seed, logger) = web::get_state_params();
    let state = game::BartogState::new(seed, logger);
    web::run(state);
}
