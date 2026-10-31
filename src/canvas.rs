use crate::{Frame, Result, dimensions, printable};

/// Mutable ASCII drawing surface. Drawing outside the grid is clipped.
#[derive(Clone, Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    cells: Vec<u8>,
}
impl Canvas {
    pub fn new(width: usize, height: usize) -> Result<Self> {
        Ok(Self {
            width,
            height,
            cells: vec![b' '; dimensions(width, height)?],
        })
    }
    pub fn from_frame(f: &Frame) -> Self {
        Self {
            width: f.width(),
            height: f.height(),
            cells: f.cells().to_vec(),
        }
    }
    pub fn set(&mut self, x: i32, y: i32, value: u8) -> Result<()> {
        printable(&[value])?;
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            self.cells[y as usize * self.width + x as usize] = value;
        }
        Ok(())
    }
    pub fn text(&mut self, x: i32, y: i32, text: &str) -> Result<()> {
        printable(text.as_bytes())?;
        if text.len() > crate::MAX_CELLS {
            return Err("Text stamp is too long".into());
        }
        for (offset, value) in text.bytes().enumerate() {
            if let Some(px) = x.checked_add(offset as i32) {
                self.set(px, y, value)?;
            }
        }
        Ok(())
    }
    pub fn overlay(
        &mut self,
        other: &Frame,
        x: i32,
        y: i32,
        transparent_spaces: bool,
    ) -> Result<()> {
        for row in 0..other.height() {
            for col in 0..other.width() {
                let value = other.cells()[row * other.width() + col];
                if transparent_spaces && value == b' ' {
                    continue;
                }
                if let (Some(px), Some(py)) = (x.checked_add(col as i32), y.checked_add(row as i32))
                {
                    self.set(px, py, value)?;
                }
            }
        }
        Ok(())
    }
    pub fn line(&mut self, start: (i32, i32), end: (i32, i32), value: u8) -> Result<()> {
        printable(&[value])?;
        if [start.0, start.1, end.0, end.1]
            .iter()
            .any(|v| !(-4096..=4096).contains(v))
        {
            return Err("Line coordinates must be within -4096..=4096".into());
        }
        let (mut x, mut y) = start;
        let dx = (end.0 - x).abs();
        let dy = -(end.1 - y).abs();
        let sx = if x < end.0 { 1 } else { -1 };
        let sy = if y < end.1 { 1 } else { -1 };
        let mut error = dx + dy;
        loop {
            self.set(x, y, value)?;
            if (x, y) == end {
                break;
            }
            let twice = error * 2;
            if twice >= dy {
                error += dy;
                x += sx;
            }
            if twice <= dx {
                error += dx;
                y += sy;
            }
        }
        Ok(())
    }
    pub fn rectangle(
        &mut self,
        x: i32,
        y: i32,
        width: usize,
        height: usize,
        value: u8,
    ) -> Result<()> {
        dimensions(width, height)?;
        let right = x
            .checked_add(width as i32 - 1)
            .ok_or("Rectangle coordinate overflow")?;
        let bottom = y
            .checked_add(height as i32 - 1)
            .ok_or("Rectangle coordinate overflow")?;
        for (a, b) in [
            ((x, y), (right, y)),
            ((right, y), (right, bottom)),
            ((right, bottom), (x, bottom)),
            ((x, bottom), (x, y)),
        ] {
            self.line(a, b, value)?;
        }
        Ok(())
    }
    pub fn finish(self) -> Frame {
        Frame::new(self.width, self.height, self.cells).expect("Canvas preserves ASCII invariant")
    }
}
