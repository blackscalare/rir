use rand::{Rng, rng};
use raylib::color::Color;

use crate::{config::get_config, constants, input_handler::InputEvent, utils::can_move};

use super::food::Food;

pub enum BlobActivity {
    None,
    Searching,
    Eating,
}

pub struct Blob {
    pub x: i32,
    pub y: i32,
    pub color: Color,
    bites: Option<u32>,
    held_food: Option<Food>,
    health: u32,
    activity: BlobActivity,
    xp: u128,
}

impl Blob {
    pub fn new(x: i32, y: i32) -> Blob {
        Blob {
            x,
            y,
            color: Self::get_random_color(),
            health: 100, // TODO constants::health?::BLOB_START_HEALTH
            activity: BlobActivity::Searching,
            bites: None,
            held_food: None,
            xp: 0,
        }
    }

    fn get_random_color() -> Color {
        let mut rng = rng();
        loop {
            let r = rng.random_range(0..=255);
            let g = rng.random_range(0..=255);
            let b = rng.random_range(0..=255);

            if r != 0 || g != 0 || b != 0 {
                return Color { r, g, b, a: 255 };
            }
        }
    }

    pub fn move_blob(&mut self, input: InputEvent) {
        if !can_move(self.x, self.y, &input) {
            return;
        }

        match input {
            InputEvent::Up => {
                self.y -= constants::speeds::BLOB_MOVE_SPEED;
            }

            InputEvent::Down => {
                self.y += constants::speeds::BLOB_MOVE_SPEED;
            }

            InputEvent::Left => {
                self.x -= constants::speeds::BLOB_MOVE_SPEED;
            }

            InputEvent::Right => {
                self.x += constants::speeds::BLOB_MOVE_SPEED;
            }

            _ => {}
        }
    }

    pub fn get_activity(&mut self) -> &BlobActivity {
        &self.activity
    }

    pub fn get_health(&mut self) -> u32 {
        self.health
    }

    pub fn pickup_food(&mut self, food: Food) {
        let bites = food.num_bites();

        self.held_food = Some(food);
        self.bites = Some(bites);
    }

    pub fn munch(&mut self) {
        if let (Some(food), Some(bites)) = (&self.held_food, self.bites) {
            if bites > 0 {
                self.xp += food.xp_value();
                self.bites = Some(bites - 1);

                if self.bites == Some(0) {
                    self.held_food = None;
                    self.activity = BlobActivity::Searching;
                }
            }
        }
    }

    pub fn search(&mut self) {
        let cfg = get_config();
        // TODO search for food
        self.move_blob(Self::get_random_input());
        // Lose health every frame
        if cfg.blob_health_enabled {
            if let Some(new_health) = self.health.checked_sub(1) {
                self.health = new_health
            } else {
                // TODO mark for removal?
                //      set activity to None where they can be revived and are removed after a certain
                //      time?
                self.activity = BlobActivity::None;
            }
        }

        if self.held_food.is_some() {
            self.activity = BlobActivity::Eating;
        }
    }

    fn get_random_input() -> InputEvent {
        let mut rng = rng();
        match rng.random_range(0..4) {
            0 => InputEvent::Up,
            1 => InputEvent::Down,
            2 => InputEvent::Left,
            _ => InputEvent::Right,
        }
    }
}
