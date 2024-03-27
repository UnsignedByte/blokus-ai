use std::ops::Add;

use ansi_term::Color;

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

impl Add<(i32, i32)> for Corner {
    type Output = (i32, i32);

    fn add(self, (x, y): (i32, i32)) -> Self::Output {
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

impl Add<(i32, i32)> for Neighbor {
    type Output = (i32, i32);

    fn add(self, (x, y): (i32, i32)) -> Self::Output {
        match self {
            Neighbor::_Pos => (x, y + 1),
            Neighbor::_Neg => (x, y - 1),
            Neighbor::Pos_ => (x + 1, y),
            Neighbor::Neg_ => (x - 1, y),
        }
    }
}

/// Enum for every player.
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

/// A trait for getting the dimensions of an object.
pub trait Dimensioned {
    fn w(&self) -> usize;

    fn h(&self) -> usize;
}
