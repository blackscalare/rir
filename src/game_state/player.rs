use raylib::ffi::Rectangle;

use crate::{
    config::get_config,
    constants::{
        self,
        sizes::{INVENTORY_SIZE_START, PLAYER_HEIGHT, PLAYER_WIDTH},
    },
    gui::GUI,
    input_handler::InputEvent,
    inventory::{
        Inventory,
        item::{InventoryItem, Item},
    },
    utils::{Position, can_move},
};

use super::{GameState, world::World};

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
    pub is_attacking: bool,
    attack_timer: i32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            x: 50,
            y: 50,
            inventory: Inventory::new(
                INVENTORY_SIZE_START,
                Some(vec![
                    InventoryItem::new(Item::Axe, 0),
                    InventoryItem::new(Item::BlobSpawner, 1),
                ]),
            ),
            direction: Direction::Down,
            is_attacking: false,
            attack_timer: 0,
        }
    }

    pub fn update(&mut self, gui: &GUI) {
        if self.is_attacking {
            self.attack_timer += 1;
            if self.attack_timer >= (get_config().target_fps / 2) as i32 {
                self.is_attacking = false;
                self.attack_timer = 0;
            }
        }

        self.inventory
            .set_selected_item_from_hotbar(gui.hud.hotbar.selected_rect - 1);
    }

    pub fn handle_other_input(&mut self, input: &InputEvent, world: &mut World) {
        if *input == InputEvent::E {
            if let Some(inventory_item) = self.inventory.get_selected_hotbar_item() {
                if inventory_item.item == Item::Axe && !self.is_attacking {
                    self.is_attacking = true;
                    self.attack_timer = 0;
                }

                if inventory_item.item == Item::BlobSpawner {
                    world.place_item(
                        Position {
                            x: self.x,
                            y: self.y,
                        },
                        Item::BlobSpawner,
                    );
                    self.inventory.remove_selected_item();
                }
            }
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
            self.inventory.add_item(InventoryItem::from_item(item));
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
