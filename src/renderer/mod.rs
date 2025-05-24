use std::collections::HashMap;

use crate::constants::extensions::GIF_EXTENSION;
use crate::constants::files::{
    PLAYER_DOWN_GIF, PLAYER_LEFT_GIF, PLAYER_RIGHT_GIF, PLAYER_UP_GIF, SMALL_BLOB_GIF,
    SMALL_TREE_GIF,
};
use crate::constants::sizes::{PLAYER_HEIGHT, PLAYER_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::game_state::GameState;
use crate::game_state::player::Direction;
use crate::gui::GUI;
use crate::renderer::animation_handler::AnimationHandler;
use raylib::prelude::*;
mod animation_handler;
mod texture_handler;

pub struct Renderer {
    width: i32,
    height: i32,
    tree_anim: AnimationHandler,
    blob_anim: AnimationHandler,
    player_anim: HashMap<Direction, AnimationHandler>,
}

impl Renderer {
    pub fn new() -> Renderer {
        unsafe {
            let tree_anim = AnimationHandler::new_from_memory(SMALL_TREE_GIF, GIF_EXTENSION, 0.2);
            let blob_anim = AnimationHandler::new_from_memory(SMALL_BLOB_GIF, GIF_EXTENSION, 0.2);
            let player_anim = HashMap::from([
                (
                    Direction::Up,
                    AnimationHandler::new_from_memory(PLAYER_UP_GIF, GIF_EXTENSION, 0.2),
                ),
                (
                    Direction::Left,
                    AnimationHandler::new_from_memory(PLAYER_LEFT_GIF, GIF_EXTENSION, 0.2),
                ),
                (
                    Direction::Right,
                    AnimationHandler::new_from_memory(PLAYER_RIGHT_GIF, GIF_EXTENSION, 0.2),
                ),
                (
                    Direction::Down,
                    AnimationHandler::new_from_memory(PLAYER_DOWN_GIF, GIF_EXTENSION, 0.2),
                ),
            ]);
            Renderer {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                tree_anim,
                blob_anim,
                player_anim,
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

        // draw_handle.draw_rectangle(
        //     game_state.get_player().x,
        //     game_state.get_player().y,
        //     PLAYER_WIDTH,
        //     PLAYER_HEIGHT,
        //     Color::BLACK,
        // );

        if let Some(pah) = self.player_anim.get(&game_state.get_player().direction) {
            draw_handle.draw_texture(
                pah.texture.get_texture(),
                game_state.get_player().x,
                game_state.get_player().y,
                Color::WHITE,
            );
        }

        gui.draw(&mut draw_handle);

        for blob in game_state.get_blobs() {
            self.blob_anim.update(draw_handle.get_frame_time());
            draw_handle.draw_texture(
                self.blob_anim.texture.get_texture(),
                blob.x,
                blob.y,
                blob.color,
            );
        }

        if !game_state.get_world().get_tree_map().is_empty() {
            self.tree_anim.update(draw_handle.get_frame_time());
            for position in game_state.get_world().get_tree_map().keys() {
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
