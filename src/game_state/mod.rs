use blob::{Blob, BlobActivity};
use player::Player;

use crate::input_handler::InputEvent;

mod blob;
mod food;
mod player;

pub struct GameState {
    player: Player,
    blobs: Vec<Blob>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            player: player::Player::new(),
            blobs: Vec::new(),
        }
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn move_player(&mut self, input: InputEvent) {
        self.player.move_player(input);
    }

    pub fn handle_blobs(&mut self) {
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
        }
    }
}
