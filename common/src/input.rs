pub trait Input {
    fn left(&self) -> bool;
    fn right(&self) -> bool;
    fn up(&self) -> bool;
    fn down(&self) -> bool;
    fn action(&self) -> bool;
}

/// A struct representing the current state of the input buttons
#[derive(Default, Clone, Copy, Debug)]
pub struct BasicInput {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub action: bool,
}

impl Input for BasicInput {
    fn left(&self) -> bool {
        self.left
    }

    fn right(&self) -> bool {
        self.right
    }

    fn up(&self) -> bool {
        self.up
    }

    fn down(&self) -> bool {
        self.down
    }

    fn action(&self) -> bool {
        self.action
    }
}
