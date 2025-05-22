use std::collections::HashMap;

use crate::utils::Position;

use super::tree::Tree;

pub struct World {
    trees: HashMap<Position, Tree>,
}

impl World {
    pub fn new() -> World {
        World {
            trees: Self::init_trees(),
        }
    }

    fn init_trees() -> HashMap<Position, Tree> {
        HashMap::from([(Position { x: 100, y: 100 }, Tree::new(100, 100))])
    }

    pub fn get_trees(&self) -> &HashMap<Position, Tree> {
        &self.trees
    }
}
