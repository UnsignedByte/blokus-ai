use ansi_term::Color;
use itertools::Itertools;

use crate::game::Dimensioned;

use super::{utils::Player, Corner, Mask, Piece};
use core::panic;
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
    piece: PieceID,
    version: usize,
}

impl PieceTransformID {
    pub fn new(piece: &PieceID, version: usize) -> Self {
        Self {
            piece: *piece,
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
        player: &Player,
        piece: &'a PieceID,
    ) -> impl Iterator<Item = (PieceTransformID, (usize, usize))> + 'a {
        let player = usize::from(player);
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
        player: &'a Player,
    ) -> impl Iterator<Item = (PieceTransformID, (usize, usize))> + 'a {
        // All the different piece transforms for the player
        self.player_pieces[usize::from(player)]
            .iter()
            .flat_map(move |piece| self.get_moves_for_piece(player, piece))
    }

    /// Place a piece on the board
    pub fn place_piece(&mut self, player: &Player, piece: PieceTransformID, pos: (usize, usize)) {
        let transformed_piece =
            &self.pieces[usize::from(player)][usize::from(piece.piece)].versions[piece.version];

        let (x, y) = pos;

        // Check if the piece can be placed
        debug_assert!(
            self.board
                .and(&transformed_piece.neighbor_mask, (x as i32, y as i32))
                .empty(),
            "Position already contained filled tiles."
        );
        debug_assert!(
            self.corners[usize::from(player)]
                .iter()
                .all(|corners| corners.contains(&(x, y))),
            "Piece was not in a corner."
        );

        // Place the piece on the board
        self.board
            .or(&transformed_piece.neighbor_mask, (x as i32, y as i32));
        // Remove the piece from the player's pieces
        self.player_pieces[usize::from(player)].remove(&piece.piece);

        // Update the corners
        for (x, y, v) in transformed_piece.tile_iter() {
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
                        .map(|color| color.paint("■"))
                        .unwrap_or_else(|| "□".into())
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
