use crate::constants::{stats::DEFAULT_AXE_DAMAGE, timers::BLOB_SPAWN_TIMER};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Item {
    pub kind: ItemKind,
    pub data: ItemData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemKind {
    BlobSpawner,
    Axe,
    Wood,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemData {
    None,
    BlobSpawnerData { spawn_timer: f64 },
    AxeData { damage: u32 },
}

impl Item {
    pub fn new(kind: ItemKind) -> Self {
        let data = match kind {
            ItemKind::BlobSpawner => ItemData::BlobSpawnerData {
                spawn_timer: BLOB_SPAWN_TIMER,
            },
            ItemKind::Axe => ItemData::AxeData {
                damage: DEFAULT_AXE_DAMAGE,
            },
            _ => ItemData::None,
        };

        Self { kind, data }
    }

    pub fn get_damage(&self) -> u32 {
        match &self.data {
            ItemData::AxeData { damage } => *damage,
            _ => 0,
        }
    }

    pub fn get_spawn_time(&self) -> f64 {
        match &self.data {
            ItemData::BlobSpawnerData { spawn_timer } => *spawn_timer,
            _ => 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
