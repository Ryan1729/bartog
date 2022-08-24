fn main() {
    let params = platform::get_state_params();
    let state = game::BartogState::new(params);
    platform::run(state);
}