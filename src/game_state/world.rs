use std::collections::{HashMap, hash_map::ValuesMut};

use crate::{inventory::item::Item, utils::Position};

use super::tree::Tree;

pub struct World {
    trees: HashMap<Position, Tree>,
    items: HashMap<Position, Item>,
    cells: Vec<Cell>,
}

enum CellContent {
    Tree(Tree),
    Item(Item),
}

pub struct Cell {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    content: HashMap<Position, CellContent>,
}

impl World {
    pub fn new() -> World {
        World {
            trees: Self::init_trees(),
            items: HashMap::new(),
            cells: vec![Cell {
                x: 0,
                y: 0,
                width: 1280,
                height: 720,
                content: HashMap::from([(
                    Position { x: 100, y: 100 },
                    CellContent::Tree(Tree::new(100, 100)),
                )]),
            }],
        }
    }

    fn init_trees() -> HashMap<Position, Tree> {
        HashMap::from([(Position { x: 100, y: 100 }, Tree::new(100, 100))])
    }

    pub fn get_trees(&mut self) -> ValuesMut<'_, Position, Tree> {
        self.trees.values_mut()
    }

    pub fn get_tree_map(&self) -> &HashMap<Position, Tree> {
        &self.trees
    }

    pub fn get_item_map(&self) -> &HashMap<Position, Item> {
        &self.items
    }

    pub fn place_item(&mut self, position: Position, item: Item) {
        self.items.insert(position, item);
    }
}
