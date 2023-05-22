//! Demonstrates how to block read events.
//!
//! cargo run --example event-read

use std::fmt::Display;
use std::io::{self, stdout};

use common::display::{Pixel, PixelDisplay};
use crossterm::cursor;
use crossterm::event::{
    poll, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::style::{Color, Print, SetForegroundColor};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{
    cursor::position,
    event::{read, EnableBracketedPaste, Event, KeyCode},
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode},
};
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
}

impl<const ROWS: usize, const COLS: usize> Display for ConsoleDisplay<ROWS, COLS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in (0..ROWS).step_by(2) {
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
            write!(f, "\n\r")?;
        }

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

fn print_events() -> io::Result<()> {
    let mut d: ConsoleDisplay<16, 42> = ConsoleDisplay::new();

    let mut last_frame_time = Instant::now();
    loop {
        // Wait up to 10ms for another event
        if poll(Duration::from_millis(10))? {
            // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
            let event = read()?;

            println!("Event::{:?}\r", event);

            if event == Event::Key(KeyCode::Char('c').into()) {
                println!("Cursor position: {:?}\r", position());
            }

            if let Event::Key(ke) = event {
                match ke.code {
                    KeyCode::Char('w') => d.fill(Pixel::On),
                    KeyCode::Char('s') => d.fill(Pixel::Off),
                    KeyCode::Char('a') => {
                        d.fill(Pixel::Off);

                        for row in (0..d.rows()).step_by(2) {
                            for col in (0..d.columns()).step_by(2) {
                                d.set_pixel(row, col, Pixel::On)
                            }
                        }
                    }
                    KeyCode::Char('d') => {
                        d.fill(Pixel::On);

                        for row in (0..d.rows()).step_by(2) {
                            for col in (0..d.columns()).step_by(2) {
                                d.set_pixel(row, col, Pixel::Off)
                            }
                        }
                    }
                    KeyCode::Char('e') => {
                        d.fill(Pixel::Off);

                        for row in 0..d.rows() {
                            for col in 0..d.columns() {
                                if (row + col) % 2 == 0 {
                                    d.set_pixel(row, col, Pixel::On)
                                }
                            }
                        }
                    }
                    KeyCode::Char('q') => {
                        d.fill(Pixel::Off);

                        for row in 0..d.rows() {
                            for col in 0..d.columns() {
                                if (row + col) % 2 == 1 {
                                    d.set_pixel(row, col, Pixel::On)
                                }
                            }
                        }
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        let current_time = Instant::now();

        if (current_time - last_frame_time) > Duration::from_millis(100) {
            last_frame_time = current_time;

            // refresh the screen

            execute!(
                stdout(),
                cursor::MoveTo(0, 0),
                SetForegroundColor(Color::Yellow),
                Print(&d),
                SetForegroundColor(Color::White)
            )?;
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

    if supports_keyboard_enhancement {
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

    execute!(stdout, EnableBracketedPaste, EnterAlternateScreen)?;

    if let Err(e) = print_events() {
        println!("Error: {:?}\r", e);
    }

    if supports_keyboard_enhancement {
        queue!(stdout, PopKeyboardEnhancementFlags)?;
    }

    execute!(stdout, PopKeyboardEnhancementFlags, LeaveAlternateScreen)?;

    disable_raw_mode()
}
