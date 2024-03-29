use std::{arch::x86_64::*, fmt::Debug};

use crate::game::Neighbor;

const BOARD_SIZE: usize = 20;

/// Struct representing a piece
/// All masks in this piece are stored as an __m256i, meaning
/// they can store 32 * 8 bits, or 8 rows of mask information.
struct Piece {
    /// Say we have a piece that looks like
    /// XX
    ///  X
    ///  XX
    /// Normally, our occupation bitmask would look like
    /// 0b110
    /// 0b010
    /// 0b011
    /// But we want to be able to check multiple locations
    /// at once. To do this, we repeat our masks as follows:
    ///
    /// 0b00110110110110110110   <
    /// 0b00010010010010010010   <
    /// 0b00011011011011011011   <
    /// 0b00110110110110110110   < 8 rows tall
    /// 0b00010010010010010010   <
    /// 0b00011011011011011011   <
    /// 0b00000000000000000000   < 2 rows of zeros here as
    /// 0b00000000000000000000   < 3 doesn't exactly divide 8
    ///   ^
    ///   2 bits of zeros as 3 doesnt exactly divide 20
    occupied_mask: __m256i,
    // /// The corner mask contains pieces of the mask that are considered corners.
    // /// In this specific piece, almost every spot is a corner, so our mask looks
    // /// like this:
    // /// 0b110
    // /// 0b000
    // /// 0b011
    // /// We repeat this mask the same way we do with the occupied mask.
    // corner_mask: __m256i,
    // /// Given the same piece as above, the neighbor mask would
    // /// look like
    // /// 0b01100
    // /// 0b10010
    // /// 0b01010
    // /// 0b01001
    // /// 0b00110
    // /// This "wraps" the original piece.
    // /// This one can't be easily repeated
    neighbor_mask: __m256i,
}

impl Piece {
    pub fn new(width: usize, height: usize, mut piece: [u32; 8]) -> Self {
        debug_assert!(width < 32);
        debug_assert!(height < 8);
        // Generate the neighbor mask
        let mut neighbor_mask = [0u32; 8];
        for x in 0..width {
            for y in 0..height {
                let cell = ((piece[y] >> x) & 1) == 1;
                if cell {
                    // loop through neighbors and fill them in the neighbor mask
                    // if they are empty in the occupation  mask
                    for dir in Neighbor::iter() {
                        let (x, y) = dir + (x as i32, y as i32);
                        if
                        // this is out of bounds of the piece, and is therefore definitely a valid neighbor
                        x < 0
                            || y < 0
                            || x >= width as i32
                            || y >= height as i32
                            // otherwise, check if the cell is empty in the piece
                            || ((piece[y as usize] >> x) & 1) == 0
                        {
                            neighbor_mask[(y + 1) as usize] |= 1 << (x + 1);
                        }
                    }
                }
            }
        }

        // Replicate the mask across the board
        let x_replicas = BOARD_SIZE / width;
        let y_replicas = 8 / height;
        for x in 0..x_replicas {
            for y in 0..y_replicas {
                let x = x * width; // x location
                let y = y * height; // y location

                // loop through each row
                for row in 0..height {
                    let piece_mask = piece[row] & ((1 << width) - 1);
                    piece[y + row] |= piece_mask << x;
                }
            }
        }

        unsafe {
            Self {
                occupied_mask: _mm256_loadu_si256(piece.as_ptr() as *const __m256i),
                neighbor_mask: _mm256_loadu_si256(neighbor_mask.as_ptr() as *const __m256i),
            }
        }
    }
}

impl Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let rows = std::mem::transmute::<__m256i, [u32; 8]>(self.occupied_mask);
            writeln!(f, "Occupied Mask:")?;
            for x in rows.iter() {
                writeln!(f, "{:032b}", x)?;
            }
            writeln!(f, "Neighbor Mask:")?;
            let rows = std::mem::transmute::<__m256i, [u32; 8]>(self.neighbor_mask);
            for x in rows.iter() {
                writeln!(f, "{:032b}", x)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    /// Check whether two ymms are equal
    unsafe fn ymm_eq(a: __m256i, b: __m256i) -> bool {
        // Fills mask with 1 if a and b are equal
        let mask = _mm256_cmpeq_epi32(a, b);
        // Take the most significant bit from each 8 bit chunk
        let ret = _mm256_movemask_epi8(mask);
        ret == -1
    }

    use super::*;
    #[test]
    fn test_mask_repetition() {
        let piece = Piece::new(3, 3, [0b110, 0b010, 0b011, 0, 0, 0, 0, 0]);

        let expected = [
            0b00110110110110110110,
            0b00010010010010010010,
            0b00011011011011011011,
            0b00110110110110110110,
            0b00010010010010010010,
            0b00011011011011011011,
            0b00000000000000000000,
            0b00000000000000000000,
        ];

        unsafe {
            let eq = ymm_eq(
                piece.occupied_mask,
                _mm256_loadu_si256(expected.as_ptr() as *const __m256i),
            );
            assert!(eq);
        }
    }

    #[test]
    fn test_neighbor_mask() {
        let piece = Piece::new(3, 3, [0b110, 0b010, 0b011, 0, 0, 0, 0, 0]);
        println!("{:?}", piece);

        let expected = [0b01100, 0b10010, 0b01010, 0b01001, 0b00110, 0, 0, 0];

        assert!(unsafe {
            ymm_eq(
                piece.neighbor_mask,
                _mm256_loadu_si256(expected.as_ptr() as *const __m256i),
            )
        })
    }
}
