//! This mod only allows the code to compile but does not provide the expected functionality

use crate::Hotkeys;

#[derive(Default)]
pub struct HotkeyManager;
impl Hotkeys for HotkeyManager {
    type Key = u32;

    type Id = i32;

    fn register<H>(
        &self,
        _key: Self::Key,
        _mods: Option<crate::Modifiers>,
        _cb: H,
    ) -> Option<Self::Id>
    where
        H: Fn() + Send + 'static,
    {
        // TODO: Replace empty placeholder
        None
    }

    fn unregister(&self, _id: Self::Id) {
        // TODO: Replace empty placeholder
    }
}

impl HotkeyManager {
    pub fn new() -> Self {
        // TODO: Replace empty placeholder
        Self {}
    }
}
