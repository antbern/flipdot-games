use core::time::Duration;

use crate::{display::PixelDisplay, input::Input, Game, GameState, RandomNumberSource};

pub struct GameMenu<'a, I: Input, D: PixelDisplay, R: RandomNumberSource> {
    games: &'a mut [&'a mut dyn Game<I, D, R>],
    current_index: usize,
}

impl<'a, I: Input, D: PixelDisplay, R: RandomNumberSource> GameMenu<'a, I, D, R> {
    pub fn new(games: &'a mut [&'a mut dyn Game<I, D, R>]) -> Self {
        Self {
            games,
            current_index: 0,
        }
    }
}

impl<I: Input, D: PixelDisplay, R: RandomNumberSource> Game<I, D, R> for GameMenu<'_, I, D, R> {
    fn update(&mut self, elapsed: Duration, input: &I, display: &mut D, random: &mut R) {
        // delegate to the currently selected game
        self.games[self.current_index].update(elapsed, input, display, random);

        // when the game is in the start state, allow changing the game using left & right
        if self.games[self.current_index].state() == GameState::Start {
            if input.left() {
                self.current_index = if self.current_index == 0 {
                    self.games.len() - 1
                } else {
                    self.current_index - 1
                }
            }

            if input.right() {
                self.current_index = if self.current_index == self.games.len() - 1 {
                    0
                } else {
                    self.current_index + 1
                }
            }

            // draw some "arrows" in the bottom corners
            display.set_pixel(display.rows() - 1, 1, crate::display::Pixel::On);
            display.set_pixel(display.rows() - 2, 0, crate::display::Pixel::On);
            display.set_pixel(display.rows() - 3, 1, crate::display::Pixel::On);
            display.set_pixel(
                display.rows() - 1,
                display.columns() - 2,
                crate::display::Pixel::On,
            );
            display.set_pixel(
                display.rows() - 2,
                display.columns() - 1,
                crate::display::Pixel::On,
            );
            display.set_pixel(
                display.rows() - 3,
                display.columns() - 2,
                crate::display::Pixel::On,
            );
        }
    }
}
