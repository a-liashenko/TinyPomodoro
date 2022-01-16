use rodio::{OutputStream, OutputStreamHandle};
use std::io::Cursor;

use crate::defines::sound::SOUND_NOTIFY;

pub struct Notification {
    bytes: Vec<u8>,
    output: Option<(OutputStream, OutputStreamHandle)>,
}

impl Default for Notification {
    fn default() -> Self {
        Self {
            bytes: Vec::from(SOUND_NOTIFY),
            output: Default::default(),
        }
    }
}

impl Notification {
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            bytes: bytes.into(),
            ..Default::default()
        }
    }

    pub fn play(&mut self) {
        let output = OutputStream::try_default().expect("Failed to get default audio device");

        // TODO: Do we really need to clone it everytime?
        let bytes = Cursor::new(self.bytes.clone());
        output
            .1
            .play_once(bytes)
            .expect("Failed to play sound")
            .detach();
        self.output = Some(output);
    }
}
