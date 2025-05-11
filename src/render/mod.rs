use crate::utils;
use raylib::prelude::*;

pub struct Renderer {
    width: i32,
    height: i32,
    rl_handle: Option<RaylibHandle>,
    rl_thread: Option<RaylibThread>,
}

impl Renderer {
    pub fn new(width: i32, height: i32) -> Renderer {
        Renderer {
            width,
            height,
            rl_handle: None,
            rl_thread: None,
        }
    }

    pub fn init(&mut self) {
        let (handle, thread) = raylib::init()
            .size(self.width, self.height)
            .title("rir")
            .build();

        self.rl_handle = Some(handle);
        self.rl_thread = Some(thread);
    }

    pub fn start(&mut self) {
        let (handle, thread) = (
            self.rl_handle.as_mut().expect("Handle not initialized"),
            self.rl_thread.as_mut().expect("Thread not initialized"),
        );

        let text = "Test";

        while !handle.window_should_close() {
            let mut draw_handle = handle.begin_drawing(thread);
            draw_handle.clear_background(Color::WHITE);
            let text_center_x_y = utils::center(self.width, self.height, text.len() as i32, 20);
            draw_handle.draw_text(text, text_center_x_y.0, text_center_x_y.1, 20, Color::BLACK);
            draw_handle.draw_fps(self.width - 50, 10);
        }
    }
}
