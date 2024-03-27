use std::{
    cmp::{max, min},
    ops::{Index, IndexMut},
};

enum BitOp {
    And,
    Or,
}

#[inline]
fn shift(x: u64, y: i32) -> u64 {
    if y < 0 {
        x >> -y
    } else {
        x << y
    }
}

/// Represents a bitmask for the game board.
/// Each cell in the board is represented by 4 bits.
/// Encoding is as follows:
///     0x0 = 0b0000 = empty
///     0x1 = 0b0001 = red
///     0x2 = 0b0010 = yellow
///     0x4 = 0b0100 = green
///     0x8 = 0b1000 = blue
#[derive(Clone, PartialEq, Hash)]
pub struct Mask {
    /// The width of the mask in cells
    width: usize,
    /// The mask stored as a single bitslice
    /// flattened
    mask: Vec<u64>,
}

impl Mask {
    /// Generate a bitmask from a 2D array of bytes.
    /// Each byte represents a cell on the board.
    pub fn new(width: usize, mask: Vec<u64>) -> Self {
        // Each row can be at most 64 bits wide
        // as it is stored as a single u64
        debug_assert!(width * 4 <= 64);

        // Now, make sure every row is contained in the width
        debug_assert!(mask.iter().all(|&row| row < (1 << (width * 4))));

        Self { width, mask }
    }

    #[inline]
    /// Get the width of the mask
    pub fn w(&self) -> usize {
        self.width
    }

    #[inline]
    /// Get the height of the mask
    pub fn h(&self) -> usize {
        self.mask.len()
    }

    /// Set the value of a cell in the mask
    pub fn set(&mut self, x: usize, y: usize, value: u64) {
        debug_assert!(x < self.w());
        debug_assert!(y < self.h());
        self.mask[y] = self.mask[y] & !(0xF << (x * 4)) | (value << (x * 4));
    }

    /// Get the value of a cell in the mask
    /// Returns None if the position is out of bounds
    pub fn get_i32(&self, x: i32, y: i32) -> Option<u64> {
        if x < 0 || y < 0 {
            return None;
        }
        self.get(x as usize, y as usize)
    }

    /// Get the value of a cell in the mask
    /// Returns None if the position is out of bounds
    pub fn get(&self, x: usize, y: usize) -> Option<u64> {
        if x >= self.w() || y >= self.h() {
            return None;
        }
        Some(self.mask[y] >> (x * 4) & 0xF)
    }

    /// Set the value of a cell in the mask without checking if the position is empty
    pub fn set_unchecked(&mut self, x: usize, y: usize, value: u64) {
        debug_assert!(x < self.w());
        debug_assert!(y < self.h());
        self.mask[y] |= value << (x * 4);
    }

    /// Check if the mask is all zeros
    pub fn empty(&self) -> bool {
        self.mask.iter().all(|&row| row == 0)
    }

    /// Bitwise AND mask with another mask
    /// at a specific position
    pub fn and(&self, other: &Mask, pos: (i32, i32)) -> Mask {
        self.binop(other, BitOp::And, pos)
    }

    /// Bitwise OR mask with another mask
    /// at a specific position
    pub fn or(&self, other: &Mask, pos: (i32, i32)) -> Mask {
        self.binop(other, BitOp::Or, pos)
    }

    /// Bitwise operation on mask with another mask
    /// placing the other mask at a specific position
    /// relative to the current mask
    fn binop(&self, other: &Mask, op: BitOp, (x, y): (i32, i32)) -> Mask {
        debug_assert!(x < self.w() as i32);
        debug_assert!(y < self.h() as i32);
        // number of rows to check
        let num_rows = min(other.h(), (self.h() as i32 - y) as usize);

        let other_y = max(-y, 0) as usize;
        let y = max(y, 0) as usize;

        // zip the two masks together
        let mask: Vec<_> = self
            .mask
            .iter()
            .skip(y)
            .take(num_rows)
            .zip(other.mask.iter().skip(other_y).take(num_rows).map(
                |row| shift(*row, x * 4), // shift the row to the right position
            ))
            // rows are zipped together now
            .map(|(row1, row2)| match op {
                BitOp::And => row1 & row2,
                BitOp::Or => (row1 | row2) & ((1 << (self.w() * 4)) - 1), // mask out the bits that are out of bounds
            })
            .collect();

        Mask::new(self.w(), mask)
    }
}

impl std::fmt::Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.mask {
            writeln!(f, "{:0width$x}", row, width = self.w())?;
        }
        Ok(())
    }
}

impl From<Mask> for Vec<u64> {
    fn from(mask: Mask) -> Self {
        mask.mask
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask() {
        // 1010
        // 0101
        let mask1 = Mask::new(4, vec![0x1010, 0x0101]);
        // 1100
        // 0011
        let mask2 = Mask::new(4, vec![0x1100, 0x0011]);

        let pos = (0, 0);

        // 1000
        // 0001
        let and = mask1.and(&mask2, pos);
        // 1110
        // 0111
        let or = mask1.or(&mask2, pos);

        assert_eq!(and.mask, vec![0x1000, 0x0001]);
        assert_eq!(or.mask, vec![0x1110, 0x0111]);

        let pos = (1, 0);

        // 1000
        // 0100
        let and = mask1.and(&mask2, pos);
        // 1010
        // 0111
        let or = mask1.or(&mask2, pos);

        assert_eq!(and.mask, vec![0x1000, 0x0100]);
        assert_eq!(or.mask, vec![0x1010, 0x0111]);

        // test negative y and x

        let pos = (-1, -1);

        // 0000
        let and = mask1.and(&mask2, pos);

        // 1011
        let or = mask1.or(&mask2, pos);

        assert_eq!(and.mask, vec![0x000]);
        assert_eq!(or.mask, vec![0x1011])
    }
}