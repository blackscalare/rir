use raylib::ffi::Rectangle;

use crate::constants::{
    sizes::{TREE_HEIGHT, TREE_WIDTH},
    stats::SMALL_TREE_HEALTH,
};

#[derive(Debug, Clone, Copy)]
pub struct Tree {
    pub x: i32,
    pub y: i32,
    pub health: u32,
    pub destroyed: bool,
}

impl Tree {
    pub fn new(x: i32, y: i32) -> Tree {
        Tree {
            x,
            y,
            health: SMALL_TREE_HEALTH,
            destroyed: false,
        }
    }

    pub fn take_damage(&mut self, damage: u32) {
        if let Some(new_health) = self.health.checked_sub(damage) {
            self.health = new_health;
        } else {
            self.destroyed = true;
        }
    }

    pub fn get_rec(&mut self) -> Rectangle {
        Rectangle {
            x: self.x as f32,
            y: self.y as f32,
            width: TREE_WIDTH as f32,
            height: TREE_HEIGHT as f32,
        }
    }
}
