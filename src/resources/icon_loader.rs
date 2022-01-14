use super::icon::Icon;
use anyhow::Result;
use eframe::epi::TextureAllocator;

use crate::defines::icons::*;

pub struct Icons {
    pub settings: Icon,

    pub play: Icon,
    pub pause: Icon,

    pub volume_on: Icon,
    pub volume_off: Icon,

    pub skip: Icon,

    pub reset: Icon,

    pub minimize: Icon,
    pub close: Icon,
    pub pin_on: Icon,
    pub pin_off: Icon,
}

impl Icons {
    pub fn preload(alloc: &impl TextureAllocator) -> Result<Self> {
        let this = Self {
            play: Icon::from_svg(ICON_PLAY, (64, 64), alloc)?,
            pause: Icon::from_svg(ICON_PAUSE, (64, 64), alloc)?,

            settings: Icon::from_svg(ICON_SETTINGS, (28, 28), alloc)?,

            volume_on: Icon::from_svg(ICON_VOLUME_ON, (28, 28), alloc)?,
            volume_off: Icon::from_svg(ICON_VOLUME_OFF, (28, 28), alloc)?,

            skip: Icon::from_svg(ICON_SKIP, (28, 28), alloc)?,

            reset: Icon::from_svg(ICON_RESET, (18, 18), alloc)?,

            close: Icon::from_svg(ICON_CLOSE, (21, 21), alloc)?,
            minimize: Icon::from_svg(ICON_MINIMIZE, (21, 21), alloc)?,
            pin_on: Icon::from_svg(ICON_PIN, (21, 21), alloc)?,
            pin_off: Icon::from_svg(ICON_PIN_OFF, (21, 21), alloc)?,
        };

        Ok(this)
    }
}
