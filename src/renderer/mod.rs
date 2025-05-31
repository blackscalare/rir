use crate::{
    constants::sizes::{WINDOW_HEIGHT, WINDOW_WIDTH},
    game_state::{GameState, player::Direction},
    gui::GUI,
    inventory::item::Item,
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
        draw_handle.clear_background(Color::GRAY);

        //gui.draw(&mut draw_handle, game_state);
        self.draw_gui(gui, game_state, &mut draw_handle);
        self.draw_player(game_state, &mut draw_handle);
        self.draw_blobs(game_state, &mut draw_handle);
        self.draw_world(game_state, &mut draw_handle);
        draw_handle.draw_fps(WINDOW_WIDTH - 100, 10);
    }

    fn draw_gui(&mut self, gui: &GUI, game_state: &GameState, draw_handle: &mut RaylibDrawHandle) {
        let total_width: i32 = (gui.hud.hotbar.size * (50 + 10) - 10) as i32;
        for i in 0..gui.hud.hotbar.size {
            let start_x = (WINDOW_WIDTH - total_width) / 2;
            let x_pos: i32 = start_x + i as i32 * (50 + 10);
            let y_pos = WINDOW_HEIGHT - 60;
            if gui.hud.hotbar.selected_rect - 1 == i {
                draw_handle.draw_rectangle_lines(x_pos, y_pos, 50, 50, Color::RED);
            } else {
                draw_handle.draw_rectangle_lines(x_pos, y_pos, 50, 50, Color::BLACK);
            }

            draw_handle.draw_text(
                (i + 1).to_string().as_str(),
                x_pos + 10,
                y_pos + 10,
                8,
                Color::BLACK,
            );

            if let Some(item) = game_state
                .get_player()
                .to_owned()
                .inventory
                .get_items()
                .iter()
                .find(|inv_item| inv_item.hotbar_slot == Some(i))
            {
                // TODO: better handling
                let anim_source: AnimationSource = match item.item {
                    Item::Axe => AnimationSource::Axe,
                    Item::BlobSpawner => AnimationSource::BlobSpawner,
                    Item::Wood => AnimationSource::PlaceholderSmall, // _ => AnimationSource::Axe,
                };

                self.update_and_draw_animation(
                    anim_source,
                    None,
                    x_pos,
                    y_pos,
                    Color::WHITE,
                    draw_handle,
                );
            }
        }
    }

    fn draw_player(&mut self, game_state: &GameState, draw_handle: &mut RaylibDrawHandle) {
        let player = game_state.get_player();

        self.update_and_draw_animation(
            AnimationSource::Player,
            Some(&player.direction),
            player.x,
            player.y,
            Color::WHITE,
            draw_handle,
        );
        // TODO: is_attacking + get what's selected in the hotbar
        if player.is_attacking {
            self.update_and_draw_animation(
                AnimationSource::Axe,
                Some(&player.direction),
                player.x,
                player.y,
                Color::WHITE,
                draw_handle,
            );
        }
    }

    fn draw_blobs(&mut self, game_state: &GameState, draw_handle: &mut RaylibDrawHandle) {
        for blob in game_state.get_blobs() {
            self.update_and_draw_animation(
                AnimationSource::Blob,
                None,
                blob.x,
                blob.y,
                blob.color,
                draw_handle,
            );
        }
    }

    fn draw_world(&mut self, game_state: &GameState, draw_handle: &mut RaylibDrawHandle) {
        self.draw_trees(game_state, draw_handle);
        self.draw_items(game_state, draw_handle);
    }

    fn draw_trees(&mut self, game_state: &GameState, draw_handle: &mut RaylibDrawHandle) {
        if !game_state.get_world().get_tree_map().is_empty() {
            for position in game_state.get_world().get_tree_map().keys() {
                self.update_and_draw_animation(
                    AnimationSource::Tree,
                    None,
                    position.x,
                    position.y,
                    Color::WHITE,
                    draw_handle,
                );
            }
        }
    }
    fn draw_items(&mut self, game_state: &GameState, draw_handle: &mut RaylibDrawHandle) {
        if !game_state.get_world().get_item_map().is_empty() {
            for (position, item) in game_state.get_world().get_item_map() {
                let anim_source = match *item {
                    Item::BlobSpawner => AnimationSource::BlobSpawner,
                    Item::Wood => AnimationSource::PlaceholderSmall,
                    Item::Axe => AnimationSource::Axe,
                };
                self.update_and_draw_animation(
                    anim_source,
                    None,
                    position.x,
                    position.y,
                    Color::WHITE,
                    draw_handle,
                );
            }
        }
    }

    /// Updates the animation and draws it with given parameters
    /// [`source`]: `AnimationSource` that exists in `self.animations`
    /// [`direction`]: Optinal Direction when something needs to face a certain way
    /// [`x`]: x position
    /// [`y`]: y position
    /// [`color`]: color of the texture. `Color::WHITE` for none
    /// [`draw_handle`]: `RaylibDrawHandle`
    ///
    /// # Examples
    /// ```
    /// self.update_and_draw_animation(
    ///     AnimationSource::Player,
    ///     Some(&player.direction),
    ///     player.x,
    ///     player.y,
    ///     Color::WHITE,
    ///     draw_handle,
    /// );
    /// ```
    /// ```
    /// self.update_and_draw_animation(
    ///     AnimationSource::Blob,
    ///     None,
    ///     blob.x,
    ///     blob.y,
    ///     blob.color,
    ///     draw_handle,
    /// );
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the animation or direction does not exist
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
                if let Some(dir) = direction {
                    draw_handle.draw_texture_ex(
                        animation.texture.get_texture(),
                        Vector2::new(x as f32, y as f32),
                        Self::get_rotation(dir),
                        1.0,
                        color,
                    );
                } else {
                    draw_handle.draw_texture(animation.texture.get_texture(), x, y, color);
                }
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

    fn get_rotation(direction: &Direction) -> f32 {
        match direction {
            Direction::Right => 0.0,
            Direction::Down => 90.0,
            Direction::Left => 180.0,
            Direction::Up => 270.0,
        }
    }
}
