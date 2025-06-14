use std::collections::HashSet;

use item::InventoryItem;

use crate::config::get_config;

pub mod item;

#[derive(Debug, Clone)]
pub struct Inventory {
    size: u32,
    items: Vec<InventoryItem>,
    selected_hotbar_item: Option<InventoryItem>,
}

impl Inventory {
    pub fn new(size: u32, start_items: Option<Vec<InventoryItem>>) -> Inventory {
        if let Some(items) = start_items {
            Inventory {
                size,
                items,
                selected_hotbar_item: None,
            }
        } else {
            Inventory {
                size,
                items: Vec::new(),
                selected_hotbar_item: None,
            }
        }
    }

    pub fn get_size(&mut self) -> u32 {
        self.size
    }

    pub fn get_items(&mut self) -> &mut Vec<InventoryItem> {
        self.items.as_mut()
    }

    pub fn add_item(&mut self, mut item: InventoryItem) {
        if let Some(slot) = self.get_next_free_hotbar_slot() {
            item.hotbar_slot = Some(slot);
            self.selected_hotbar_item = Some(item);
        }

        self.items.push(item);
    }

    fn get_next_free_hotbar_slot(&mut self) -> Option<u32> {
        let used_slots: HashSet<u32> = self
            .items
            .iter()
            .filter_map(|item| item.hotbar_slot)
            .collect();

        (1..=get_config().hotbar_size).find(|slot| !used_slots.contains(slot))
    }

    pub fn set_selected_item_from_hotbar(&mut self, index: u32) {
        if let Some(item) = self.items.get(index as usize) {
            self.selected_hotbar_item = Some(*item);
        }
    }

    pub fn get_selected_hotbar_item(&mut self) -> Option<InventoryItem> {
        self.selected_hotbar_item
    }

    pub fn remove_selected_item(&mut self) {
        if let Some(selected_item) = self.selected_hotbar_item {
            self.items.retain(|item| *item != selected_item);
            self.selected_hotbar_item = None;
        }
    }
}
