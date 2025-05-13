mod constants;
mod input_handler;
mod renderer;
mod utils;

use input_handler::InputEvent;

use std::collections::VecDeque;

use crate::input_handler::poll_inputs;

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

fn main() {
    let (mut handle, thread) = raylib::init().size(WIDTH, HEIGHT).title("rir").build();

    let renderer = renderer::Renderer::new(WIDTH, HEIGHT);

    while !handle.window_should_close() {
        renderer.update(&mut handle, &thread);
        let mut input_events: VecDeque<InputEvent> = poll_inputs(&handle);

        while let Some(event) = input_events.pop_front() {
            print!("Pressing ");
            match event {
                InputEvent::Up => {
                    print!("up ");
                }
                InputEvent::Down => {
                    print!("down ");
                }
                InputEvent::Left => {
                    print!("left ");
                }
                InputEvent::Right => {
                    print!("right ");
                }
            }
            println!("");
        }
    }
}
