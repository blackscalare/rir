use crate::constants;
use crate::utils;
use raylib::prelude::*;

pub struct Renderer {
    width: i32,
    height: i32,
}

impl Renderer {
    pub fn new(width: i32, height: i32) -> Renderer {
        Renderer { width, height }
    }

    pub fn update(&self, handle: &mut RaylibHandle, thread: &RaylibThread) {
        let mut draw_handle = handle.begin_drawing(thread);
        draw_handle.clear_background(Color::WHITE);
        let text_center_x_y = utils::center(
            self.width,
            self.height,
            constants::texts::TEST_TEXT.len() as i32,
            20,
        );
        draw_handle.draw_text(
            constants::texts::TEST_TEXT,
            text_center_x_y.0,
            text_center_x_y.1,
            20,
            Color::BLACK,
        );
        draw_handle.draw_fps(self.width - 50, 10);
    }
}
