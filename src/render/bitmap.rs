use super::{Pos, Size};

/// RGBA bitmap.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Bitmap {
    data: Vec<u32>,
    size: Size,
}

impl Bitmap {
    #[inline]
    pub fn new(size: Size) -> Self {
        Self {
            data: vec![0; size.w as usize * size.h as usize],
            size,
        }
    }

    /// Resizes the bitmap with `size` as the size.
    /// This completely resets the data.
    pub fn resize(&mut self, size: Size) {
        self.data = vec![0; size.w as usize * size.h as usize];
        self.size = size;
    }

    #[inline]
    pub fn pixels(&self) -> &[u32] {
        &self.data
    }

    /// Gives a full horizontal line of pixels
    pub fn line(&self, pos: Pos, width: u16) -> &[u32] {
        let idx = self.index(pos);
        &self.data[idx..(idx + width as usize)]
    }

    /// Gives a full horizontal line of pixels (mutable)
    pub fn line_mut(&mut self, pos: Pos, width: u16) -> &mut [u32] {
        let idx = self.index(pos);
        &mut self.data[idx..(idx + width as usize)]
    }

    /// A function that looks like this:
    ///
    /// ```ignore
    /// #[inline]
    /// pub fn size(&self) -> Size {
    ///     self.size
    /// }
    /// ```
    #[inline]
    pub fn size(&self) -> Size {
        self.size
    }

    pub fn copy_all_pixels(&mut self, other: &Bitmap) {
        self.data.copy_from_slice(&other.data)
    }

    pub fn copy_pixels(&mut self, other: &Bitmap, src_pos: Pos, dst_pos: Pos, size: Size) {
        for y in 0..size.h {
            let dst_line = self.line_mut(dst_pos, size.w);
            let src_line = other.line(src_pos, size.w);
            dst_line.copy_from_slice(src_line);
        }
    }

    /// Converts a position to the index of a pixel on the bitmap.
    fn index(&self, pos: Pos) -> usize {
        debug_assert!(
            pos.x >= 0 && pos.y >= 0,
            "Position has a negative coordinate"
        );
        debug_assert!(
            (pos.x as i32) < (self.size.w as i32),
            "Position exceeds bitmap width"
        );
        debug_assert!(
            (pos.y as i32) < (self.size.h as i32),
            "Position exceeds bitmap height"
        );

        pos.y as usize * self.size.w as usize * pos.x as usize
    }
}
