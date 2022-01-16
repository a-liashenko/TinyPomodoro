use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, UnregisterHotKey, MOD_NOREPEAT};
use windows::Win32::UI::WindowsAndMessaging::{PeekMessageW, PM_REMOVE, WM_HOTKEY};

use super::actions::Action;
use super::{makedword, Mod};
use crate::{Hotkeys, Modifiers};

type Callback = Box<dyn Fn() + Send>;
type Handlers = Arc<Mutex<HashMap<u32, Callback>>>;

pub struct WinHotkeys {
    _loop: JoinHandle<()>,
    tx: Sender<Action>,
    handler: Handlers,
}

impl WinHotkeys {
    pub fn new() -> Self {
        let handler = Arc::new(Mutex::new(HashMap::new()));
        let (tx, rx) = std::sync::mpsc::channel();

        let h = handler.clone();
        let _loop = std::thread::spawn(move || Self::event_loop(rx, h));
        Self { _loop, tx, handler }
    }

    fn try_action(rx: &Receiver<Action>) -> Option<Action> {
        use std::sync::mpsc::TryRecvError;

        match rx.try_recv() {
            Ok(e) => Some(e),
            Err(e) => match e {
                TryRecvError::Empty => None,
                TryRecvError::Disconnected => Some(Action::Exit),
            },
        }
    }

    fn try_message() -> Option<u32> {
        let mut msg = Default::default();

        let exists = unsafe { PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE) };
        if !exists.as_bool() {
            return None;
        }

        if msg.message == WM_HOTKEY {
            Some(msg.lParam.0 as u32)
        } else {
            None
        }
    }

    fn event_loop(rx: Receiver<Action>, handlers: Handlers) {
        loop {
            if let Some(act) = Self::try_action(&rx) {
                match act {
                    Action::Exit => break,
                    Action::Register { key, modifier, id } => {
                        let modifier = match modifier {
                            None => MOD_NOREPEAT,
                            Some(v) => v | MOD_NOREPEAT,
                        };
                        unsafe { RegisterHotKey(None, id, modifier | MOD_NOREPEAT, key) }
                            .expect("Failed to register hotkey")
                    }
                    Action::Unregister { id } => unsafe {
                        UnregisterHotKey(None, id);
                    },
                }
            }

            if let Some(key) = Self::try_message() {
                let lock = handlers.lock().unwrap();
                if let Some(h) = lock.get(&key) {
                    h()
                } else {
                    println!("Received unknown hotkey {}", key);
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
}

impl Default for WinHotkeys {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for WinHotkeys {
    fn drop(&mut self) {
        let _ = self.tx.send(Action::Exit);
    }
}

impl Hotkeys for WinHotkeys {
    type Key = u32;
    type Id = i32;

    fn register<H>(&self, key: Self::Key, mods: Option<Modifiers>, cb: H) -> Option<Self::Id>
    where
        H: Fn() + Send + 'static,
    {
        let id = 0;
        let mods = mods.map(|e| Mod::from(e).0);
        let act = Action::register(key, id, mods);
        self.tx.send(act).expect("Hotkeys loop is dead");

        let key = match mods {
            None => makedword(0, key as u16),
            Some(v) => makedword(v as u16, key as u16),
        };
        self.handler.lock().unwrap().insert(key, Box::new(cb));

        Some(id)
    }

    fn unregister(&self, id: Self::Id) {
        self.tx
            .send(Action::Unregister { id })
            .expect("Hotkeys loop is dead");
    }
}
