use std::collections::HashMap;

use crate::{
    constants::{
        extensions::{GIF_EXTENSION, PNG_EXTENSION},
        files::{
            AXE_GIF, PLACEHOLDER_PNG, PLAYER_DOWN_GIF, PLAYER_LEFT_GIF, PLAYER_RIGHT_GIF,
            PLAYER_UP_GIF, SMALL_BLOB_GIF, SMALL_TREE_GIF,
        },
        sizes::{PLACEHOLDER_LARGE, PLACEHOLDER_MEDIUM, PLACEHOLDER_SMALL},
    },
    game_state::player::Direction,
    renderer::texture_handler::AnimatedTexture,
};

pub enum Animations {
    Single(Animation),
    Multiple(HashMap<Direction, Animation>),
}

#[derive(PartialEq, Eq, Hash)]
pub enum AnimationSource {
    Tree,
    Blob,
    Player,
    Axe,
    BlobSpawner,
    PlaceholderSmall,
    PlaceholderMedium,
    PlaceholderLarge,
}

pub struct Animation {
    pub texture: AnimatedTexture,
    pub current_frame: i32,
    pub frame_timer: f32,
    pub frame_delay: f32,
}

pub struct AnimationHandler {
    pub animations: HashMap<AnimationSource, Animations>,
}

impl AnimationHandler {
    pub fn new() -> Self {
        Self {
            animations: Self::generate_animations(),
        }
    }

    fn generate_animations() -> HashMap<AnimationSource, Animations> {
        unsafe {
            HashMap::from([
                (
                    AnimationSource::Tree,
                    Animations::Single(Animation::new_from_memory(
                        SMALL_TREE_GIF,
                        GIF_EXTENSION,
                        0.2,
                    )),
                ),
                (
                    AnimationSource::Blob,
                    Animations::Single(Animation::new_from_memory(
                        SMALL_BLOB_GIF,
                        GIF_EXTENSION,
                        0.2,
                    )),
                ),
                (
                    AnimationSource::Player,
                    Animations::Multiple(HashMap::from([
                        (
                            Direction::Up,
                            Animation::new_from_memory(PLAYER_UP_GIF, GIF_EXTENSION, 0.2),
                        ),
                        (
                            Direction::Left,
                            Animation::new_from_memory(PLAYER_LEFT_GIF, GIF_EXTENSION, 0.2),
                        ),
                        (
                            Direction::Right,
                            Animation::new_from_memory(PLAYER_RIGHT_GIF, GIF_EXTENSION, 0.2),
                        ),
                        (
                            Direction::Down,
                            Animation::new_from_memory(PLAYER_DOWN_GIF, GIF_EXTENSION, 0.2),
                        ),
                    ])),
                ),
                (
                    AnimationSource::Axe,
                    Animations::Single(Animation::new_from_memory(AXE_GIF, GIF_EXTENSION, 0.2)),
                ),
                (
                    AnimationSource::BlobSpawner,
                    Animations::Single(Animation::new_from_memory(
                        PLACEHOLDER_PNG,
                        PNG_EXTENSION,
                        0.0,
                    )),
                ),
                (
                    AnimationSource::PlaceholderSmall,
                    Animations::Single(Animation::new_placeholder(
                        PLACEHOLDER_SMALL as u32,
                        PLACEHOLDER_SMALL as u32,
                    )),
                ),
                (
                    AnimationSource::PlaceholderMedium,
                    Animations::Single(Animation::new_placeholder(
                        PLACEHOLDER_MEDIUM as u32,
                        PLACEHOLDER_MEDIUM as u32,
                    )),
                ),
                (
                    AnimationSource::PlaceholderLarge,
                    Animations::Single(Animation::new_placeholder(
                        PLACEHOLDER_LARGE as u32,
                        PLACEHOLDER_LARGE as u32,
                    )),
                ),
            ])
        }
    }
}

impl Animation {
    // pub unsafe fn new(texture_path: &str, frame_delay: f32) -> Self {
    //     Self {
    //         texture: AnimatedTexture::new(texture_path),
    //         current_frame: 0,
    //         frame_timer: 0.0,
    //         frame_delay,
    //     }
    // }

    pub unsafe fn new_from_memory(data: &[u8], extension: &str, frame_delay: f32) -> Self {
        Self {
            texture: AnimatedTexture::new_from_memory(data, extension),
            current_frame: 0,
            frame_timer: 0.0,
            frame_delay,
        }
    }

    pub unsafe fn new_placeholder(width: u32, height: u32) -> Self {
        Self {
            texture: AnimatedTexture::new_placeholder(width, height),
            current_frame: 0,
            frame_timer: 0.0,
            frame_delay: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.texture.frame_count() <= 1 {
            return;
        }

        self.frame_timer += delta_time;
        if self.frame_timer >= self.frame_delay {
            self.frame_timer = 0.0;
            self.current_frame = (self.current_frame + 1) % self.texture.frame_count();
        }

        self.texture.update_texture_to_frame(self.current_frame);
    }

    // pub fn get_frame_source_rect(&self) -> raylib::core::math::Rectangle {
    //     let (w, h) = self.texture.frame_size();
    //
    //     println!("{} {}", w, h);
    //     raylib::core::math::Rectangle {
    //         x: 0.0,
    //         y: (self.current_frame * h) as f32,
    //         width: w as f32,
    //         height: h as f32,
    //     }
    // }
}
