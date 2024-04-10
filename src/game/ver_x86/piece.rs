use crate::game::{utils::PieceID, ver_x86::utils::ymm, Corner, Neighbor};
use std::{arch::x86_64::*, fmt::Debug};

const BOARD_SIZE: i8 = 20;

/// Struct representing a piece
/// All masks in this piece are stored as an __m256i, meaning
/// they can store 32 * 8 bits, or 8 rows of mask information.
pub struct Piece {
    /// Width of the piece
    pub width: i8,
    /// Height of the piece
    pub height: i8,
    /// Bitmask of the piece
    pub occupied_mask: __m256i,
    // /// Say we have a piece that looks like
    // /// ```str
    // /// XX
    // ///  X
    // ///  XX
    // /// ```
    // /// Normally, our occupation bitmask would look like
    // /// ```str
    // /// 0b110
    // /// 0b010
    // /// 0b011
    // /// ````
    // /// But we want to be able to check multiple locations
    // /// at once. To do this, we repeat our masks as follows:
    // ///```str
    // ///0b00110110110110110110   <
    // ///0b00010010010010010010   <
    // ///0b00011011011011011011   <
    // ///0b00110110110110110110   < 8 rows tall
    // ///0b00010010010010010010   <
    // ///0b00011011011011011011   <
    // ///0b00000000000000000000   < 2 rows of zeros here as
    // ///0b00000000000000000000   < 3 doesn't exactly divide 8
    // ///  ^
    // ///  2 bits of zeros as 3 doesnt exactly divide 20
    // /// ```
    // pub repeated_mask: __m256i,
    /// The corner mask contains pieces diagonally adjacent to the mask
    /// In this specific piece, this would look like the following:
    /// ```str
    /// 0b10010
    /// 0b00000
    /// 0b10001
    /// 0b00000
    /// 0b01001
    /// ```
    /// We repeat this mask the same way we do with the occupied mask.
    corner_mask: __m256i,
    /// Given the same piece as above, the neighbor mask would
    /// look like
    /// ```str
    /// 0b01100
    /// 0b10010
    /// 0b01010
    /// 0b01001
    /// 0b00110
    /// ```
    /// This "wraps" the original piece.
    /// This one isn't replicated like above.
    pub neighbor_mask: __m256i,
}

impl Piece {
    pub fn new(width: i8, height: i8, id: PieceID, mut piece: [u32; 8]) -> Self {
        debug_assert!(width < 32);
        debug_assert!(height < 8);
        // Generate the neighbor mask
        let mut neighbor_mask = [0u32; 8];
        let mut corner_mask = [0u32; 8];
        for x in 0..width {
            for y in 0..height {
                let cell = ((piece[y as usize] >> x) & 1) == 1;
                if cell {
                    // loop through neighbors and fill them in the neighbor mask
                    // if they are empty in the occupation  mask
                    for dir in Neighbor::iter() {
                        let (x, y) = dir + (x, y);
                        if x < 0 // this is out of bounds of the piece, and is therefore definitely a valid neighbor
                            || y < 0
                            || x >= width
                            || y >= height
                            // otherwise, check if the cell is empty in the piece
                            || ((piece[y as usize] >> x) & 1) == 0
                        {
                            neighbor_mask[(y + 1) as usize] |= 1 << (x + 1);
                        }
                    }

                    // Loop through the corner neighbors and fill them in
                    // if the two neighbors in that direction are empty
                    // and the diagonally adjacent tile is also empty
                    for dir in Corner::iter() {
                        let (cx, cy) = dir + (x, y);
                        if neighbor_mask[(cy + 1) as usize] & (1 << (x + 1)) != 0
                            && neighbor_mask[(y + 1) as usize] & (1 << (cx + 1)) != 0
                        // can skip the diagonal check as we have no pieces that do this
                        {
                            corner_mask[(cy + 1) as usize] |= 1 << (cx + 1);
                        }
                    }
                }
            }
        }

        // Replicate the mask across the board
        // let x_replicas = BOARD_SIZE / width;
        // let y_replicas = 8 / height;
        // let mut repeated_piece = [0u32; 8];
        // for x in 0..x_replicas {
        //     for y in 0..y_replicas {
        //         let x = x * width; // x location
        //         let y = y * height; // y location

        //         // loop through each row
        //         for row in 0..height {
        //             let piece_mask = piece[row as usize] & ((1 << width) - 1);
        //             repeated_piece[(y + row) as usize] |= piece_mask << x;
        //         }
        //     }
        // }

        unsafe {
            Self {
                width,
                height,
                occupied_mask: ymm(piece),
                corner_mask: ymm(corner_mask),
                // repeated_mask: ymm(repeated_piece),
                neighbor_mask: ymm(neighbor_mask),
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
            let rows = std::mem::transmute::<__m256i, [u32; 8]>(self.corner_mask);
            writeln!(f, "Corner Mask:")?;
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
    use crate::game::ver_x86::utils::{ymm, ymm_eq};

    use super::*;
    #[test]
    fn test_occupied_mask() {
        let piece = Piece::new(3, 3, PieceID::from(0), [0b110, 0b010, 0b011, 0, 0, 0, 0, 0]);
        println!("{:?}", piece);

        let expected = [0b110, 0b010, 0b011, 0, 0, 0, 0, 0];

        assert!(unsafe { ymm_eq(piece.occupied_mask, ymm(expected),) });
    }

    // #[test]
    // fn test_mask_repetition() {
    //     let piece = Piece::new(3, 3, PieceID::from(0), [0b110, 0b010, 0b011, 0, 0, 0, 0, 0]);

    //     let expected = [
    //         0b00110110110110110110,
    //         0b00010010010010010010,
    //         0b00011011011011011011,
    //         0b00110110110110110110,
    //         0b00010010010010010010,
    //         0b00011011011011011011,
    //         0b00000000000000000000,
    //         0b00000000000000000000,
    //     ];

    //     assert!(unsafe { ymm_eq(piece.repeated_mask, ymm(expected),) });
    // }

    #[test]
    fn test_neighbor_mask() {
        let piece = Piece::new(3, 3, PieceID::from(0), [0b110, 0b010, 0b011, 0, 0, 0, 0, 0]);
        println!("{:?}", piece);

        let expected = [0b01100, 0b10010, 0b01010, 0b01001, 0b00110, 0, 0, 0];

        assert!(unsafe { ymm_eq(piece.neighbor_mask, ymm(expected),) });
    }
}
