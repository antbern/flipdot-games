use core::time::Duration;

use crate::{display::Pixel, Game, Input, RandomNumberSource};

pub struct TetrisGame<const ROWS: usize, const COLS: usize> {
    state: State,
    update_timer: Duration,
    update_rate: Duration,
    position_x: isize,
    position_y: isize,
    apple_position_x: isize,
    apple_position_y: isize,
    board: [[isize; COLS]; ROWS],
    score: usize,
    state_wait_timer: Duration,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum State {
    PreStart,
    Running,
    GameOver,
}

impl<const ROWS: usize, const COLS: usize> TetrisGame<ROWS, COLS> {
    pub fn new() -> Self {
        Self {
            state: State::PreStart,
            update_timer: Duration::ZERO,
            update_rate: Duration::from_millis(400),
            position_x: COLS as isize / 2,
            position_y: ROWS as isize / 2,
            apple_position_x: 5,
            apple_position_y: 5,
            board: [[0; COLS]; ROWS],
            score: 0,
            state_wait_timer: Duration::ZERO,
        }
    }

    fn reset(&mut self) {}
}

impl<const ROWS: usize, const COLS: usize> Game for TetrisGame<ROWS, COLS> {
    fn update(
        &mut self,
        elapsed: core::time::Duration,
        input: crate::Input,
        display: &mut impl crate::display::PixelDisplay,
        rng: &mut impl RandomNumberSource,
    ) -> bool {
        if self.state == State::PreStart {
            display.clear();
            display.draw_text(0, 0, "RDY");

            // delay for starting the game
            self.state_wait_timer += elapsed;

            if input.action && self.state_wait_timer > Duration::from_millis(1000) {
                // moving on, reset the timer (for use by the game over state)
                self.state_wait_timer = Duration::ZERO;

                self.state = State::Running;
            }
            return true;
        } else if self.state == State::GameOver {
            display.clear();
            display.draw_text(0, 0, "DEAD");
            display.draw_text(8, 0, "=");
            display.draw_number(8, 10, self.score);

            // delay for leaving the game over state
            self.state_wait_timer += elapsed;

            if input.action && self.state_wait_timer > Duration::from_millis(1000) {
                *self = TetrisGame::new(); // restart by reinstantiating self ;)
            }

            return true;
        } // else continue with the game logic

        self.update_timer += elapsed;

        if self.update_timer > self.update_rate {
            self.update_timer -= self.update_rate;

            // redraw
            display.clear();

            display.set_pixel(
                self.position_y as usize,
                self.position_x as usize,
                Pixel::On,
            );
            if self.apple_position_x >= 0 {
                display.set_pixel(
                    self.apple_position_y as usize,
                    self.apple_position_x as usize,
                    Pixel::On,
                );
            }
            for r in 0..ROWS {
                for c in 0..COLS {
                    if self.board[r][c] > 0 {
                        display.set_pixel(r, c, Pixel::On);
                    }
                }
            }

            return true;
        }

        false
    }
}
