use core::time::Duration;

use crate::{
    Game, RandomNumberSource,
    display::{Pixel, PixelDisplay},
    input::Input,
};

pub struct SnakeGame<const ROWS: usize, const COLS: usize> {
    state: State,
    update_timer: Duration,
    update_rate: Duration,
    position_x: isize,
    position_y: isize,
    apple_position_x: isize,
    apple_position_y: isize,
    board: [[isize; COLS]; ROWS],
    direction: Direction,
    length: usize,
    state_wait_timer: Duration,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum State {
    PreStart,
    Running,
    GameOver,
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn dx(&self) -> isize {
        match *self {
            Direction::Up => 0,
            Direction::Down => 0,
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }

    fn dy(&self) -> isize {
        match *self {
            Direction::Up => -1,
            Direction::Down => 1,
            Direction::Left => 0,
            Direction::Right => 0,
        }
    }

    fn is_opposite_to(&self, direction: Direction) -> bool {
        use Direction::*;

        matches!(
            (*self, direction),
            (Up, Down) | (Down, Up) | (Left, Right) | (Right, Left)
        )
    }
}

impl<const ROWS: usize, const COLS: usize> SnakeGame<ROWS, COLS> {
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
            direction: Direction::Up,
            length: 0,
            state_wait_timer: Duration::ZERO,
        }
    }
}

impl<const ROWS: usize, const COLS: usize> Default for SnakeGame<ROWS, COLS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const ROWS: usize, const COLS: usize, I: Input, D: PixelDisplay, R: RandomNumberSource>
    Game<I, D, R> for SnakeGame<ROWS, COLS>
{
    fn update(&mut self, elapsed: Duration, input: &I, display: &mut D, random: &mut R) {
        if self.state == State::PreStart {
            display.clear();
            display.draw_text(0, 0, "RDY");
            display.draw_text(8, COLS as isize / 2 - 3, "S");

            // delay for starting the game
            self.state_wait_timer += elapsed;

            if input.action() && self.state_wait_timer > Duration::from_millis(1000) {
                // moving on, reset the timer (for use by the game over state)
                self.state_wait_timer = Duration::ZERO;

                self.state = State::Running;
            }
            return;
        } else if self.state == State::GameOver {
            display.clear();
            display.draw_text(0, 0, "DEAD");
            display.draw_text(8, 0, "=");
            display.draw_number(8, 10, self.length);

            // delay for leaving the game over state
            self.state_wait_timer += elapsed;

            if input.action() && self.state_wait_timer > Duration::from_millis(1000) {
                *self = SnakeGame::new(); // restart by reinstantiating self ;)
            }

            return;
        } // else continue with the game logic

        self.update_timer += elapsed;

        let new_direction = if input.left() {
            Direction::Left
        } else if input.right() {
            Direction::Right
        } else if input.up() {
            Direction::Up
        } else if input.down() {
            Direction::Down
        } else {
            self.direction
        };

        if !new_direction.is_opposite_to(self.direction) {
            self.direction = new_direction;
        }

        // spawn apple in random position (not on snake itself) if unspecified
        if self.apple_position_x < 0 {
            loop {
                self.apple_position_x = (random.next_u32() as usize % COLS) as isize;
                self.apple_position_y = (random.next_u32() as usize % ROWS) as isize;

                if self.board[self.apple_position_y as usize][self.apple_position_x as usize] == 0 {
                    break;
                }
            }
        }

        if self.update_timer > self.update_rate {
            self.update_timer -= self.update_rate;

            // update board

            for r in 0..ROWS {
                for c in 0..COLS {
                    self.board[r][c] -= 1;

                    if self.board[r][c] < 0 {
                        self.board[r][c] = 0;
                    }
                }
            }
            self.board[self.position_y as usize][self.position_x as usize] = self.length as isize;

            // move head in current direction
            self.position_x += self.direction.dx();
            self.position_y += self.direction.dy();

            // check for collision with the walls
            if self.position_x < 0
                || self.position_y < 0
                || self.position_x >= COLS as isize
                || self.position_y >= ROWS as isize
            {
                self.state = State::GameOver;
                return;
            }

            // check for collision with the apple
            if self.position_x == self.apple_position_x && self.position_y == self.apple_position_y
            {
                self.update_rate =
                    Duration::from_millis(50).max(self.update_rate - Duration::from_millis(10));
                self.length += 1;
                self.apple_position_x = -1;
                self.apple_position_y = -1;
            }
            // check for collision with self
            if self.board[self.position_y as usize][self.position_x as usize] > 0 {
                // TODO: CRASH!!!
                self.state = State::GameOver;
            }
        }

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
    }

    fn state(&self) -> crate::GameState {
        match self.state {
            State::PreStart => crate::GameState::Start,
            State::Running => crate::GameState::Playing,
            State::GameOver => crate::GameState::GameOver,
        }
    }
}
