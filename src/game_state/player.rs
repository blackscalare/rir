use crate::{
    constants::{self, sizes::INVENTORY_SIZE_START},
    input_handler::InputEvent,
    inventory::{Inventory, item::Item},
    utils::can_move,
};

#[derive(Debug, Clone)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub inventory: Inventory,
}

impl Player {
    pub fn new() -> Player {
        Player {
            x: 50,
            y: 50,
            inventory: Inventory::new(INVENTORY_SIZE_START),
        }
    }

    pub fn move_player(&mut self, input: &InputEvent) {
        if !can_move(self.x, self.y, input) {
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

            _ => {}
        }
    }

    pub fn add_item(&mut self, item: Item) {
        if self.inventory.get_items().len() + 1 < self.inventory.get_size() as usize {
            self.inventory.add_item(item);
        }
    }
}
