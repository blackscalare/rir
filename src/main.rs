mod constants;
mod game_state;
mod input_handler;
mod renderer;
mod utils;
use input_handler::InputEvent;

use std::collections::VecDeque;

use crate::input_handler::poll_inputs;

fn main() {
    let (mut handle, thread) = raylib::init()
        .size(
            constants::sizes::WINDOW_WIDTH,
            constants::sizes::WINDOW_HEIGHT,
        )
        .title("rir")
        .build();

    handle.set_target_fps(60);
    let renderer = renderer::Renderer::new();
    let mut game_state = game_state::GameState::new();

    while !handle.window_should_close() {
        renderer.update(&mut handle, &thread, &game_state);
        let mut input_events: VecDeque<InputEvent> = poll_inputs(&handle);

        while let Some(event) = input_events.pop_front() {
            game_state.move_player(event);
        }

        game_state.handle_blobs();
    }
}
