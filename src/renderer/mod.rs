use crate::constants;
use crate::game_state::GameState;
use crate::utils;
use raylib::prelude::*;

pub struct Renderer {
    width: i32,
    height: i32,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            width: constants::sizes::WINDOW_WIDTH,
            height: constants::sizes::WINDOW_HEIGHT,
        }
    }

    pub fn update(&self, handle: &mut RaylibHandle, thread: &RaylibThread, game_state: &GameState) {
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

        draw_handle.draw_rectangle(
            game_state.get_player().x,
            game_state.get_player().y,
            constants::sizes::PLAYER_WIDTH,
            constants::sizes::PLAYER_HEIGHT,
            Color::BLACK,
        );

        for blob in game_state.get_blobs() {
            draw_handle.draw_circle(blob.x, blob.y, constants::sizes::BLOB_RADIUS, blob.color);
        }

        draw_handle.draw_fps(self.width - 50, 10);
    }
}
