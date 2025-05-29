use item::{InventoryItem, Item};

pub mod item;

#[derive(Debug, Clone)]
pub struct Inventory {
    size: u32,
    items: Vec<InventoryItem>,
}

impl Inventory {
    pub fn new(size: u32, start_items: Option<Vec<InventoryItem>>) -> Inventory {
        if let Some(items) = start_items {
            Inventory { size, items }
        } else {
            Inventory {
                size,
                items: Vec::new(),
            }
        }
    }

    pub fn get_size(&mut self) -> u32 {
        self.size
    }

    pub fn get_items(&mut self) -> &mut Vec<InventoryItem> {
        self.items.as_mut()
    }

    pub fn add_item(&mut self, item: InventoryItem) {
        self.items.push(item);
    }
}
