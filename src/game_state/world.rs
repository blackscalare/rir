use std::collections::HashMap;

use raylib::ffi::CheckCollisionRecs;

use crate::{
    inventory::item::{Item, ItemKind},
    utils::Position,
};

use super::{player::Player, tree::Tree};

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

    pub fn get_tree_map(&self) -> &HashMap<Position, Tree> {
        &self.trees
    }

    pub fn get_item_map(&self) -> &HashMap<Position, Item> {
        &self.items
    }

    pub fn place_item(&mut self, position: Position, item: Item) {
        self.items.insert(position, item);
    }

    pub fn handle_collisions(&mut self, player: &mut Player) {
        self.handle_tree_collisions(player);
    }

    fn handle_tree_collisions(&mut self, player: &mut Player) {
        let mut trees_to_destroy: Vec<Position> = vec![];

        for (position, tree) in self.trees.iter_mut() {
            unsafe {
                let did_collide = CheckCollisionRecs(player.get_rec(), tree.get_rec());
                if did_collide && player.is_attacking {
                    println!("Attacked tree");
                    if let Some(item) = player.get_selected_hotbar_item() {
                        tree.take_damage(item.item.get_damage());
                    }
                    if tree.health == 0 {
                        trees_to_destroy.push(*position);
                    }
                }
            }
        }

        self.destroy_trees(trees_to_destroy);
    }

    fn destroy_trees(&mut self, trees_to_destroy: Vec<Position>) {
        for position in trees_to_destroy {
            self.trees.remove(&position);
            self.items.insert(position, Item::new(ItemKind::Wood));
        }
    }
}
