use super::Mask;
use crate::game::{Corner, Dimensioned, Neighbor, Reflection, Rotation, Transformation};
use rustc_hash::FxHashSet;
use std::{fmt::Debug, hash::Hash};

/// Rotate a mask by 90 degrees clockwise.
fn rotate(mask: &Mask) -> Mask {
    let width = mask.w();
    let height = mask.h();
    let mut new_mask = Mask::new(height, vec![0; width as usize]);
    for i in 0..width {
        for j in 0..height {
            let bit = mask.get(i, j).unwrap();
            new_mask.set(j, width - i - 1, bit);
        }
    }
    new_mask
}

/// Flip a mask vertically.
fn flip(mask: Mask) -> Mask {
    let w = mask.w();
    let raw: Vec<u128> = mask.into();
    let raw = raw.into_iter().rev().collect();

    Mask::new(w, raw)
}

/// Transform a mask by a transformation.
fn transform(transformation: Transformation, mask: &Mask) -> Mask {
    let Transformation(rotation, reflection) = transformation;
    let mask = match rotation {
        Rotation::Zero => mask.clone(),
        Rotation::Ninety => rotate(mask),
        Rotation::OneEighty => rotate(&rotate(mask)),
        Rotation::TwoSeventy => rotate(&rotate(&rotate(mask))),
    };
    match reflection {
        Reflection::Flip => flip(mask),
        Reflection::NoFlip => mask,
    }
}

/// Transformed piece.
pub struct TransformedPiece {
    /// The mask of the piece.
    pub mask: Mask,
    /// Mask representing the neighbors of the piece
    /// in the board.
    /// If the piece looks like
    ///   01
    ///   11
    /// then the neighbor mask will be
    ///   0010
    ///   01f1
    ///   1ff1
    ///   0110
    pub neighbor_mask: Mask,
    /// Corners of the pieces
    pub corners: [Vec<(i8, i8)>; Corner::N],
}

impl TransformedPiece {
    pub fn new(mask: Mask) -> Self {
        const EMPTY: Vec<(i8, i8)> = Vec::new();
        let mut corners = [EMPTY; Corner::N];

        let mut neighbor_mask = Mask::new(mask.w() + 2, vec![0; (mask.h() + 2) as usize]);
        for i in 0..mask.w() {
            for j in 0..mask.h() {
                let cell = mask.get(i, j).unwrap();
                if cell == 0 {
                    continue;
                }
                // In the neighbor mask, set this cell to F
                neighbor_mask.set_unchecked(i + 1, j + 1, 0xF);
                // set the neighbors to the value of the cell
                for neighbor in Neighbor::iter() {
                    let (x, y) = neighbor + (i, j);

                    neighbor_mask.set_unchecked(x + 1, y + 1, cell);
                }

                // Check if this is a corner for each direction
                for corner in Corner::iter() {
                    let (x, y) = corner + (i, j);

                    let neighbor1 = mask.get_i8(x, j).unwrap_or(0);
                    let neighbor2 = mask.get_i8(i, y).unwrap_or(0);

                    if neighbor1 == 0 && neighbor2 == 0 {
                        debug_assert!(mask.get_i8(x, y).unwrap_or(0) == 0);
                        corners[corner as usize].push((i, j));
                    }
                }
            }
        }

        Self {
            mask,
            neighbor_mask,
            corners,
        }
    }

    /// Iterate over all non-empty cells in the neighbor mask
    /// Returns x, y relative to the top left corner of the piece
    pub fn tile_iter(&self) -> impl Iterator<Item = (i8, i8, u128)> + '_ {
        (0..self.neighbor_mask.w()).flat_map(move |x| {
            (0..self.neighbor_mask.h()).filter_map(move |y| {
                let v = self.neighbor_mask.get(x, y).unwrap();
                if v != 0 {
                    Some((x - 1, y - 1, v))
                } else {
                    None
                }
            })
        })
    }
}

impl Dimensioned for TransformedPiece {
    #[inline]
    /// Get the width of the mask
    fn w(&self) -> i8 {
        self.mask.w()
    }

    #[inline]
    /// Get the height of the mask
    fn h(&self) -> i8 {
        self.mask.h()
    }
}

impl Hash for TransformedPiece {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.mask.hash(state);
    }
}

impl PartialEq for TransformedPiece {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask
    }
}

impl Eq for TransformedPiece {}

impl Debug for TransformedPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.mask, f)
    }
}

/// A piece in the game.
pub struct Piece {
    /// The different unique versions of the piece.
    pub versions: Vec<TransformedPiece>,
}

impl Piece {
    pub fn new(mask: Mask) -> Self {
        let versions: Vec<_> = Transformation::iter()
            .map(|transformation| TransformedPiece::new(transform(transformation, &mask)))
            .collect::<FxHashSet<TransformedPiece>>()
            .into_iter()
            .collect();

        Self { versions }
    }
}

impl Iterator for Piece {
    type Item = TransformedPiece;

    fn next(&mut self) -> Option<Self::Item> {
        self.versions.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        // 01
        // 11
        let mask = Mask::new(2, vec![0x01, 0x11]);
        let rotated = rotate(&mask);
        // 10
        // 11
        assert_eq!(rotated, Mask::new(2, vec![0x10, 0x11]));

        // 011
        // 110
        let mask = Mask::new(3, vec![0x011, 0x110]);
        let rotated = rotate(&mask);
        // 10
        // 11
        // 01
        assert_eq!(rotated, Mask::new(2, vec![0x10, 0x11, 0x01]));
    }

    #[test]
    fn test_flip() {
        let mask = Mask::new(2, vec![0x01, 0x11]);
        let flipped = flip(mask);
        assert_eq!(flipped, Mask::new(2, vec![0x11, 0x01]));
    }

    #[test]
    fn test_transform() {
        let mask = Mask::new(2, vec![0x01, 0x11]);
        let transformation = Transformation(Rotation::OneEighty, Reflection::Flip);
        // 01
        // 11
        // ->
        // 11
        // 10
        // ->
        // 10
        // 11
        let transformed = transform(transformation, &mask);
        assert_eq!(transformed, Mask::new(2, vec![0x10, 0x11]));
    }

    #[test]
    fn test_unique_transformation_count() {
        let mask = Mask::new(3, vec![0x010, 0x111, 0x010]);
        let piece = Piece::new(mask);
        assert_eq!(piece.versions.len(), 1);

        let mask = Mask::new(2, vec![0x01, 0x11]);
        let piece = Piece::new(mask);
        assert_eq!(piece.versions.len(), 4);

        let mask = Mask::new(3, vec![0x011, 0x110]);
        let piece = Piece::new(mask);
        assert_eq!(piece.versions.len(), 4);

        let mask = Mask::new(3, vec![0x100, 0x111]);
        let piece = Piece::new(mask);
        assert_eq!(piece.versions.len(), 8);
    }

    fn eq_orderless<T>(a: &[T], b: &[T])
    where
        T: Eq + Ord + Debug + Clone,
    {
        let mut a = a.to_owned();
        let mut b = b.to_owned();
        a.sort();
        b.sort();
        assert_eq!(a, b);
    }

    #[test]
    fn test_transformed_pieces() {
        let mask = Mask::new(2, vec![0x01, 0x11]);
        let transformed_piece = TransformedPiece::new(mask);

        const REPEAT: Vec<(i8, i8)> = Vec::new();
        let mut expected_corners = [REPEAT; Corner::N];
        // Pos Pos corners are here:
        //  01
        //  X1
        expected_corners[Corner::PosPos as usize].extend(vec![(1, 1)]);
        // Neg Pos corners are here:
        //  01
        //  1X
        expected_corners[Corner::NegPos as usize].extend(vec![(0, 1)]);
        // Pos Neg corners are here:
        //  0X
        //  X1
        expected_corners[Corner::PosNeg as usize].extend(vec![(0, 0), (1, 1)]);

        // Neg Neg corners are here:
        //  0X
        //  11
        expected_corners[Corner::NegNeg as usize].extend(vec![(0, 0)]);

        for (corner, expected) in transformed_piece
            .corners
            .iter()
            .zip(expected_corners.iter())
        {
            eq_orderless(corner, expected);
        }

        let mask = Mask::new(3, vec![0x010, 0x111, 0x010]);
        let transformed_piece = TransformedPiece::new(mask);

        let mut expected_corners = [REPEAT; Corner::N];
        // Pos Pos corners are here:
        //  010
        //  X11
        //  0X0
        expected_corners[Corner::PosPos as usize].extend(vec![(2, 1), (1, 2)]);

        // Neg Pos corners are here:
        //  010
        //  11X
        //  0X0
        expected_corners[Corner::NegPos as usize].extend(vec![(0, 1), (1, 2)]);

        // Pos Neg corners are here:
        //  0X0
        //  X11
        //  010
        expected_corners[Corner::PosNeg as usize].extend(vec![(1, 0), (2, 1)]);

        // Neg Neg corners are here:
        //  0X0
        //  11X
        //  010
        expected_corners[Corner::NegNeg as usize].extend(vec![(1, 0), (0, 1)]);

        for (corner, expected) in transformed_piece
            .corners
            .iter()
            .zip(expected_corners.iter())
        {
            eq_orderless(corner, expected);
        }
    }
}
