use crate::{constants, input_handler::InputEvent, utils::can_move};

pub struct Player {
    pub x: i32,
    pub y: i32,
}

impl Player {
    pub fn new() -> Player {
        Player { x: 50, y: 50 }
    }

    pub fn move_player(&mut self, input: InputEvent) {
        if !can_move(self.x, self.y, &input) {
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
}
