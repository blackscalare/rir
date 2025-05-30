use std::{collections::VecDeque, process::exit};

use blob::{Blob, BlobActivity};
use player::Player;
use raylib::{RaylibHandle, ffi::CheckCollisionRecs};
use world::World;

use crate::{constants::timers::BLOB_SPAWN_TIMER, gui::GUI, input_handler::InputEvent};

mod blob;
mod food;
pub mod player;
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

    pub fn update(
        &mut self,
        handle: &mut RaylibHandle,
        input_events: &mut VecDeque<InputEvent>,
        gui: &GUI,
    ) {
        unsafe {
            self.handle_player(input_events, gui);
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

    // pub fn get_blobs_mut(&mut self) -> &mut Vec<Blob> {
    //     &mut self.blobs
    // }

    unsafe fn handle_player(&mut self, input_events: &mut VecDeque<InputEvent>, gui: &GUI) {
        self.player.update(gui);
        // TODO: better collision detection
        //      move to world
        //      handle collision, problem with borrowing
        for tree in self.world.get_trees() {
            unsafe {
                let did_collide = CheckCollisionRecs(self.player.get_rec(), tree.get_rec());
                if did_collide {
                    println!("Did collide");
                    tree.take_damage(1);
                    println!("tree health {}", tree.health);
                }
            }
        }
        for event in input_events {
            self.handle_input(event);
        }
    }

    fn handle_input(&mut self, input: &InputEvent) {
        if *input == InputEvent::Escape {
            exit(0);
        }

        self.player.move_player(input);
        self.player.handle_other_input(input, &mut self.world);
    }

    pub fn handle_blobs(&mut self, handle: &mut RaylibHandle) {
        if self.should_spawn_blob(handle) && !self.world.get_item_map().is_empty() {
            println!("Creating new blob!");
            // TODO: implement BlobSpawner spawn logic
            for (position, _) in self.world.get_item_map() {
                self.blobs.push(Blob::new(position.x, position.y));
            }
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

            // TODO: use a dead flag and start a counter before it's removed
            if blob.get_health() == 0 {
                // TODO: remove
            }
        }
    }

    fn should_spawn_blob(&mut self, handle: &RaylibHandle) -> bool {
        let current_time = handle.get_time();
        if current_time - self.last_blob_spawn >= BLOB_SPAWN_TIMER {
            self.last_blob_spawn = current_time;
            return true;
        }

        false
    }
}
