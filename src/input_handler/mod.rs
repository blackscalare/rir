use raylib::prelude::*;
use std::collections::VecDeque;
pub enum InputEvent {
    Up,
    Down,
    Left,
    Right,
}

//struct InputHandler {}

//impl InputHandler {
pub fn poll_inputs(rl: &RaylibHandle) -> VecDeque<InputEvent> {
    let mut events = VecDeque::new();

    if rl.is_key_down(KeyboardKey::KEY_W) {
        events.push_back(InputEvent::Up);
    }

    if rl.is_key_down(KeyboardKey::KEY_D) {
        events.push_back(InputEvent::Right);
    }

    if rl.is_key_down(KeyboardKey::KEY_S) {
        events.push_back(InputEvent::Down);
    }

    if rl.is_key_down(KeyboardKey::KEY_A) {
        events.push_back(InputEvent::Left);
    }

    events
}
//}
