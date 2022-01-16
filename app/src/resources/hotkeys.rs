use crate::config::{Actions, AppConfig};
use hotkeys::{HotkeyManager, Modifiers};
use std::sync::mpsc::Receiver;

pub struct Hotkeys {
    _manager: HotkeyManager,
    rx: Receiver<Actions>,
}

impl Hotkeys {
    pub fn new(_cfg: &AppConfig) -> Self {
        use hotkeys::Hotkeys;

        let (tx, rx) = std::sync::mpsc::channel();
        let _manager = HotkeyManager::default();

        _manager.register(0xDD, Some(Modifiers::Alt), move || {
            let _ = tx.send(Actions::ToggleTimer);
        });

        Self { _manager, rx }
    }

    pub fn next_action(&mut self) -> Option<Actions> {
        self.rx.try_recv().ok()
    }
}
