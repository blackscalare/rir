use raylib::prelude::*;
mod render;
pub mod utils;

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Hello World")
        .build();

    let text = "Hello, world!";

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        let text_center = utils::center(WIDTH, HEIGHT, text.len() as i32, 20);

        d.draw_text(text, text_center.0, text_center.1, 20, Color::BLACK);
        d.draw_fps(WIDTH - 50, 10);
    }
}
