use crate::{
    constants::sizes::WINDOW_WIDTH,
    game_state::{GameState, player::Direction},
    gui::GUI,
};
use animation_handler::{AnimationHandler, AnimationSource, Animations};
use raylib::prelude::*;
mod animation_handler;
mod texture_handler;

pub struct Renderer {
    animations: AnimationHandler,
}

impl Renderer {
    pub fn new() -> Renderer {
        unsafe {
            Renderer {
                animations: AnimationHandler::new(),
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

        gui.draw(&mut draw_handle);

        let player = game_state.get_player();

        self.update_and_draw_animation(
            AnimationSource::Player,
            Some(&player.direction),
            player.x,
            player.y,
            Color::WHITE,
            &mut draw_handle,
        );

        for blob in game_state.get_blobs() {
            self.update_and_draw_animation(
                AnimationSource::Blob,
                None,
                blob.x,
                blob.y,
                blob.color,
                &mut draw_handle,
            );
        }

        if !game_state.get_world().get_tree_map().is_empty() {
            for position in game_state.get_world().get_tree_map().keys() {
                self.update_and_draw_animation(
                    AnimationSource::Tree,
                    None,
                    position.x,
                    position.y,
                    Color::WHITE,
                    &mut draw_handle,
                );
            }
        }

        draw_handle.draw_fps(WINDOW_WIDTH - 100, 10);
    }

    fn update_and_draw_animation(
        &mut self,
        source: AnimationSource,
        direction: Option<&Direction>,
        x: i32,
        y: i32,
        color: Color,
        draw_handle: &mut RaylibDrawHandle,
    ) {
        match self.animations.animations.get_mut(&source) {
            Some(Animations::Single(animation)) => {
                animation.update(draw_handle.get_frame_time());
                draw_handle.draw_texture(animation.texture.get_texture(), x, y, color);
            }
            Some(Animations::Multiple(direction_map)) => {
                if let Some(dir) = direction {
                    if let Some(animation) = direction_map.get_mut(dir) {
                        animation.update(draw_handle.get_frame_time());
                        draw_handle.draw_texture(animation.texture.get_texture(), x, y, color);
                    }
                }
            }
            _ => {
                panic!("Animation not found or direction missing!");
            }
        }
    }
}
