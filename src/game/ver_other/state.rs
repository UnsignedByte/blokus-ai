use super::{Mask, Piece};
use crate::game::{
    utils::{PieceID, PieceTransformID},
    Corner, Dimensioned, Player,
};
use ansi_term::Color;
use core::panic;
use itertools::Itertools;
use once_cell::sync::Lazy;
use rustc_hash::FxHashSet;
use std::fmt::{Debug, Display};

static NEIGHBOR_MASKS: Lazy<[Mask; 4]> = Lazy::new(|| {
    [
        Mask::new(3, vec![0x010, 0x1f1, 0x010]),
        Mask::new(3, vec![0x020, 0x2f2, 0x020]),
        Mask::new(3, vec![0x040, 0x4f4, 0x040]),
        Mask::new(3, vec![0x080, 0x8f8, 0x080]),
    ]
});

pub static PIECES: Lazy<[Vec<Piece>; 4]> = Lazy::new(|| {
    let blocks = vec![
        // 1 tile
        Mask::new(1, vec![0x1]),
        // 2 tiles
        Mask::new(2, vec![0x11]),
        // 3 tiles
        Mask::new(2, vec![0x11, 0x01]),
        Mask::new(3, vec![0x111]),
        // 4 tiles
        Mask::new(4, vec![0x1111]),
        Mask::new(3, vec![0x111, 0x001]),
        Mask::new(3, vec![0x110, 0x011]),
        Mask::new(2, vec![0x11, 0x11]),
        Mask::new(3, vec![0x111, 0x010]),
        // 5 tiles
        Mask::new(3, vec![0x011, 0x110, 0x010]),
        Mask::new(5, vec![0x11111]),
        Mask::new(4, vec![0x1111, 0x1000]),
        Mask::new(4, vec![0x0111, 0x1100]),
        Mask::new(3, vec![0x111, 0x110]),
        Mask::new(3, vec![0x111, 0x010, 0x010]),
        Mask::new(3, vec![0x111, 0x101]),
        Mask::new(3, vec![0x111, 0x100, 0x100]),
        Mask::new(3, vec![0x001, 0x011, 0x110]),
        Mask::new(3, vec![0x010, 0x111, 0x010]),
        Mask::new(4, vec![0x1111, 0x0100]),
        Mask::new(3, vec![0x110, 0x010, 0x011]),
    ];

    // Uses a hack to generate the pieces for all 4 players.
    // Given a piece that looks like
    // 010
    // 111
    // for example, note that shifting each row to the left by one
    // gives the piece
    // 020
    // 222
    // which is the same piece for player 2.
    // This is done for all 4 players.

    [
        blocks.clone().into_iter().map(Piece::new).collect(),
        blocks
            .clone()
            .into_iter()
            .map(|block| block << 1)
            .map(Piece::new)
            .collect(),
        blocks
            .clone()
            .into_iter()
            .map(|block| block << 2)
            .map(Piece::new)
            .collect(),
        blocks
            .into_iter()
            .map(|block| block << 3)
            .map(Piece::new)
            .collect(),
    ]
});

/// A move.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Move {
    pub piece: PieceTransformID,
    pub pos: (i8, i8),
}

impl Move {
    pub fn new(piece: PieceTransformID, pos: (i8, i8)) -> Self {
        Self { piece, pos }
    }
}

impl From<(PieceTransformID, (i8, i8))> for Move {
    fn from((piece, pos): (PieceTransformID, (i8, i8))) -> Self {
        Self { piece, pos }
    }
}

impl From<Move> for (PieceTransformID, (i8, i8)) {
    fn from(m: Move) -> Self {
        (m.piece, m.pos)
    }
}

/// The game state.
pub struct State {
    board: Mask,
    /// Corners for every player
    /// separated by corner direction
    corners: [[FxHashSet<(i8, i8)>; Corner::N]; Player::N],
    /// Playable pieces for every player
    player_pieces: [Vec<bool>; Player::N],
}

impl State {
    pub fn new(w: i8, h: i8) -> Self {
        let mut corners: [[FxHashSet<(i8, i8)>; Corner::N]; Player::N] =
            std::array::from_fn(|_| std::array::from_fn(|_| FxHashSet::default()));

        // First player starts at the (0, 0) corner
        corners[usize::from(Player::Player1)][Corner::PosPos as usize].insert((0, 0));
        // Second player starts at the (w - 1, 0) corner
        corners[usize::from(Player::Player2)][Corner::NegPos as usize].insert((w - 1, 0));
        // Third player starts at the (w - 1, h - 1) corner
        corners[usize::from(Player::Player3)][Corner::NegNeg as usize].insert((w - 1, h - 1));
        // Fourth player starts at the (0, h - 1) corner
        corners[usize::from(Player::Player4)][Corner::PosNeg as usize].insert((0, h - 1));

        let player_pieces = std::array::from_fn(|i| vec![true; PIECES[i].len()]);

        Self {
            board: Mask::new(w, vec![0; h as usize]),
            corners,
            player_pieces,
        }
    }

    fn get_moves_for_piece<'a>(
        &'a self,
        player: &Player,
        piece: PieceID,
    ) -> impl Iterator<Item = Move> + 'a {
        let player = usize::from(player);
        debug_assert!(
            self.player_pieces[player][usize::from(piece)],
            "Attempted to play a piece that the player doesn't have."
        );

        PIECES[player][usize::from(piece)]
            .versions
            .iter()
            .enumerate()
            .flat_map(move |(tid, piece_transform)| {
                let tid = PieceTransformID::new(&piece, tid);

                // Set of possible moves to try
                self.corners[player]
                    .iter()
                    .enumerate()
                    // Get all pairs of opposite direction corners
                    .flat_map(move |(corner, corners)| {
                        corners.iter().copied().cartesian_product(
                            piece_transform.corners[usize::from(Corner::from(corner).opposite())]
                                .iter()
                                .copied(),
                        )
                    })
                    // Map to the top left corner positions of the transformed piece
                    .map(|((x, y), (dx, dy))| (x - dx, y - dy))
                    // Filter out moves that are out of bounds
                    .filter(|(cx, cy)| {
                        *cx >= 0
                            && *cy >= 0
                            && (*cx + piece_transform.w()) <= self.w()
                            && (*cy + piece_transform.h()) <= self.h()
                    })
                    // Filter out moves that are in invalid positions
                    // I.E. have neighbors of the same color
                    .filter(|(cx, cy)| {
                        self.board
                            .no_overlap(&piece_transform.neighbor_mask, (*cx - 1, *cy - 1))
                    })
                    .map(move |(cx, cy)| Move::new(tid, (cx, cy)))
            })
    }

    /// Get the possible moves for a player
    pub fn get_moves<'a>(&'a self, player: &'a Player) -> impl Iterator<Item = Move> + 'a {
        // All the different piece transforms for the player
        self.player_pieces[usize::from(player)]
            .iter()
            .enumerate()
            .filter(|(_, v)| **v)
            .flat_map(move |(piece, _)| self.get_moves_for_piece(player, PieceID::from(piece)))
    }

    /// Place a piece on the board
    pub fn place_piece(&mut self, player: &Player, mv: &Move) {
        let Move { piece, pos } = *mv;

        let transformed_piece =
            &PIECES[usize::from(player)][usize::from(piece.piece)].versions[piece.version];

        let (x, y) = pos;

        // Check if the piece can be placed
        debug_assert!(
            self.board
                .no_overlap(&transformed_piece.neighbor_mask, (x - 1, y - 1)),
            "Position already contained filled tiles."
        );

        // Place the piece on the board
        self.board = self.board.or(&transformed_piece.mask, (x, y));
        // Remove the piece from the player's pieces
        self.player_pieces[usize::from(player)][usize::from(piece.piece)] = false;

        // Update the corners
        for (x, y, v) in transformed_piece.tile_iter() {
            let x = x + pos.0;
            let y = y + pos.1;
            if x < 0 || y < 0 {
                continue;
            }
            match v {
                0b1111 => {
                    for cornerset in self.corners.iter_mut() {
                        for corner in cornerset.iter_mut() {
                            corner.remove(&(x, y));
                        }
                    }
                }
                p if p == player.mask() => {
                    for corner in self.corners[usize::from(player)].iter_mut() {
                        corner.remove(&(x, y));
                    }
                }
                _ => panic!("Invalid tile value"),
            }
        }

        // add new corners
        for (corner, corners) in transformed_piece.corners.iter().enumerate() {
            let corner = Corner::from(corner);
            for (x, y) in corners.iter().copied() {
                let x = x + pos.0;
                let y = y + pos.1;
                let (x, y) = corner + (x, y);
                if x >= 0
                    && y >= 0
                    && x < self.w()
                    && y < self.h()
                    && self
                        .board
                        .no_overlap(&NEIGHBOR_MASKS[usize::from(player)], (x - 1, y - 1))
                {
                    self.corners[usize::from(player)][usize::from(corner)].insert((x, y));
                }
            }
        }
    }
}

impl Dimensioned for State {
    #[inline]
    fn w(&self) -> i8 {
        self.board.w()
    }

    #[inline]
    fn h(&self) -> i8 {
        self.board.h()
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.h() {
            for col in 0..self.w() {
                let cell = self.board.get(col, row).unwrap();
                let color = match cell {
                    0b0000 => None,
                    0b1111 => Some(Color::Purple),
                    x if x == Player::Player1.mask() => Some(Player::Player1.color()),
                    x if x == Player::Player2.mask() => Some(Player::Player2.color()),
                    x if x == Player::Player3.mask() => Some(Player::Player3.color()),
                    x if x == Player::Player4.mask() => Some(Player::Player4.color()),
                    _ => panic!("Invalid cell value"),
                };
                write!(
                    f,
                    "{}",
                    color
                        .map(|color| color.paint(format!("{:x}", cell)))
                        .unwrap_or_else(|| format!("{:x}", cell).into())
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)?;

        Ok(())
    }
}
