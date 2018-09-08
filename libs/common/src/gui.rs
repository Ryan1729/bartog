pub type UIId = u8;

#[derive(Debug)]
pub struct UIContext {
    pub hot: UIId,
    pub active: UIId,
    pub next_hot: UIId,
}

impl UIContext {
    pub fn new() -> Self {
        UIContext {
            hot: 0,
            active: 0,
            next_hot: 0,
        }
    }

    pub fn set_not_active(&mut self) {
        self.active = 0;
    }
    pub fn set_active(&mut self, id: UIId) {
        self.active = id;
    }
    pub fn set_next_hot(&mut self, id: UIId) {
        self.next_hot = id;
    }
    pub fn set_not_hot(&mut self) {
        self.hot = 0;
    }
    pub fn frame_init(&mut self) {
        if self.active == 0 {
            self.hot = self.next_hot;
        }
        self.next_hot = 0;
    }
}

pub struct ButtonSpec {
    pub text: String,
    pub x: u8,
    pub y: u8,
    pub w: u8,
    pub h: u8,
    pub id: UIId,
}
