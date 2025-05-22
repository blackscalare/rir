use item::Item;

pub mod item;

#[derive(Debug, Clone)]
pub struct Inventory {
    size: u32,
    items: Vec<Item>,
}

impl Inventory {
    pub fn new(size: u32) -> Inventory {
        Inventory {
            size,
            items: Vec::new(),
        }
    }

    pub fn get_size(&mut self) -> u32 {
        self.size
    }

    pub fn get_items(&mut self) -> &mut Vec<Item> {
        self.items.as_mut()
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
}
