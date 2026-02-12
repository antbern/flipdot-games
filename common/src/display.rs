use core::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::font_monospace;

/// static lookup table for strings showing the score
const NUMBER_STR_LOOKUP_100: &[&str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
    "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31", "32",
    "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47", "48",
    "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "60", "61", "62", "63", "64",
    "65", "66", "67", "68", "69", "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "80",
    "81", "82", "83", "84", "85", "86", "87", "88", "89", "90", "91", "92", "93", "94", "95", "96",
    "97", "98", "99", "100", ">100",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pixel {
    On,
    Off,
}

pub trait PixelDisplay<const ROWS: usize, const COLUMNS: usize> {
    /// Sets the pixel to the desired state
    fn set_pixel(&mut self, row: usize, col: usize, value: Pixel);

    fn clear(&mut self) {
        self.fill(Pixel::Off);
    }

    fn fill(&mut self, value: Pixel) {
        for row in 0..ROWS {
            for col in 0..COLUMNS {
                self.set_pixel(row, col, value);
            }
        }
    }

    fn rows(&self) -> usize {
        ROWS
    }
    fn columns(&self) -> usize {
        COLUMNS
    }

    fn draw_text(&mut self, start_row: isize, start_col: isize, text: &str) {
        let mut column = start_col;

        for c in text.chars() {
            let c = c as u8;

            let bytes = font_monospace::get_bytes_for_char(c);

            // go through row by row
            for (i, b) in bytes.iter().enumerate() {
                // go through column by column and draw each pixel as needed
                for j in (0..font_monospace::char_width()).rev() {
                    if b & (1 << j) != 0 {
                        let row = start_row + i as isize;
                        let col = column + j as isize;

                        if row >= 0
                            && row < self.rows() as isize
                            && col >= 0
                            && col < self.columns() as isize
                        {
                            self.set_pixel(row as usize, col as usize, Pixel::On)
                        }
                    }
                }
            }

            column += font_monospace::char_width() as isize;
        }
    }

    /// Draws a number onto the screen using a look-up table.
    /// Supports numbers up to and including 100, prints ">100" otherwise.
    fn draw_number(&mut self, start_row: isize, start_col: isize, number: usize) {
        if number >= NUMBER_STR_LOOKUP_100.len() - 1 {
            self.draw_text(
                start_row,
                start_col,
                NUMBER_STR_LOOKUP_100[NUMBER_STR_LOOKUP_100.len() - 1],
            );
        } else {
            self.draw_text(start_row, start_col, NUMBER_STR_LOOKUP_100[number]);
        }
    }

    // fn rotated<'a>(&'a mut self) -> &mut RotatedDisplay<'a, COLUMNS, ROWS, _> {
    //     RotatedDisplay::new(self)
    // }
}

/// A display that has been rotated 90 degrees
pub struct RotatedDisplay<
    'a,
    const ROWS: usize,
    const COLUMNS: usize,
    D: PixelDisplay<COLUMNS, ROWS>,
> {
    display: &'a mut D,
}

impl<const ROWS: usize, const COLUMNS: usize, D: PixelDisplay<COLUMNS, ROWS> + Display> Display
    for RotatedDisplay<'_, ROWS, COLUMNS, D>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.display.fmt(f)
    }
}

impl<'a, const ROWS: usize, const COLUMNS: usize, D: PixelDisplay<COLUMNS, ROWS>>
    RotatedDisplay<'a, ROWS, COLUMNS, D>
{
    pub fn new(display: &'a mut D) -> Self {
        Self { display }
    }

    pub fn inner(&self) -> &D {
        self.display
    }

    pub fn inner_mut(&mut self) -> &mut D {
        self.display
    }
}

impl<const ROWS: usize, const COLUMNS: usize, D: PixelDisplay<ROWS, COLUMNS>>
    PixelDisplay<COLUMNS, ROWS> for RotatedDisplay<'_, COLUMNS, ROWS, D>
{
    fn set_pixel(&mut self, row: usize, col: usize, value: Pixel) {
        self.display
            .set_pixel((ROWS as isize - col as isize - 1) as usize, row, value)
    }

    fn rows(&self) -> usize {
        COLUMNS
    }

    fn columns(&self) -> usize {
        ROWS
    }
}

/// Convenience derive Deref
impl<const ROWS: usize, const COLUMNS: usize, D: PixelDisplay<COLUMNS, ROWS>> Deref
    for RotatedDisplay<'_, ROWS, COLUMNS, D>
{
    type Target = D;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

/// Convenience derive DerefMut
impl<const ROWS: usize, const COLUMNS: usize, D: PixelDisplay<COLUMNS, ROWS>> DerefMut
    for RotatedDisplay<'_, ROWS, COLUMNS, D>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}
