use crate::config::{Actions, AppConfig};
use chrono::{DateTime, Duration, Utc};
use eframe::egui::{InputState, Key, Modifiers};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Binding {
    key: Key,
    mods: Modifiers,
}

impl Binding {
    #[cfg(target_os = "macos")]
    pub fn is_pressed(&self, input: &InputState) -> bool {
        input.key_down(self.key) && self.mods == input.modifiers
    }

    #[cfg(any(target_os = "windows", target_os = "linux"))]
    pub fn is_pressed(&self, input: &InputState) -> bool {
        if !input.key_down(self.key) {
            return false;
        }

        if self.mods.ctrl {
            return self.mods.ctrl == input.modifiers.ctrl;
        }

        self.mods == input.modifiers
    }
}

impl Binding {
    fn new(key: Key, mods: Modifiers) -> Self {
        Self { key, mods }
    }
}

pub struct Hotkeys {
    keys: HashMap<Actions, Binding>,
    trigger: Binding,
    triggered_at: DateTime<Utc>,
}

impl Hotkeys {
    pub fn new(_cfg: &AppConfig) -> Self {
        // TODO: Parse hotkey from config
        let mut keys = HashMap::new();
        keys.insert(
            Actions::ToggleTimer,
            Binding::new(
                Key::M,
                Modifiers {
                    ctrl: true,
                    ..Default::default()
                },
            ),
        );

        // TODO: Replace Z with option (?)
        Self {
            keys,
            trigger: Binding::new(Key::Z, Modifiers::default()),
            triggered_at: Utc::now(),
        }
    }

    // TODO: Lazy static?
    pub fn delay() -> Duration {
        Duration::milliseconds(100)
    }

    pub fn next_action(&mut self, input: &InputState) -> Option<Actions> {
        for (act, bind) in &self.keys {
            if bind.is_pressed(input) {
                // TODO: Add progressive delay
                if &self.trigger == bind && Utc::now() - self.triggered_at < Self::delay() {
                    continue;
                }

                self.trigger = *bind;
                self.triggered_at = Utc::now();
                return Some(*act);
            }
        }

        None
    }
}
