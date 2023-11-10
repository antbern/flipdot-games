//! Demonstrates how to block read events.
//!
//! cargo run --example event-read

use std::fmt::Display;
use std::io::{self, stdout};

use common::display::{Pixel, PixelDisplay};
use common::snake::SnakeGame;
use common::tetris::TetrisGame;
use common::{Game, Input, RandomNumberSource};
use crossterm::event::{
    poll, KeyEventKind, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
    PushKeyboardEnhancementFlags,
};
use crossterm::style::{Color, Print, SetForegroundColor};
use crossterm::terminal::{Clear, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, terminal};
use crossterm::{
    event::{read, Event, KeyCode},
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::prelude::*;
use std::time::{Duration, Instant};

const HELP: &str = r#"Blocking read()
 - Keyboard, mouse, focus and terminal resize events enabled
 - Hit "c" to print current cursor position
 - Use Esc to quit
"#;

struct ConsoleDisplay<const ROWS: usize, const COLS: usize> {
    buffer: [[Pixel; COLS]; ROWS],
}

impl<const ROWS: usize, const COLS: usize> ConsoleDisplay<ROWS, COLS> {
    pub fn new() -> Self {
        Self {
            buffer: [[Pixel::Off; COLS]; ROWS],
        }
    }

    pub fn changed(&self, other: &Self) -> bool {
        self.buffer == other.buffer
    }
}

impl<const ROWS: usize, const COLS: usize> Display for ConsoleDisplay<ROWS, COLS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "┏")?;
        for _ in 0..COLS {
            write!(f, "━")?;
        }
        write!(f, "┓\n\r")?;

        for row in (0..ROWS).step_by(2) {
            write!(f, "┃")?;
            for col in 0..COLS {
                write!(
                    f,
                    "{}",
                    match (self.buffer[row][col], self.buffer[row + 1][col]) {
                        (Pixel::On, Pixel::On) => "█",
                        (Pixel::On, Pixel::Off) => "▀",
                        (Pixel::Off, Pixel::On) => "▄",
                        (Pixel::Off, Pixel::Off) => " ",
                    }
                )?;
            }
            write!(f, "┃\n\r")?;
        }
        write!(f, "┗")?;
        for _ in 0..COLS {
            write!(f, "━")?;
        }
        write!(f, "┛\n\r")?;

        Ok(())
    }
}

impl<const ROWS: usize, const COLS: usize> PixelDisplay for ConsoleDisplay<ROWS, COLS> {
    const ROWS: usize = ROWS;
    const COLUMNS: usize = COLS;

    fn set_pixel(&mut self, row: usize, col: usize, value: Pixel) {
        self.buffer[row][col] = value;
    }
}

struct Random {
    rng: ThreadRng,
}

impl RandomNumberSource for Random {
    fn next_u32(&mut self) -> u32 {
        self.rng.gen()
    }
}
fn print_events() -> io::Result<()> {
    let mut d: ConsoleDisplay<42, 16> = ConsoleDisplay::new();
    let mut d2: ConsoleDisplay<42, 16> = ConsoleDisplay::new(); // for double buffering

    let mut i = Input::default();
    let mut rng = Random { rng: thread_rng() };

    //let mut game = TickerGame::new();
    // let mut game: SnakeGame<42, 16> = SnakeGame::new();
    let mut game: TetrisGame<42, 16> = TetrisGame::new();

    let mut last_frame_time = Instant::now();
    loop {
        // Wait up to 10ms for another event
        if poll(Duration::from_millis(10))? {
            // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
            let event = read()?;

            if let Event::Key(ke) = event {
                match (ke.code, ke.kind) {
                    (KeyCode::Char('w'), KeyEventKind::Press) => i.up = true,
                    (KeyCode::Char('a'), KeyEventKind::Press) => i.left = true,
                    (KeyCode::Char('s'), KeyEventKind::Press) => i.down = true,
                    (KeyCode::Char('d'), KeyEventKind::Press) => i.right = true,
                    (KeyCode::Char('w'), KeyEventKind::Release) => i.up = false,
                    (KeyCode::Char('a'), KeyEventKind::Release) => i.left = false,
                    (KeyCode::Char('s'), KeyEventKind::Release) => i.down = false,
                    (KeyCode::Char('d'), KeyEventKind::Release) => i.right = false,
                    (KeyCode::Char(' '), KeyEventKind::Press) => i.action = true,
                    (KeyCode::Char(' '), KeyEventKind::Release) => i.action = false,

                    (KeyCode::Esc, _) => break,
                    _ => {}
                }
            }
        }

        let current_time = Instant::now();
        let elapsed = current_time - last_frame_time;

        if elapsed > Duration::from_millis(10) {
            last_frame_time = current_time;

            // double buffering to only update if the display actually changed...

            game.update(elapsed, i, &mut d, &mut rng);

            // update display only if concents changed
            if d.changed(&d2) {
                execute!(
                    stdout(),
                    terminal::BeginSynchronizedUpdate,
                    Clear(crossterm::terminal::ClearType::All),
                    cursor::MoveTo(0, 0),
                    SetForegroundColor(Color::Yellow),
                    Print(&d),
                    SetForegroundColor(Color::White),
                    terminal::EndSynchronizedUpdate
                )?;
            }

            // swap the buffers
            std::mem::swap(&mut d, &mut d2);
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    println!("{}", HELP);

    enable_raw_mode()?;

    let mut stdout = io::stdout();

    let supports_keyboard_enhancement = matches!(
        crossterm::terminal::supports_keyboard_enhancement(),
        Ok(true)
    );

    execute!(stdout, EnterAlternateScreen)?;
    if supports_keyboard_enhancement {
        println!("Enabling Keyboard Enhancement");
        queue!(
            stdout,
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                    | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
            )
        )?;
    }

    if let Err(e) = print_events() {
        println!("Error: {:?}\r", e);
    }

    if supports_keyboard_enhancement {
        execute!(stdout, PopKeyboardEnhancementFlags)?;
    }
    execute!(stdout, LeaveAlternateScreen)?;

    disable_raw_mode()
}
