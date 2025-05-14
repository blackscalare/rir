use crate::{constants, input_handler::InputEvent};

pub struct Player {
    pub x: i32,
    pub y: i32,
}

impl Player {
    pub fn new() -> Player {
        Player { x: 50, y: 50 }
    }

    pub fn move_player(&mut self, input: InputEvent) {
        if !self.can_move(&input) {
            return;
        }

        match input {
            InputEvent::Up => {
                self.y -= constants::speeds::PLAYER_MOVE_SPEED;
            }

            InputEvent::Down => {
                self.y += constants::speeds::PLAYER_MOVE_SPEED;
            }

            InputEvent::Left => {
                self.x -= constants::speeds::PLAYER_MOVE_SPEED;
            }

            InputEvent::Right => {
                self.x += constants::speeds::PLAYER_MOVE_SPEED;
            }
        }
    }

    fn can_move(&self, input: &InputEvent) -> bool {
        match input {
            InputEvent::Up => self.y - 1 > 0,
            InputEvent::Down => {
                self.y + 1 < (constants::sizes::WINDOW_HEIGHT - constants::sizes::PLAYER_HEIGHT)
            }
            InputEvent::Left => self.x - 1 > 0,
            InputEvent::Right => {
                self.x + 1 < (constants::sizes::WINDOW_WIDTH - constants::sizes::PLAYER_WIDTH)
            }
        }
    }
}
