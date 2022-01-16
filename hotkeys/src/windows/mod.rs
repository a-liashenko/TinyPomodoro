mod actions;
mod utils;
mod win_hotkeys;

use crate::Modifiers;
pub use win_hotkeys::WinHotkeys;
use windows::Win32::UI::Input::KeyboardAndMouse::{MOD_CONTROL, MOD_WIN};

pub struct Mod(pub u32);
impl From<Modifiers> for Mod {
    fn from(val: Modifiers) -> Self {
        use windows::Win32::UI::Input::KeyboardAndMouse::{MOD_ALT, MOD_SHIFT};
        let m = match val {
            Modifiers::Alt => MOD_ALT,
            Modifiers::Shift => MOD_SHIFT,
            Modifiers::Control => MOD_CONTROL,
            Modifiers::Command => MOD_WIN,
        };
        Mod(m)
    }
}

pub const fn makedword(lo: u16, hi: u16) -> u32 {
    ((lo as u32 & 0xffff) | ((hi as u32 & 0xffff) << 16)) as _
}
