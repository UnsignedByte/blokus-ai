use std::{
    fmt::{Debug, Display},
    ops::Add,
};

use ansi_term::Color;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// A piece ID.
pub struct PieceID {
    piece: usize,
}

impl From<usize> for PieceID {
    fn from(piece: usize) -> Self {
        Self { piece }
    }
}

impl From<PieceID> for usize {
    fn from(piece: PieceID) -> Self {
        piece.piece
    }
}

impl From<&PieceID> for usize {
    fn from(piece: &PieceID) -> Self {
        piece.piece
    }
}

impl Debug for PieceID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.piece)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// A piece transform ID.
pub struct PieceTransformID {
    pub piece: PieceID,
    pub version: usize,
}

impl PieceTransformID {
    pub fn new(piece: &PieceID, version: usize) -> Self {
        Self {
            piece: *piece,
            version,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
/// Corner direction mapping.
pub enum Corner {
    PosPos,
    NegPos,
    PosNeg,
    NegNeg,
}

impl Corner {
    pub const N: usize = 4;

    #[inline]
    pub fn iter() -> impl Iterator<Item = Corner> {
        [
            Corner::PosPos,
            Corner::NegPos,
            Corner::PosNeg,
            Corner::NegNeg,
        ]
        .into_iter()
    }

    #[inline]
    pub fn opposite(&self) -> Corner {
        match self {
            Corner::PosPos => Corner::NegNeg,
            Corner::NegPos => Corner::PosNeg,
            Corner::PosNeg => Corner::NegPos,
            Corner::NegNeg => Corner::PosPos,
        }
    }
}

impl From<Corner> for usize {
    fn from(value: Corner) -> Self {
        match value {
            Corner::PosPos => 0,
            Corner::NegPos => 1,
            Corner::PosNeg => 2,
            Corner::NegNeg => 3,
        }
    }
}

impl From<usize> for Corner {
    fn from(i: usize) -> Self {
        match i {
            0 => Corner::PosPos,
            1 => Corner::NegPos,
            2 => Corner::PosNeg,
            3 => Corner::NegNeg,
            _ => panic!("Invalid corner"),
        }
    }
}

impl Add<(i8, i8)> for Corner {
    type Output = (i8, i8);

    fn add(self, (x, y): (i8, i8)) -> Self::Output {
        match self {
            Corner::PosPos => (x + 1, y + 1),
            Corner::NegPos => (x - 1, y + 1),
            Corner::PosNeg => (x + 1, y - 1),
            Corner::NegNeg => (x - 1, y - 1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
/// Neighbor direction mapping.
pub enum Neighbor {
    _Pos,
    _Neg,
    Pos_,
    Neg_,
}

impl Neighbor {
    pub const N: usize = 4;

    #[inline]
    pub fn iter() -> impl Iterator<Item = Neighbor> {
        [
            Neighbor::_Pos,
            Neighbor::_Neg,
            Neighbor::Pos_,
            Neighbor::Neg_,
        ]
        .into_iter()
    }
}

impl From<Neighbor> for usize {
    fn from(value: Neighbor) -> Self {
        match value {
            Neighbor::_Pos => 0,
            Neighbor::_Neg => 1,
            Neighbor::Pos_ => 2,
            Neighbor::Neg_ => 3,
        }
    }
}

impl From<usize> for Neighbor {
    fn from(i: usize) -> Self {
        match i {
            0 => Neighbor::_Pos,
            1 => Neighbor::_Neg,
            2 => Neighbor::Pos_,
            3 => Neighbor::Neg_,
            _ => panic!("Invalid neighbor"),
        }
    }
}

impl Add<(i8, i8)> for Neighbor {
    type Output = (i8, i8);

    fn add(self, (x, y): (i8, i8)) -> Self::Output {
        match self {
            Neighbor::_Pos => (x, y + 1),
            Neighbor::_Neg => (x, y - 1),
            Neighbor::Pos_ => (x + 1, y),
            Neighbor::Neg_ => (x - 1, y),
        }
    }
}

/// Enum for every player.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Player {
    Player1,
    Player2,
    Player3,
    Player4,
}

impl Player {
    pub const N: usize = 4;

    #[inline]
    pub fn iter() -> impl Iterator<Item = Player> {
        [
            Player::Player1,
            Player::Player2,
            Player::Player3,
            Player::Player4,
        ]
        .into_iter()
    }

    pub fn next(&self) -> Player {
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player3,
            Player::Player3 => Player::Player4,
            Player::Player4 => Player::Player1,
        }
    }

    #[inline]
    /// Get the ansi color code for the player.
    pub fn color(&self) -> Color {
        match self {
            Player::Player1 => Color::Red,
            Player::Player2 => Color::Green,  // green
            Player::Player3 => Color::Yellow, // yellow
            Player::Player4 => Color::Blue,   // blue
        }
    }

    #[inline]
    pub const fn mask(&self) -> u128 {
        match self {
            Player::Player1 => 0b0001,
            Player::Player2 => 0b0010,
            Player::Player3 => 0b0100,
            Player::Player4 => 0b1000,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = self.color();
        write!(f, "{}", color.paint(format!("{}", usize::from(self))))
    }
}

impl From<usize> for Player {
    fn from(i: usize) -> Self {
        match i {
            0 => Player::Player1,
            1 => Player::Player2,
            2 => Player::Player3,
            3 => Player::Player4,
            _ => panic!("Invalid player"),
        }
    }
}

impl From<Player> for usize {
    fn from(value: Player) -> Self {
        match value {
            Player::Player1 => 0,
            Player::Player2 => 1,
            Player::Player3 => 2,
            Player::Player4 => 3,
        }
    }
}

impl From<&Player> for usize {
    fn from(value: &Player) -> Self {
        match value {
            Player::Player1 => 0,
            Player::Player2 => 1,
            Player::Player3 => 2,
            Player::Player4 => 3,
        }
    }
}

#[derive(Clone, Hash)]
/// Represents the rotation of a piece.
pub enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

#[derive(Clone, Hash)]
/// Represents the reflection of a piece.
pub enum Reflection {
    Flip,
    NoFlip,
}

#[derive(Clone, Hash)]
/// A transformation
pub struct Transformation(pub Rotation, pub Reflection);

impl Transformation {
    pub fn iter() -> impl Iterator<Item = Transformation> {
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

/// A trait for getting the dimensions of an object.
pub trait Dimensioned {
    fn w(&self) -> i8;

    fn h(&self) -> i8;
}
