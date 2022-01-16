#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::WinHotkeys as HotkeyManager;

#[derive(Debug, Clone, Copy)]
pub enum Modifiers {
    Alt,
    Shift,
    Control,
    Command,
}

pub trait Hotkeys {
    type Key;
    type Id;

    fn register<H>(&self, key: Self::Key, mods: Option<Modifiers>, cb: H) -> Option<Self::Id>
    where
        H: Fn() + Send + 'static;

    fn unregister(&self, id: Self::Id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hotkeys() {
        let hotkeys = HotkeyManager::new();
        let id = hotkeys
            .register(0x42, Some(Modifiers::Alt), || {
                println!("Pressed");
            })
            .unwrap();

        std::thread::sleep(std::time::Duration::from_secs(5));
        hotkeys.unregister(id);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
