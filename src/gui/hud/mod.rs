pub mod hotbar;
use crate::gui::hud::hotbar::Hotbar;

pub struct HUD {
    hotbar: Hotbar,
}

impl HUD {
    pub fn new() -> HUD {
        HUD {
            hotbar: Hotbar::new(),
        }
    }
}
