use std::collections::VecDeque;

use hud::HUD;
use raylib::prelude::RaylibDrawHandle;

use crate::input_handler::InputEvent;

mod hud;

pub struct GUI {
    hud: HUD,
}

impl GUI {
    pub fn new() -> GUI {
        GUI { hud: HUD::new() }
    }

    pub fn update(&self, input_events: &mut VecDeque<InputEvent>) {}

    pub fn draw(&self, draw_handle: &RaylibDrawHandle) {
        //draw_handle.draw_rectangle_rec(rec, color);
    }
}
