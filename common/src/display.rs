use crate::font_monospace;

#[derive(Clone, Copy, Debug)]
pub enum Pixel {
    On,
    Off,
}

pub trait PixelDisplay {
    const ROWS: usize;
    const COLUMNS: usize;

    fn set_pixel(&mut self, row: usize, col: usize, value: Pixel);

    fn clear(&mut self) {
        self.fill(Pixel::Off);
    }

    fn fill(&mut self, value: Pixel) {
        for row in 0..Self::ROWS {
            for col in 0..Self::COLUMNS {
                self.set_pixel(row, col, value);
            }
        }
    }

    fn rows(&self) -> usize {
        Self::ROWS
    }
    fn columns(&self) -> usize {
        Self::COLUMNS
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
}
