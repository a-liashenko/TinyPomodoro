use eframe::egui::Color32;
use serde::{Deserialize, Serialize};

use crate::color::ColorHex;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppStyle {
    #[serde(with = "crate::serde_helpers::color")]
    pub background: Color32,
    #[serde(with = "crate::serde_helpers::color")]
    pub foreground: Color32,

    #[serde(with = "crate::serde_helpers::color")]
    pub circle_focus: Color32,
    #[serde(with = "crate::serde_helpers::color")]
    pub circle_short_break: Color32,
    #[serde(with = "crate::serde_helpers::color")]
    pub circle_long_break: Color32,

    #[serde(with = "crate::serde_helpers::color")]
    pub rounds: Color32,
}

impl Default for AppStyle {
    fn default() -> Self {
        Self {
            background: Color32::from_hex_panic("#2f384b"),
            foreground: Color32::from_hex_panic("#f0ead6"),
            circle_focus: Color32::from_hex_panic("#f25a48"),
            circle_short_break: Color32::from_hex_panic("#68d6ce"),
            circle_long_break: Color32::from_hex_panic("#8bbd78"),
            rounds: Color32::from_hex_panic("#fceea7"),
        }
    }
}
