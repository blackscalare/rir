use crate::constants::stats::SMALL_TREE_HEALTH;

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
}
