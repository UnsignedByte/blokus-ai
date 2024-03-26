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

/// Rotate a mask by 90 degrees clockwise.
fn rotate(mask: &Mask) -> Mask {
    let width = mask.w();
    let height = mask.h();
    let mut new_mask = Mask::new(height, vec![0; width]);
    for i in 0..width {
        for j in 0..height {
            let bit = mask.get(i, j);
            new_mask.set(j, width - i - 1, bit);
        }
    }
    new_mask
}

/// Flip a mask vertically.
fn flip(mask: Mask) -> Mask {
    let w = mask.w();
    let raw: Vec<u64> = mask.into();
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
struct TransformedPiece {
    mask: Mask,
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
    neighbor_mask: Mask,
}

impl TransformedPiece {}

/// A piece in the game.
pub struct Piece {
    /// The different unique versions of the piece.
    versions: Vec<TransformedPiece>,
}

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
}
