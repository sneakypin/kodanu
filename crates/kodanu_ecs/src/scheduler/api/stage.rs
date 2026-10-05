#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    TimeUpdate,
    PreInputUpdate,
    Startup,
    PreFixedUpdate,
    FixedUpdate,
    PostFixedUpdate,
    PreUpdate,
    Update,
    LateUpdate,
    UpdateEvent,
    PreRender,
    Render,
    PostRender,
    PostInputUpdate,
}

impl Stage {
    pub const COUNT: usize = 14;

    pub fn as_usize(self) -> usize {
        self as usize
    }
}
