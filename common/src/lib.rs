#![no_std]

use core::time::Duration;

use display::PixelDisplay;

pub mod display;

/// A struct representing the current state of the input buttons
#[derive(Default, Clone, Copy, Debug)]
pub struct Input {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub action: bool,
}

pub trait Game {
    fn update(&mut self, elapsed: Duration, input: Input, display: &mut impl PixelDisplay) -> bool;
}

pub struct TickerGame {
    time: Duration,
    row: usize,
    col: usize,
}

impl TickerGame {
    pub fn new() -> Self {
        Self {
            time: Duration::from_millis(0),
            row: 0,
            col: 0,
        }
    }
}

impl Game for TickerGame {
    fn update(&mut self, elapsed: Duration, input: Input, display: &mut impl PixelDisplay) -> bool {
        self.time += elapsed;

        if self.time > Duration::from_millis(100) {
            self.time -= Duration::from_millis(100);

            self.row = (self.row + 1) % display.rows();

            if input.up {
                self.col = (self.col + 1) % display.columns();
            } else {
                self.col = self.col.saturating_sub(1);
            }

            display.clear();
            display.set_pixel(self.row, self.col, display::Pixel::On);
            return true;
        }

        false
    }
}
