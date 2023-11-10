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

#[derive(Default, Clone, Copy, Debug)]
pub struct DebouncedInput {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    action: bool,
    last_left: bool,
    last_right: bool,
    last_up: bool,
    last_down: bool,
    last_action: bool,
}

impl DebouncedInput {
    pub fn update<I: Input>(&mut self, input: &I) {
        self.left = !self.last_left && input.left();
        self.right = !self.last_right && input.right();
        self.up = !self.last_up && input.up();
        self.down = !self.last_down && input.down();
        self.action = !self.last_action && input.action();

        self.last_left = input.left();
        self.last_right = input.right();
        self.last_up = input.up();
        self.last_down = input.down();
        self.last_action = input.action();
    }
}
impl Input for DebouncedInput {
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
