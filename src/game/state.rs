use itertools::Itertools;

use crate::game::Dimensioned;

use super::{Corner, Mask, Piece, TransformedPiece};
use std::collections::HashSet;

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
    /// separated by direction
    corners: [[HashSet<(usize, usize)>; 4]; 4],
    /// Pieces for every player
    player_pieces: [HashSet<PieceID>; 4],
    /// All pieces
    pieces: &'game [Vec<Piece>; 4],
}

impl<'game> State<'game> {
    pub fn new(w: usize, h: usize, pieces: &'game [Vec<Piece>; 4]) -> Self {
        let mut corners: [[HashSet<(usize, usize)>; 4]; 4] =
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

    #[inline]
    /// Get a piece transform from a piece transform ID
    fn get_piece_transform(
        &self,
        player: usize,
        piece_transform: PieceTransformID,
    ) -> &TransformedPiece {
        &self.pieces[player][piece_transform.piece].versions[piece_transform.version]
    }

    fn get_moves_for_piece<'a>(
        &'a self,
        player: usize,
        piece: &'a PieceID,
    ) -> impl Iterator<Item = (PieceTransformID, (usize, usize))> + 'a {
        debug_assert!(
            self.player_pieces[player].contains(&piece),
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
