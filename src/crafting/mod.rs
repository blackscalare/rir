use std::{collections::HashMap, fs};

use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::inventory::item::{Item, ItemKind};

#[derive(Debug, Clone)]
struct Materials(Item, u32);

#[derive(Debug, Clone)]
pub struct Recipe {
    name: String,
    materials: Vec<Materials>,
}

impl Recipe {
    pub fn new(name: String, materials: Vec<Materials>) -> Self {
        Self { name, materials }
    }
}

pub static RECIPES: Lazy<HashMap<i32, Recipe>> =
    Lazy::new(|| load_recipes_from_file("recipes.toml"));

#[derive(Debug, Deserialize)]
struct MaterialConfig {
    kind: String,
    amount: u32,
}

#[derive(Debug, Deserialize)]
struct RecipeConfig {
    id: i32,
    name: String,
    materials: Vec<MaterialConfig>,
}

#[derive(Debug, Deserialize)]
struct RecipeFile {
    recipes: Vec<RecipeConfig>,
}

fn load_recipes_from_file(path: &str) -> HashMap<i32, Recipe> {
    let content = fs::read_to_string(path).expect("Failed to read recipe file");
    let parsed: RecipeFile = toml::from_str(&content).expect("Failed to parse recipe TOML");

    parsed
        .recipes
        .into_iter()
        .map(|r| {
            let materials = r
                .materials
                .into_iter()
                .map(|m| Materials(Item::new(ItemKind::from_str(&m.kind)), m.amount))
                .collect();
            (r.id, Recipe::new(r.name, materials))
        })
        .collect()
}
