use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{fs, sync::RwLock};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub target_fps: u32,
    pub window_title: String,
    pub game_name: String,
    pub blob_health_enabled: bool,
    pub hotbar_size: u32,
    pub debug: bool,
}

// Global config behind RwLock
static CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| {
    let config = load_config("config.toml").unwrap_or_else(|e| {
        eprintln!("Failed to load config: {e}");
        std::process::exit(1);
    });
    RwLock::new(config)
});

// Public getter for read-only use
pub fn get_config() -> Config {
    CONFIG.read().unwrap().clone()
}

// Call this to reload the config from disk
pub fn reload_config() {
    match load_config("config.toml") {
        Ok(new_config) => {
            *CONFIG.write().unwrap() = new_config;
            println!("✅ Config reloaded.");
        }
        Err(e) => {
            eprintln!("❌ Failed to reload config: {e}");
        }
    }
}

fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
