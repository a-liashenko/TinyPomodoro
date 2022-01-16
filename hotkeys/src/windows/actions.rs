pub enum Action {
    Register {
        modifier: Option<u32>,
        key: u32,
        id: i32,
    },
    Unregister {
        id: i32,
    },
    Exit,
}

impl Action {
    pub fn register(key: u32, id: i32, modifier: Option<u32>) -> Self {
        Self::Register { key, id, modifier }
    }
}
