use crate::config::get_config;

pub struct Hotbar {
    pub size: u32,
    pub selected_rect: u32,
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
