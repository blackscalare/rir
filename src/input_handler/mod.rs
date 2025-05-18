use raylib::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum InputEvent {
    Up,
    Down,
    Left,
    Right,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Enter,
    MouseLeft,
    MouseRight,
    E,
    Escape,
}

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

    if rl.is_key_down(KeyboardKey::KEY_E) {
        events.push_back(InputEvent::E);
    }

    if rl.is_key_down(KeyboardKey::KEY_ENTER) {
        events.push_back(InputEvent::Enter);
    }

    if rl.is_key_down(KeyboardKey::KEY_ONE) || rl.is_key_down(KeyboardKey::KEY_KP_1) {
        events.push_back(InputEvent::Key1);
    }

    if rl.is_key_down(KeyboardKey::KEY_ONE) || rl.is_key_down(KeyboardKey::KEY_KP_1) {
        events.push_back(InputEvent::Key1);
    }

    if rl.is_key_down(KeyboardKey::KEY_TWO) || rl.is_key_down(KeyboardKey::KEY_KP_2) {
        events.push_back(InputEvent::Key2);
    }

    if rl.is_key_down(KeyboardKey::KEY_THREE) || rl.is_key_down(KeyboardKey::KEY_KP_3) {
        events.push_back(InputEvent::Key3);
    }

    if rl.is_key_down(KeyboardKey::KEY_FOUR) || rl.is_key_down(KeyboardKey::KEY_KP_4) {
        events.push_back(InputEvent::Key4);
    }

    if rl.is_key_down(KeyboardKey::KEY_FIVE) || rl.is_key_down(KeyboardKey::KEY_KP_5) {
        events.push_back(InputEvent::Key5);
    }

    if rl.is_key_down(KeyboardKey::KEY_ONE) || rl.is_key_down(KeyboardKey::KEY_KP_1) {
        events.push_back(InputEvent::Key1);
    }

    if rl.is_key_down(KeyboardKey::KEY_ESCAPE) {
        events.push_back(InputEvent::Escape);
    }

    if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
        events.push_back(InputEvent::MouseLeft);
    }

    if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
        events.push_back(InputEvent::MouseRight);
    }

    events
}
