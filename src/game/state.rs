use ansi_term::Color;
use itertools::Itertools;

use crate::game::Dimensioned;

use super::{utils::Player, Corner, Mask, Piece};
use std::{collections::HashSet, fmt::Debug};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
/// A piece transform ID.
pub struct PieceTransformID {
    piece: usize,
    version: usize,
}

impl PieceTransformID {
    pub fn new(piece: &PieceID, version: usize) -> Self {
        Self {
            piece: piece.into(),
            version,
        }
    }
}

/// The game state.
pub struct State<'game> {
    board: Mask,
    /// Corners for every player
    /// separated by corner direction
    corners: [[HashSet<(usize, usize)>; Corner::N]; Player::N],
    /// Playable pieces for every player
    player_pieces: [HashSet<PieceID>; Player::N],
    /// All pieces for every player
    pieces: &'game [Vec<Piece>; Player::N],
}

impl<'game> State<'game> {
    pub fn new(w: usize, h: usize, pieces: &'game [Vec<Piece>; Player::N]) -> Self {
        let mut corners: [[HashSet<(usize, usize)>; Corner::N]; Player::N] =
            std::array::from_fn(|_| std::array::from_fn(|_| HashSet::new()));

        // First player starts at the (0, 0) corner
        corners[0][Corner::PosPos as usize].insert((0, 0));
        // Second player starts at the (w - 1, 0) corner
        corners[1][Corner::NegPos as usize].insert((w - 1, 0));
        // Third player starts at the (w - 1, h - 1) corner
        corners[2][Corner::NegNeg as usize].insert((w - 1, h - 1));
        // Fourth player starts at the (0, h - 1) corner
        corners[3][Corner::PosNeg as usize].insert((0, h - 1));

        let player_pieces =
            std::array::from_fn(|i| (0..pieces[i].len()).map(PieceID::from).collect());

        Self {
            board: Mask::new(w, vec![0; h]),
            corners,
            pieces,
            player_pieces,
        }
    }

    fn get_moves_for_piece<'a>(
        &'a self,
        player: usize,
        piece: &'a PieceID,
    ) -> impl Iterator<Item = (PieceTransformID, (usize, usize))> + 'a {
        debug_assert!(
            self.player_pieces[player].contains(piece),
            "Attempted to play a piece that the player doesn't have."
        );

        self.pieces[player][usize::from(piece)]
            .versions
            .iter()
            .enumerate()
            .flat_map(move |(tid, piece_transform)| {
                let tid = PieceTransformID::new(piece, tid);
                let w = piece_transform.w();
                let h = piece_transform.h();

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
                    .map(|((x, y), (dx, dy))| (x as i32 - dx as i32, y as i32 - dy as i32))
                    .unique()
                    // Filter out moves that are out of bounds
                    .filter(|(cx, cy)| {
                        *cx >= 0
                            && *cy >= 0
                            && (*cx + piece_transform.w() as i32) < self.w() as i32
                            && (*cy + piece_transform.h() as i32) < self.h() as i32
                    })
                    // Filter out moves that are in invalid positions
                    // I.E. have neighbors of the same color
                    .filter(|(cx, cy)| {
                        self.board
                            .and(&piece_transform.neighbor_mask, (*cx - 1, *cy - 1))
                            .empty()
                    })
                    .map(move |(cx, cy)| (tid, (cx as usize, cy as usize)))
            })
    }

    /// Get the possible moves for a player
    pub fn get_moves<'a>(
        &'a self,
        player: usize,
    ) -> impl Iterator<Item = (PieceTransformID, (usize, usize))> + 'a {
        // All the different piece transforms for the player
        self.player_pieces[player]
            .iter()
            .flat_map(move |piece| self.get_moves_for_piece(player, piece))
    }
}

impl Dimensioned for State<'_> {
    #[inline]
    fn w(&self) -> usize {
        self.board.w()
    }

    #[inline]
    fn h(&self) -> usize {
        self.board.h()
    }
}

impl Debug for State<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.h() {
            for col in 0..self.w() {
                let cell = self.board.get(col, row).unwrap();
                let color = match cell {
                    0b0000 => None,
                    0b1111 => Some(Color::Purple),
                    0b0001 => Some(Player::Player1.color()),
                    0b0010 => Some(Player::Player2.color()),
                    0b0100 => Some(Player::Player3.color()),
                    0b1000 => Some(Player::Player4.color()),
                    _ => panic!("Invalid cell value"),
                };
                write!(
                    f,
                    "{}",
                    color
                        .map(|color| color.paint("■"))
                        .unwrap_or_else(|| "□".into())
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
