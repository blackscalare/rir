use crate::{constants, input_handler::InputEvent};

// pub fn center(screen_width: i32, screen_height: i32, width: i32, size: i32) -> (i32, i32) {
//     (
//         (screen_width / 2) - (width + size),
//         (screen_height / 2) - size,
//     )
// }

// TODO: should block movement on objects in world
pub fn can_move(x: i32, y: i32, input: &InputEvent) -> bool {
    match input {
        InputEvent::Up => y - 1 > 0,
        InputEvent::Down => {
            y + 1 < (constants::sizes::WINDOW_HEIGHT - constants::sizes::PLAYER_HEIGHT)
        }
        InputEvent::Left => x - 1 > 0,
        InputEvent::Right => {
            x + 1 < (constants::sizes::WINDOW_WIDTH - constants::sizes::PLAYER_WIDTH)
        }
        _ => false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
