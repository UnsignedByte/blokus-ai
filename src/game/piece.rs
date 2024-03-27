use std::{collections::HashSet, hash::Hash};

use crate::game::{Corner, Dimensioned, Neighbor};

use super::Mask;

#[derive(Clone, Hash)]
/// Represents the rotation of a piece.
enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

#[derive(Clone, Hash)]
/// Represents the reflection of a piece.
enum Reflection {
    Flip,
    NoFlip,
}

#[derive(Clone, Hash)]
/// A transformation
struct Transformation(Rotation, Reflection);

impl Transformation {
    fn iter() -> impl Iterator<Item = Transformation> {
        use Reflection::*;
        use Rotation::*;
        [
            Transformation(Zero, NoFlip),
            Transformation(Ninety, NoFlip),
            Transformation(OneEighty, NoFlip),
            Transformation(TwoSeventy, NoFlip),
            Transformation(Zero, Flip),
            Transformation(Ninety, Flip),
            Transformation(OneEighty, Flip),
            Transformation(TwoSeventy, Flip),
        ]
        .into_iter()
    }
}

/// Rotate a mask by 90 degrees clockwise.
fn rotate(mask: &Mask) -> Mask {
    let width = mask.w();
    let height = mask.h();
    let mut new_mask = Mask::new(height, vec![0; width]);
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
    pub corners: [Vec<(usize, usize)>; Corner::N],
}

impl TransformedPiece {
    pub fn new(mask: Mask) -> Self {
        const EMPTY: Vec<(usize, usize)> = Vec::new();
        let mut corners = [EMPTY; Corner::N];

        let mut neighbor_mask = Mask::new(mask.w() + 2, vec![0; mask.h() + 2]);
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
                    let (x, y) = neighbor + (i as i32, j as i32);

                    neighbor_mask.set_unchecked((x + 1) as usize, (y + 1) as usize, cell);
                }

                // Check if this is a corner for each direction
                for corner in Corner::iter() {
                    let (x, y) = corner + (i as i32, j as i32);

                    let neighbor1 = mask.get_i32(x, j as i32).unwrap_or(0);
                    let neighbor2 = mask.get_i32(i as i32, y).unwrap_or(0);

                    if neighbor1 == 0 && neighbor2 == 0 {
                        debug_assert!(mask.get_i32(x, y).unwrap_or(0) == 0);
                        corners[corner as usize].push((i, j));
                    }
                }
            }
        }

        let neighbor_mask = Mask::new(mask.w(), vec![0; mask.h()]);
        Self {
            neighbor_mask,
            corners,
        }
    }

    /// Iterate over all non-empty cells in the neighbor mask
    pub fn tile_iter(&self) -> impl Iterator<Item = (usize, usize, u128)> + '_ {
        (0..self.neighbor_mask.w()).flat_map(move |x| {
            (0..self.neighbor_mask.h()).filter_map(move |y| {
                let v = self.neighbor_mask.get(x, y).unwrap();
                if v != 0 {
                    Some((x, y, v))
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
    fn w(&self) -> usize {
        self.neighbor_mask.w() - 2
    }

    #[inline]
    /// Get the height of the mask
    fn h(&self) -> usize {
        self.neighbor_mask.h() - 2
    }
}

impl Hash for TransformedPiece {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.neighbor_mask.hash(state);
    }
}

impl PartialEq for TransformedPiece {
    fn eq(&self, other: &Self) -> bool {
        self.neighbor_mask == other.neighbor_mask
    }
}

impl Eq for TransformedPiece {}

/// A piece in the game.
pub struct Piece {
    /// The different unique versions of the piece.
    pub versions: Vec<TransformedPiece>,
}

impl Piece {
    pub fn new(mask: Mask) -> Self {
        let versions: Vec<_> = Transformation::iter()
            .map(|transformation| TransformedPiece::new(transform(transformation, &mask)))
            .collect::<HashSet<TransformedPiece>>()
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
}
