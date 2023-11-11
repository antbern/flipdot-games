use core::time::Duration;
use log::debug;

use crate::{
    display::{Pixel, PixelDisplay},
    input::Input,
    Game, RandomNumberSource,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BoardState {
    Free,
    Occupied,
}
pub struct TetrisGame<const ROWS: usize, const COLS: usize> {
    state: State,
    update_timer: Duration,
    update_rate: Duration,
    board: [[BoardState; COLS]; ROWS],

    score: usize,
    state_wait_timer: Duration,
    current: Option<Tetronomicon>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum State {
    PreStart,
    Running,
    GameOver,
}

#[derive(Copy, Clone, Debug)]
enum Type {
    Square,
    L,
    T,
    Line,
}
impl Type {
    fn new_random(rng: &mut impl RandomNumberSource) -> Self {
        match rng.next_u32() % 4 {
            0 => Self::Square,
            1 => Self::L,
            2 => Self::T,
            3 => Self::Line,
            _ => Self::Square,
        }
    }
    /// Returns a list of offsets to apply for this type
    fn pattern(self) -> &'static [(isize, isize); 4] {
        match self {
            Type::Square => &[(0, 0), (0, 1), (1, 0), (1, 1)],
            Type::L => &[(-2, 0), (-1, 0), (0, 0), (0, 1)],
            Type::T => &[(-1, 0), (0, 0), (0, 1), (1, 0)],
            Type::Line => &[(-1, 0), (0, 0), (1, 0), (2, 0)],
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Rotation {
    R0,
    R90,
    R180,
    R270,
}

impl Rotation {
    fn new_random(rng: &mut impl RandomNumberSource) -> Self {
        match rng.next_u32() % 4 {
            0 => Self::R0,
            1 => Self::R90,
            2 => Self::R180,
            3 => Self::R270,
            _ => Self::R0,
        }
    }

    /// Applies this rotation to a (ROW, COL) tuple
    fn apply(&self, offset: (isize, isize)) -> (isize, isize) {
        match self {
            Rotation::R0 => offset,
            Rotation::R90 => (-offset.1, offset.0),
            Rotation::R180 => (-offset.0, -offset.1),
            Rotation::R270 => (offset.1, -offset.0),
        }
    }

    fn rotate_right(&mut self) {
        *self = match self {
            Rotation::R0 => Self::R90,
            Rotation::R90 => Self::R180,
            Rotation::R180 => Self::R270,
            Rotation::R270 => Self::R0,
        }
    }

    fn rotate_left(&mut self) {
        *self = match self {
            Rotation::R0 => Self::R270,
            Rotation::R90 => Self::R0,
            Rotation::R180 => Self::R90,
            Rotation::R270 => Self::R180,
        }
    }
}

struct Tetronomicon {
    kind: Type,
    rotation: Rotation,
    row: isize,
    column: isize,
}
impl Tetronomicon {
    fn new_random(rng: &mut impl RandomNumberSource) -> Self {
        Self {
            kind: Type::new_random(rng),
            rotation: Rotation::new_random(rng),
            row: -4, // start above the screen
            column: 8,
        }
    }

    fn cells(&self) -> impl Iterator<Item = (isize, isize)> {
        // create a clones that can be moved into the map
        let rotation = self.rotation;
        let row = self.row;
        let column = self.column;
        self.kind
            .pattern()
            .iter()
            .map(move |offset| rotation.apply(*offset))
            .map(move |(r, c)| (r + row, c + column))
    }
}

// fn all_within_bounds<'a, I>(display: &impl crate::display::PixelDisplay, cells: I) -> bool
// where
//     I: Iterator<Item = &'a (isize, isize)>,
// {
//     if row >= 0
//                     && row < display.rows() as isize
//                     && col >= 0
//                     && col < display.columns() as isize
//                 {
// }

impl<const ROWS: usize, const COLS: usize> TetrisGame<ROWS, COLS> {
    pub fn new() -> Self {
        Self {
            state: State::PreStart,
            update_timer: Duration::ZERO,
            update_rate: Duration::from_millis(400),
            board: [[BoardState::Free; COLS]; ROWS],
            score: 0,
            state_wait_timer: Duration::ZERO,
            current: None,
        }
    }

    // Check if a Tetronomicon does not go outide the edges of the field and not collide with any other occupied cell
    fn is_valid(t: &Tetronomicon, board: &[[BoardState; COLS]; ROWS]) -> bool {
        // first make sure all cells are within the bounds
        for (row, col) in t.cells() {
            // left right and bottom, not checking the top (since they all start at the top)
            if row >= ROWS as isize || col < 0 || col >= COLS as isize {
                return false;
            }
        }

        // make sure no parts of the board is occupied
        for (row, col) in t.cells() {
            // left right and bottom, not checking the top (since they all start at the top)
            if row >= 0 && board[row as usize][col as usize] == BoardState::Occupied {
                return false;
            }
        }

        true
    }

    /// Move the current block down one row and run all the logic
    fn move_down(&mut self, rng: &mut impl RandomNumberSource) {
        if let Some(t) = &mut self.current {
            t.row += 1;

            // if the new position is not valid, then we collided with the bottom or anything on the board
            // so copy all the cells to the board!
            if !Self::is_valid(t, &self.board) {
                t.row -= 1;

                // if we collided above the top, then game is over
                if t.row <= 0 {
                    self.state = State::GameOver;
                }

                self.score += 1;

                for (row, col) in t.cells() {
                    if row >= 0 && row < ROWS as isize && col >= 0 && col < COLS as isize {
                        self.board[row as usize][col as usize] = BoardState::Occupied;
                    }
                }

                // run the "fall down" algorithm to remove full rows (start from top)
                for row in 0..=(ROWS - 1) {
                    let row_full =
                        (0..COLS - 1).all(|col| self.board[row][col] == BoardState::Occupied);

                    if !row_full {
                        continue;
                    }

                    // row full, move cells down!
                    for r in (0..=(row - 1)).rev() {
                        self.board[r + 1] = self.board[r];
                    }

                    // accumulate points!
                    self.score += 10;
                }

                *t = Tetronomicon::new_random(rng);
            }
        }
    }
}

impl<const ROWS: usize, const COLS: usize, I: Input, D: PixelDisplay, R: RandomNumberSource>
    Game<I, D, R> for TetrisGame<ROWS, COLS>
{
    fn update(&mut self, elapsed: Duration, input: &I, display: &mut D, random: &mut R) {
        if self.state == State::PreStart {
            display.clear();
            display.draw_text(0, 0, "RDY");

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
            display.draw_number(8, 10, self.score);

            // delay for leaving the game over state
            self.state_wait_timer += elapsed;

            if input.action() && self.state_wait_timer > Duration::from_millis(1000) {
                *self = TetrisGame::new(); // restart by reinstantiating self ;)
            }

            return;
        } // else continue with the game logic

        self.update_timer += elapsed;

        // always let the user rotate the block
        if let Some(t) = &mut self.current {
            // TODO: make sure the actions are valid!
            if input.action() {
                t.rotation.rotate_right();
                if !Self::is_valid(t, &self.board) {
                    t.rotation.rotate_left();
                }
            }

            if input.right() {
                t.column += 1;
                if !Self::is_valid(t, &self.board) {
                    t.column -= 1;
                }
            }

            if input.left() {
                t.column -= 1;
                if !Self::is_valid(t, &self.board) {
                    t.column += 1;
                }
            }

            if input.down() {
                self.move_down(random);
            }
        }

        if self.update_timer > self.update_rate {
            self.update_timer -= self.update_rate;

            if self.current.is_none() {
                self.current = Some(Tetronomicon::new_random(random))
            }
            self.move_down(random);
        }

        // redraw
        display.clear();

        for r in 0..ROWS {
            for c in 0..COLS {
                if self.board[r][c] == BoardState::Occupied {
                    display.set_pixel(r, c, Pixel::On);
                }
            }
        }

        // display the tetris block
        if let Some(t) = &self.current {
            for (row, col) in t.cells() {
                if row >= 0
                    && row < display.rows() as isize
                    && col >= 0
                    && col < display.columns() as isize
                {
                    display.set_pixel(row as usize, col as usize, Pixel::On);
                }
            }
        }
    }
}
