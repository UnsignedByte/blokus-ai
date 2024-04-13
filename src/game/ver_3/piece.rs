use crate::game::{Corner, Neighbor};

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

        let as_u16 = piece[0] | (piece[1] << 4) | (piece[2] << 8) | (piece[3] << 12);
        let as_u16 = as_u16 as u16;

        let mut neighbor_mask = [0; 6];
        let mut corner_mask = [0; 6];

        for x in 0..4 {
            for y in 0..4 {
                neighbor_mask[y + 1] |= piece[y] << (x + 1);
                for dir in Neighbor::iter() {
                    let (nx, ny) = dir + (x as i8, y as i8);

                    neighbor_mask[(ny + 1) as usize] |= piece[ny as usize] << (nx + 1);
                }

                for corner in Corner::iter() {
                    // if both neighbors are empty, then this is a corner
                    let (x, y) = (x as i8, y as i8);
                    let (cx, cy) = corner + (x, y);

                    if (piece[y as usize] >> x) & 1 != 0
                        && (neighbor_mask[(y + 1) as usize] >> (cx + 1)) & 1 == 0
                        && (neighbor_mask[(cy + 1) as usize] >> (x + 1)) & 1 == 0
                    {
                        corner_mask[(cy + 1) as usize] |= 1 << (cx + 1);
                    }
                }
            }
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
