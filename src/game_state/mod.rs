use std::{
    collections::{HashMap, VecDeque},
    process::exit,
};

use blob::{Blob, BlobActivity};
use player::Player;
use raylib::RaylibHandle;
use world::World;

use crate::{gui::GUI, input_handler::InputEvent, inventory::item::ItemKind};

mod blob;
mod food;
pub mod player;
mod tree;
mod world;

pub struct GameState {
    world: World,
    player: Player,
    blobs: Vec<Blob>,
    last_blob_spawn: HashMap<usize, f64>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            world: World::new(),
            player: player::Player::new(),
            blobs: Vec::new(),
            last_blob_spawn: HashMap::new(),
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

    pub fn get_player(&mut self) -> &mut Player {
        &mut self.player
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
        self.world.handle_collisions(&mut self.player);
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
        let item_map = self.world.get_item_map();
        let mut spawner_indices_to_spawn = Vec::new();

        if !item_map.is_empty()
            && item_map
                .iter()
                .any(|(_, item)| item.kind == ItemKind::BlobSpawner)
        {
            for (i, (position, item)) in item_map.iter().enumerate() {
                if item.kind == ItemKind::BlobSpawner {
                    let should_spawn = {
                        if let Some(last_spawn) = self.last_blob_spawn.get(&i) {
                            let current_time = handle.get_time();
                            current_time - last_spawn >= item.get_spawn_time()
                        } else {
                            true
                        }
                    };

                    if should_spawn {
                        spawner_indices_to_spawn.push((i, position));
                    }
                }
            }
        }

        for (i, position) in spawner_indices_to_spawn {
            let current_time = handle.get_time();
            self.last_blob_spawn.insert(i, current_time);
            self.blobs.push(Blob::new(position.x, position.y));
            println!("Spawned blob at ({}, {})", position.x, position.y);
        }

        for blob in self.blobs.iter_mut() {
            match blob.get_activity() {
                BlobActivity::None => {}
                BlobActivity::Eating => blob.munch(),
                BlobActivity::Searching => blob.search(),
            }

            if blob.get_health() == 0 {
                // TODO: mark as dead
            }
        }
    }
}
