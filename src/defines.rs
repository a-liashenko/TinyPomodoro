pub const APP_NAME: &str = "Tiny Pomodoro";
pub const APP_CONFIG_NAME: &str = "tiny_pomodoro.yaml";

// Thanks https://feathericons.com/ for icons
pub mod icons {
    pub const ICON_SETTINGS: &[u8] = include_bytes!("../misc/settings.svg");

    pub const ICON_PLAY: &[u8] = include_bytes!("../misc/play-circle.svg");
    pub const ICON_PAUSE: &[u8] = include_bytes!("../misc/pause-circle.svg");

    pub const ICON_VOLUME_ON: &[u8] = include_bytes!("../misc/volume-2.svg");
    pub const ICON_VOLUME_OFF: &[u8] = include_bytes!("../misc/volume-x.svg");

    pub const ICON_SKIP: &[u8] = include_bytes!("../misc/skip-forward.svg");
    pub const ICON_RESET: &[u8] = include_bytes!("../misc/refresh-ccw.svg");

    pub const ICON_MINIMIZE: &[u8] = include_bytes!("../misc/minus.svg");
    pub const ICON_CLOSE: &[u8] = include_bytes!("../misc/close.svg");
    pub const ICON_PIN: &[u8] = include_bytes!("../misc/pin-outline.svg");
    pub const ICON_PIN_OFF: &[u8] = include_bytes!("../misc/pin-off-outline.svg");
}

// Thanks https://fonts.google.com/ for fonts
pub mod fonts {
    pub const FONT_ROBOTO: &[u8] = include_bytes!("../misc/Roboto-Light.ttf");
    pub const FONT_ROBOTO_MONO: &[u8] = include_bytes!("../misc/RobotoMono-Light.ttf");
}

// Thanks https://mixkit.co/free-sound-effects/notification/ for notification
pub mod sound {
    pub const SOUND_NOTIFY: &[u8] = include_bytes!("../misc/sound.wav");
}
