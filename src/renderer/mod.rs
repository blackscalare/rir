use crate::constants::files::SMALL_TREE_GIF;
use crate::constants::sizes::{PLAYER_HEIGHT, PLAYER_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::game_state::GameState;
use crate::gui::GUI;
use crate::renderer::animation_handler::AnimationHandler;
use raylib::prelude::*;
mod animation_handler;
mod texture_handler;
use std::env;
use std::path::Path;

pub struct Renderer {
    width: i32,
    height: i32,
    pub tree_anim: AnimationHandler,
}

impl Renderer {
    pub fn new() -> Renderer {
        unsafe {
            let tree_anim = AnimationHandler::new_from_memory(SMALL_TREE_GIF, ".gif", 0.2);
            Renderer {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                tree_anim,
            }
        }
    }

    pub fn update(
        &mut self,
        handle: &mut RaylibHandle,
        thread: &RaylibThread,
        game_state: &GameState,
        gui: &GUI,
    ) {
        let mut draw_handle = handle.begin_drawing(thread);
        draw_handle.clear_background(Color::WHITE);

        draw_handle.draw_rectangle(
            game_state.get_player().x,
            game_state.get_player().y,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
            Color::BLACK,
        );

        gui.draw(&mut draw_handle);

        for blob in game_state.get_blobs() {
            draw_handle.draw_circle(blob.x, blob.y, blob.radius, blob.color);
        }

        if !game_state.get_world().get_trees().is_empty() {
            self.tree_anim.update(draw_handle.get_frame_time());
            for position in game_state.get_world().get_trees().keys() {
                draw_handle.draw_texture(
                    self.tree_anim.texture.get_texture(),
                    position.x,
                    position.y,
                    Color::WHITE,
                );
            }
        }

        draw_handle.draw_fps(self.width - 100, 10);
    }
}
