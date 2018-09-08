extern crate web;

extern crate game;

fn main() {
    let params = web::get_state_params();
    let state = game::BartogState::new(params);
    web::run(state);
}
