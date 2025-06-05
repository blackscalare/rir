use raylib::ffi::Rectangle;

use crate::{
    constants::{
        sizes::{PLACEHOLDER_MEDIUM, PLACEHOLDER_SMALL},
        stats::DEFAULT_AXE_DAMAGE,
        timers::BLOB_SPAWN_TIMER,
    },
    utils::Position,
};

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

    pub fn get_rec(&self, position: &Position) -> Rectangle {
        Rectangle {
            x: position.x as f32,
            y: position.y as f32,
            width: Self::get_width(self.kind),
            height: Self::get_height(self.kind),
        }
    }

    fn get_width(item_kind: ItemKind) -> f32 {
        match item_kind {
            ItemKind::BlobSpawner => PLACEHOLDER_MEDIUM as f32,
            ItemKind::Wood => PLACEHOLDER_SMALL as f32,
            _ => 0.0,
        }
    }

    fn get_height(item_kind: ItemKind) -> f32 {
        match item_kind {
            ItemKind::BlobSpawner => PLACEHOLDER_MEDIUM as f32,
            ItemKind::Wood => PLACEHOLDER_SMALL as f32,
            _ => 0.0,
        }
    }

    pub fn is_pickup(&self) -> bool {
        match self.kind {
            ItemKind::Wood => true,
            _ => false,
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
