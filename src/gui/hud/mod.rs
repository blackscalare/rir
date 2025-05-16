pub mod hotbar;
use crate::gui::hud::hotbar::Hotbar;

pub struct HUD {
    pub hotbar: Hotbar,
}

impl HUD {
    pub fn new() -> HUD {
        HUD {
            hotbar: Hotbar::new(),
        }
    }
}
