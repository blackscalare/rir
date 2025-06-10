mod config;
mod constants;
mod game_state;
mod gui;
mod input_handler;
mod inventory;
mod renderer;
mod utils;
mod crafting;
use crate::config::reload_config;
use config::get_config;
use constants::sizes::{WINDOW_HEIGHT, WINDOW_WIDTH};
use gui::GUI;
use input_handler::InputEvent;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

use std::collections::VecDeque;

use crate::input_handler::poll_inputs;

fn main() {
    let _watcher = watch_config_changes();
    let cfg = get_config();
    let (mut handle, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title(&cfg.window_title)
        .build();

    handle.set_target_fps(cfg.target_fps);
    let mut renderer = renderer::Renderer::new();
    let mut game_state = game_state::GameState::new();
    let mut gui = GUI::new();

    println!("Finished initiating, starting {} main loop", cfg.game_name);
    while !handle.window_should_close() {
        let mut input_events: VecDeque<InputEvent> = poll_inputs(&handle);
        game_state.update(&mut handle, &mut input_events, &gui);
        gui.update(&mut input_events);
        renderer.update(&mut handle, &thread, &mut game_state, &gui);

        input_events.clear();
    }
}

fn watch_config_changes() -> RecommendedWatcher {
    fn reload_on_modify(res: Result<notify::Event, notify::Error>) {
        if let Ok(event) = res {
            if event.kind.is_modify() {
                println!("🔄 Config file changed: {:?}", event);
                reload_config();
            }
        }
    }
    let mut watcher =
        notify::recommended_watcher(reload_on_modify).expect("Failed to create watcher");

    watcher
        .watch(Path::new("config.toml"), RecursiveMode::NonRecursive)
        .expect("Failed to watch config.toml");

    watcher
}
