use player::Player;

use crate::input_handler::InputEvent;

mod player;

pub struct GameState {
    player: Player,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            player: player::Player::new(),
        }
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn move_player(&mut self, input: InputEvent) {
        self.player.move_player(input);
    }
}
