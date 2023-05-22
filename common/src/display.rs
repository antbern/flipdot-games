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
}
