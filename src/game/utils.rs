use std::ops::Add;

#[derive(Clone, Copy, PartialEq, Eq)]
/// Corner direction mapping.
pub enum Corner {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Corner {
    pub fn iter() -> impl Iterator<Item = Corner> {
        [
            Corner::UpLeft,
            Corner::UpRight,
            Corner::DownLeft,
            Corner::DownRight,
        ]
        .into_iter()
    }
}

impl From<Corner> for usize {
    fn from(value: Corner) -> Self {
        match value {
            Corner::UpLeft => 0,
            Corner::UpRight => 1,
            Corner::DownLeft => 2,
            Corner::DownRight => 3,
        }
    }
}

impl From<usize> for Corner {
    fn from(i: usize) -> Self {
        match i {
            0 => Corner::UpLeft,
            1 => Corner::UpRight,
            2 => Corner::DownLeft,
            3 => Corner::DownRight,
            _ => panic!("Invalid corner"),
        }
    }
}

impl Add<(i32, i32)> for Corner {
    type Output = (i32, i32);

    fn add(self, (x, y): (i32, i32)) -> Self::Output {
        match self {
            Corner::UpLeft => (x + 1, y - 1),
            Corner::UpRight => (x - 1, y - 1),
            Corner::DownLeft => (x + 1, y + 1),
            Corner::DownRight => (x - 1, y + 1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
/// Neighbor direction mapping.
pub enum Neighbor {
    Up,
    Down,
    Left,
    Right,
}

impl Neighbor {
    pub fn iter() -> impl Iterator<Item = Neighbor> {
        [
            Neighbor::Up,
            Neighbor::Down,
            Neighbor::Left,
            Neighbor::Right,
        ]
        .into_iter()
    }
}

impl From<Neighbor> for usize {
    fn from(value: Neighbor) -> Self {
        match value {
            Neighbor::Up => 0,
            Neighbor::Down => 1,
            Neighbor::Left => 2,
            Neighbor::Right => 3,
        }
    }
}

impl From<usize> for Neighbor {
    fn from(i: usize) -> Self {
        match i {
            0 => Neighbor::Up,
            1 => Neighbor::Down,
            2 => Neighbor::Left,
            3 => Neighbor::Right,
            _ => panic!("Invalid neighbor"),
        }
    }
}

impl Add<(i32, i32)> for Neighbor {
    type Output = (i32, i32);

    fn add(self, (x, y): (i32, i32)) -> Self::Output {
        match self {
            Neighbor::Up => (x, y - 1),
            Neighbor::Down => (x, y + 1),
            Neighbor::Left => (x + 1, y),
            Neighbor::Right => (x - 1, y),
        }
    }
}
