use std::{
    cmp::{max, min},
    ops::Shl,
};

use super::Dimensioned;

#[inline]
fn shift(x: u128, y: i32) -> u128 {
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
    mask: Vec<u128>,
}

impl Mask {
    /// Generate a bitmask from a 2D array of bytes.
    /// Each byte represents a cell on the board.
    pub fn new(width: usize, mask: Vec<u128>) -> Self {
        // Each row can be at most 128 bits wide
        // as it is stored as a single u128
        debug_assert!(width * 4 <= 128);

        // Now, make sure every row is contained in the width
        // This is disabled as we are using a const function
        debug_assert!(mask.iter().all(|&row| row < (1 << (width * 4))));

        Self { width, mask }
    }

    /// Set the value of a cell in the mask
    pub fn set(&mut self, x: usize, y: usize, value: u128) {
        debug_assert!(x < self.w());
        debug_assert!(y < self.h());
        self.mask[y] = self.mask[y] & !(0xF << (x * 4)) | (value << (x * 4));
    }

    /// Get the value of a cell in the mask
    /// Returns None if the position is out of bounds
    pub fn get_i32(&self, x: i32, y: i32) -> Option<u128> {
        if x < 0 || y < 0 {
            return None;
        }
        self.get(x as usize, y as usize)
    }

    /// Get the value of a cell in the mask
    /// Returns None if the position is out of bounds
    pub fn get(&self, x: usize, y: usize) -> Option<u128> {
        if x >= self.w() || y >= self.h() {
            return None;
        }
        Some(self.mask[y] >> (x * 4) & 0xF)
    }

    /// Set the value of a cell in the mask without checking if the position is empty
    pub fn set_unchecked(&mut self, x: usize, y: usize, value: u128) {
        debug_assert!(x < self.w());
        debug_assert!(y < self.h());
        self.mask[y] |= value << (x * 4);
    }

    /// Check if the mask is all zeros
    pub fn empty(&self) -> bool {
        self.mask.iter().all(|&row| row == 0)
    }

    /// Bitwise or mask
    pub fn assign_or(&mut self, other: &Mask, pos: (i32, i32)) {
        let (x, y) = pos;
        debug_assert!(x < self.w() as i32);
        debug_assert!(y < self.h() as i32);
        // number of rows to check
        let num_rows = min(other.h(), (self.h() as i32 - y) as usize);

        let other_y = max(-y, 0) as usize;
        let y = max(y, 0) as usize;

        let num_rows = num_rows - other_y;

        let w = self.w();

        // zip the two masks together
        self.mask
            .iter_mut()
            .skip(y)
            .take(num_rows)
            .zip(other.mask.iter().skip(other_y).map(
                |row| shift(*row, x * 4), // shift the row to the right position
            ))
            // rows are zipped together now
            .for_each(|(row1, row2)| *row1 = (*row1 | row2) & ((1 << (w * 4)) - 1));
    }

    /// Bitwise OR mask with another mask
    /// at a specific position
    pub fn or(&self, other: &Mask, pos: (i32, i32)) -> Mask {
        let mut mask = self.clone();
        mask.assign_or(other, pos);
        mask
    }

    /// Check if two masks don't overlap
    pub fn no_overlap(&self, other: &Mask, pos: (i32, i32)) -> bool {
        let (x, y) = pos;
        debug_assert!(x < self.w() as i32);
        debug_assert!(y < self.h() as i32);
        // number of rows to check
        let num_rows = min(other.h(), (self.h() as i32 - y) as usize);

        let other_y = max(-y, 0) as usize;
        let y = max(y, 0) as usize;

        let num_rows = num_rows - other_y;

        // zip the two masks together
        self.mask
            .iter()
            .skip(y)
            .take(num_rows)
            .zip(other.mask.iter().skip(other_y).map(
                |row| shift(*row, x * 4), // shift the row to the right position
            ))
            // rows are zipped together now
            .map(|(row1, row2)| row1 & row2)
            .all(|x| x == 0)
    }

    pub fn iter(&self) -> impl Iterator<Item = &'_ u128> {
        self.mask.iter()
    }
}

impl Dimensioned for Mask {
    #[inline]
    /// Get the width of the mask
    fn w(&self) -> usize {
        self.width
    }

    #[inline]
    /// Get the height of the mask
    fn h(&self) -> usize {
        self.mask.len()
    }
}

impl Shl<usize> for Mask {
    type Output = Mask;

    fn shl(self, rhs: usize) -> Self::Output {
        let mask = self.mask.iter().map(|&row| row << rhs).collect();
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

impl From<Mask> for Vec<u128> {
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

        // 1110
        // 0111
        let or = mask1.or(&mask2, pos);

        assert_eq!(or.mask, vec![0x1110, 0x0111]);

        let pos = (1, 0);

        // 1010
        // 0111
        let or = mask1.or(&mask2, pos);

        assert_eq!(or.mask, vec![0x1010, 0x0111]);

        // test negative y and x

        let pos = (-1, -1);

        // 1011
        // 0101
        let or = mask1.or(&mask2, pos);

        assert_eq!(or.mask, vec![0x1011, 0x0101]);
    }
}
