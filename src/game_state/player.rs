use raylib::ffi::Rectangle;

use crate::{
    constants::{
        self,
        sizes::{INVENTORY_SIZE_START, PLAYER_HEIGHT, PLAYER_WIDTH},
    },
    input_handler::InputEvent,
    inventory::{Inventory, item::Item},
    utils::can_move,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub inventory: Inventory,
    pub direction: Direction,
}

impl Player {
    pub fn new() -> Player {
        Player {
            x: 50,
            y: 50,
            inventory: Inventory::new(INVENTORY_SIZE_START),
            direction: Direction::Down,
        }
    }

    pub fn move_player(&mut self, input: &InputEvent) {
        if !can_move(self.x, self.y, input) {
            return;
        }

        match input {
            InputEvent::Up => {
                self.y -= constants::speeds::PLAYER_MOVE_SPEED;
                self.direction = Direction::Up;
            }

            InputEvent::Down => {
                self.y += constants::speeds::PLAYER_MOVE_SPEED;
                self.direction = Direction::Down;
            }

            InputEvent::Left => {
                self.x -= constants::speeds::PLAYER_MOVE_SPEED;
                self.direction = Direction::Left;
            }

            InputEvent::Right => {
                self.x += constants::speeds::PLAYER_MOVE_SPEED;
                self.direction = Direction::Right;
            }

            _ => {}
        }
    }

    pub fn add_item(&mut self, item: Item) {
        if self.inventory.get_items().len() + 1 < self.inventory.get_size() as usize {
            self.inventory.add_item(item);
        }
    }

    pub fn get_rec(&mut self) -> Rectangle {
        Rectangle {
            x: self.x as f32,
            y: self.y as f32,
            width: PLAYER_WIDTH as f32,
            height: PLAYER_HEIGHT as f32,
        }
    }
}
