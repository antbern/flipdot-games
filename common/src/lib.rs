#![no_std]

use core::time::Duration;

use display::PixelDisplay;
use input::Input;

pub mod display;
pub mod font_monospace;
pub mod input;
pub mod snake;
pub mod tetris;

/// Trait for system-specific generation of a seed for the random number generator
pub trait RandomNumberSource {
    fn next_u32(&mut self) -> u32;
}

pub trait Game<I: Input, D: PixelDisplay, R: RandomNumberSource> {
    /// Runs the logic of the game given the current state of the input and shows it to the display
    /// Should always draw the current state to the display in immediate-mode like style.
    fn update(&mut self, elapsed: Duration, input: &I, display: &mut D, random: &mut R);
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

impl Default for TickerGame {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: Input, D: PixelDisplay, R: RandomNumberSource> Game<I, D, R> for TickerGame {
    fn update(&mut self, elapsed: Duration, input: &I, display: &mut D, _random: &mut R) {
        self.time += elapsed;

        if self.time > Duration::from_millis(200) {
            self.time -= Duration::from_millis(200);

            self.row = (self.row + 1) % display.rows();

            if input.up() {
                self.col = (self.col + 1) % display.columns();
            } else {
                self.col = self.col.saturating_sub(1);
            }
        }

        display.clear();
        display.set_pixel(self.row, self.col, display::Pixel::On);

        display.draw_text(0, 0, "HELLO!");
    }
}
