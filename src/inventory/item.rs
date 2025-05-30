#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    // TODO: expand into something more than enum so BlobSpawners can have their own
    //       spawn timers
    BlobSpawner,
    Axe,
}

impl Item {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InventoryItem {
    pub item: Item,
    pub hotbar_slot: Option<u32>,
}

impl InventoryItem {
    pub fn new(item: Item, hotbar_slot: u32) -> Self {
        InventoryItem {
            item,
            hotbar_slot: Some(hotbar_slot),
        }
    }

    pub fn from_item(item: Item) -> Self {
        InventoryItem {
            item,
            hotbar_slot: None,
        }
    }
}
