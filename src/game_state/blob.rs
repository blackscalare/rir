use crate::{constants, input_handler::InputEvent, utils::can_move};

use super::food::Food;

pub enum BlobActivity {
    None,
    Searching,
    Eating,
}

pub struct Blob {
    pub x: i32,
    pub y: i32,
    bites: Option<u32>,
    held_food: Option<Food>,
    health: u32,
    activity: BlobActivity,
    xp: u128,
}

impl Blob {
    pub fn new() -> Blob {
        Blob {
            x: 0,
            y: 0,
            health: 0,
            activity: BlobActivity::None,
            bites: None,
            held_food: None,
            xp: 0,
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
        }
    }

    pub fn get_activity(&mut self) -> &BlobActivity {
        &self.activity
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
                }
            }
        }
    }

    pub fn search(&mut self) {
        // TODO search for food
        if self.held_food.is_some() {
            self.activity = BlobActivity::Eating;
        }
    }
}
