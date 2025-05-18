mod constants;
mod game_state;
mod gui;
mod input_handler;
mod renderer;
mod utils;
use crate::config::reload_config;
use config::get_config;
use gui::GUI;
use input_handler::InputEvent;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

use std::collections::VecDeque;

use crate::input_handler::poll_inputs;
mod config;

fn main() {
    let _watcher = watch_config_changes();
    let cfg = get_config();
    let (mut handle, thread) = raylib::init()
        .size(
            constants::sizes::WINDOW_WIDTH,
            constants::sizes::WINDOW_HEIGHT,
        )
        .title(&cfg.window_title)
        .build();

    handle.set_target_fps(cfg.target_fps);
    let renderer = renderer::Renderer::new();
    let mut game_state = game_state::GameState::new();
    let mut gui = GUI::new();

    println!("Finished initiating, starting {} main loop", cfg.game_name);
    while !handle.window_should_close() {
        let mut input_events: VecDeque<InputEvent> = poll_inputs(&handle);
        game_state.update(&mut handle, &mut input_events);
        gui.update(&mut input_events);
        renderer.update(&mut handle, &thread, &game_state, &gui);

        input_events.clear();
    }
}

fn watch_config_changes() -> RecommendedWatcher {
    let mut watcher = notify::recommended_watcher(|res| {
        if let Ok(event) = res {
            println!("🔄 Config file changed: {:?}", event);
            reload_config();
        }
    })
    .expect("Failed to create watcher");

    watcher
        .watch(Path::new("config.toml"), RecursiveMode::NonRecursive)
        .expect("Failed to watch config.toml");

    watcher
}
