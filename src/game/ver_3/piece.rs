use crate::game::{Corner, Neighbor};
use std::fmt::Debug;

/// Struct representing a piece
/// All masks in this piece are stored as an __m256i, meaning
/// they can store 32 * 8 bits, or 8 rows of mask information.
pub struct Piece {
    /// Width of the piece
    pub width: i8,
    /// Height of the piece
    pub height: i8,
    /// ID mask of the piece,
    /// contains 1s at all the indices of this piece
    /// and its transformations
    /// used to remove pieces from the player's hand
    pub id_mask: u128,
    /// U16 of the piece
    pub as_u16: u16,
    pub neighbor_mask: [u8; 6],
    pub corner_mask: [u8; 6],
}

impl Piece {
    pub fn new(width: i8, height: i8, id_mask: u128, piece: [u8; 4]) -> Self {
        debug_assert!(width <= 4);
        debug_assert!(height <= 4);

        let as_u16 = piece[0] as u16
            | ((piece[1] as u16) << 4)
            | ((piece[2] as u16) << 8)
            | ((piece[3] as u16) << 12);

        let mut neighbor_mask = [0; 6];
        let mut corner_mask = [0; 6];
        // Generate the neighbor mask
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

        for row in 0..height as usize {
            neighbor_mask[row + 1] |= piece[row] << 1;
        }

        Self {
            id_mask,
            width,
            height,
            as_u16,
            neighbor_mask,
            corner_mask,
        }
    }
}

impl Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = self.corner_mask;
        writeln!(f, "Corner Mask:")?;
        for x in rows.iter() {
            writeln!(f, "{:032b}", x)?;
        }
        writeln!(f, "Neighbor Mask:")?;
        let rows = self.neighbor_mask;
        for x in rows.iter() {
            writeln!(f, "{:032b}", x)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbor_mask() {
        let piece = Piece::new(3, 3, 0, [0b110, 0b010, 0b011, 0]);
        println!("{:?}", piece);

        let expected = [0b01100, 0b11110, 0b01110, 0b01111, 0b00110, 0];

        assert!(piece
            .neighbor_mask
            .iter()
            .zip(expected.iter())
            .all(|(a, b)| a == b));
    }
    #[test]
    fn test_neighbor_mask_one() {
        let piece = Piece::new(1, 1, 0, [1, 0, 0, 0]);
        println!("{:?}", piece);

        let expected = [0b010, 0b111, 0b010, 0, 0, 0];

        assert!(piece
            .neighbor_mask
            .iter()
            .zip(expected.iter())
            .all(|(a, b)| a == b));
    }

    #[test]
    fn test_corner_mask_one() {
        let piece = Piece::new(1, 1, 0, [1, 0, 0, 0]);
        println!("{:?}", piece);

        let expected = [0b101, 0, 0b101, 0, 0, 0];

        assert!(piece
            .corner_mask
            .iter()
            .zip(expected.iter())
            .all(|(a, b)| a == b));
    }

    #[test]
    fn test_corner_mask_l() {
        // Piece has these corners:
        // -  -
        //  XX
        // - X
        //  - -
        let piece = Piece::new(2, 2, 0, [0b11, 0b01, 0, 0]);
        println!("{:?}", piece);

        let expected = [0b1001, 0, 0b1000, 0b101, 0, 0];

        assert!(piece
            .corner_mask
            .iter()
            .zip(expected.iter())
            .all(|(a, b)| a == b));
    }
}
