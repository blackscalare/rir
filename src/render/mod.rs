use crate::constants;
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

    // Checks if handle and thread is initialized, and starts the drawing.
    // Blocks the thread so use in the last spot
    // TODO do not block main thread
    //      or create threads where everything else like input is handled
    pub fn start(&mut self) {
        let (handle, thread) = (
            self.rl_handle.as_mut().expect("Handle not initialized"),
            self.rl_thread.as_mut().expect("Thread not initialized"),
        );

        Self::draw(handle, thread, self.width, self.height);
    }

    fn draw(handle: &mut RaylibHandle, thread: &RaylibThread, width: i32, height: i32) {
        while !handle.window_should_close() {
            let mut draw_handle = handle.begin_drawing(thread);
            draw_handle.clear_background(Color::WHITE);
            let text_center_x_y =
                utils::center(width, height, constants::texts::TEST_TEXT.len() as i32, 20);
            draw_handle.draw_text(
                constants::texts::TEST_TEXT,
                text_center_x_y.0,
                text_center_x_y.1,
                20,
                Color::BLACK,
            );
            draw_handle.draw_fps(width - 50, 10);
        }
    }
}
