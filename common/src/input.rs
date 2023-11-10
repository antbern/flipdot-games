
/// A struct representing the current state of the input buttons
#[derive(Default, Clone, Copy, Debug)]
pub struct Input {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub action: bool,
}
