use std::{collections::VecDeque, process::exit};

use blob::{Blob, BlobActivity};
use player::Player;
use raylib::RaylibHandle;
use world::World;

use crate::input_handler::InputEvent;

mod blob;
mod food;
mod player;
mod tree;
mod world;

pub struct GameState {
    world: World,
    player: Player,
    blobs: Vec<Blob>,
    last_blob_spawn: f64,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            world: World::new(),
            player: player::Player::new(),
            blobs: Vec::new(),
            last_blob_spawn: 0.0,
        }
    }

    pub fn update(&mut self, handle: &mut RaylibHandle, input_events: &mut VecDeque<InputEvent>) {
        for event in input_events {
            self.handle_input(event);
        }

        self.handle_blobs(handle);
    }

    pub fn get_world(&self) -> &World {
        &self.world
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn get_blobs(&self) -> &Vec<Blob> {
        &self.blobs
    }

    pub fn get_blobs_mut(&mut self) -> &mut Vec<Blob> {
        &mut self.blobs
    }

    fn handle_input(&mut self, input: &InputEvent) {
        if *input == InputEvent::Escape {
            exit(0);
        }
        self.move_player(input);
    }

    fn move_player(&mut self, input: &InputEvent) {
        self.player.move_player(input);
    }

    pub fn handle_blobs(&mut self, handle: &mut RaylibHandle) {
        if self.blobs.is_empty() || self.should_spawn_blob(handle) {
            println!("Creating new blob!");
            self.blobs.push(Blob::new(self.player.x, self.player.y));
        }

        for blob in self.blobs.iter_mut() {
            match blob.get_activity() {
                BlobActivity::None => {}

                BlobActivity::Eating => {
                    blob.munch();
                }

                BlobActivity::Searching => {
                    blob.search();
                }
            }

            // TODO use a dead flag and start a counter before it's removed
            if blob.get_health() == 0 {
                // TODO remove
            }
        }
    }

    fn should_spawn_blob(&mut self, handle: &RaylibHandle) -> bool {
        let current_time = handle.get_time();
        // TODO constants::timers::BLOB_SPAWN_TIMER
        if current_time - self.last_blob_spawn >= 5.0 {
            self.last_blob_spawn = current_time;
            return true;
        }

        false
    }
}
