use crate::config::get_config;

pub struct Hotbar {
    size: u32,
    selected_rect: u32,
}

impl Hotbar {
    pub fn new() -> Hotbar {
        let cfg = get_config();
        Hotbar {
            size: cfg.hotbar_size,
            selected_rect: 1000,
        }
    }
}
