use once_cell::sync::Lazy;

use super::{
    utils::{rotate_down_1, shift_up_1, ymm},
    Piece,
};
use crate::game::Player;
use std::arch::x86_64::*;

const PIECE_COUNT: usize = 91;

pub static PIECES: Lazy<[Piece; PIECE_COUNT]> = Lazy::new(|| {
    [
        // 1 tile pieces
        // X - no transformations
        Piece::new(1, 1, 0b1, [0b1, 0, 0, 0, 0, 0, 0, 0]),
        // 2 tile pieces
        // XX - 2 states
        Piece::new(2, 1, 0b11 << 1, [0b11, 0, 0, 0, 0, 0, 0, 0]),
        Piece::new(1, 2, 0b11 << 1, [0b1, 0b1, 0, 0, 0, 0, 0, 0]),
        // 3 tile pieces
        // XXX - 2 states
        Piece::new(3, 1, 0b11 << 3, [0b111, 0, 0, 0, 0, 0, 0, 0]),
        Piece::new(1, 3, 0b11 << 3, [0b1, 0b1, 0b1, 0, 0, 0, 0, 0]),
        // XX
        // X  - 4 states
        Piece::new(2, 2, 0b1111 << 5, [0b11, 0b1, 0, 0, 0, 0, 0, 0]),
        Piece::new(2, 2, 0b1111 << 5, [0b11, 0b10, 0, 0, 0, 0, 0, 0]),
        Piece::new(2, 2, 0b1111 << 5, [0b1, 0b11, 0, 0, 0, 0, 0, 0]),
        Piece::new(2, 2, 0b1111 << 5, [0b10, 0b11, 0, 0, 0, 0, 0, 0]),
        // 4 tile pieces
        // XXXX - 2 states
        Piece::new(4, 1, 0b11 << 9, [0b1111, 0, 0, 0, 0, 0, 0, 0]),
        Piece::new(1, 4, 0b11 << 9, [0b1, 0b1, 0b1, 0b1, 0, 0, 0, 0]),
        // XXX
        // X   - 8 states
        Piece::new(3, 2, 0b11111111 << 11, [0b111, 0b1, 0, 0, 0, 0, 0, 0]),
        Piece::new(3, 2, 0b11111111 << 11, [0b111, 0b100, 0, 0, 0, 0, 0, 0]),
        Piece::new(3, 2, 0b11111111 << 11, [0b1, 0b111, 0, 0, 0, 0, 0, 0]),
        Piece::new(3, 2, 0b11111111 << 11, [0b100, 0b111, 0, 0, 0, 0, 0, 0]),
        Piece::new(2, 3, 0b11111111 << 11, [0b11, 0b10, 0b10, 0, 0, 0, 0, 0]),
        Piece::new(2, 3, 0b11111111 << 11, [0b11, 0b1, 0b1, 0, 0, 0, 0, 0]),
        Piece::new(2, 3, 0b11111111 << 11, [0b1, 0b1, 0b11, 0, 0, 0, 0, 0]),
        Piece::new(2, 3, 0b11111111 << 11, [0b10, 0b10, 0b11, 0, 0, 0, 0, 0]),
        // XX
        //  XX - 4 states
        Piece::new(3, 2, 0b1111 << 19, [0b110, 0b011, 0, 0, 0, 0, 0, 0]),
        Piece::new(3, 2, 0b1111 << 19, [0b011, 0b110, 0, 0, 0, 0, 0, 0]),
        Piece::new(2, 3, 0b1111 << 19, [0b10, 0b11, 0b01, 0, 0, 0, 0, 0]),
        Piece::new(2, 3, 0b1111 << 19, [0b01, 0b11, 0b10, 0, 0, 0, 0, 0]),
        // XXX
        //  X - 4 states
        Piece::new(3, 2, 0b1111 << 23, [0b111, 0b010, 0, 0, 0, 0, 0, 0]),
        Piece::new(3, 2, 0b1111 << 23, [0b010, 0b111, 0, 0, 0, 0, 0, 0]),
        Piece::new(2, 3, 0b1111 << 23, [0b10, 0b11, 0b10, 0, 0, 0, 0, 0]),
        Piece::new(2, 3, 0b1111 << 23, [0b01, 0b11, 0b01, 0, 0, 0, 0, 0]),
        // XX
        // XX - 1 state
        Piece::new(2, 2, 0b1 << 27, [0b11, 0b11, 0, 0, 0, 0, 0, 0]),
        // 5 tile pieces
        // XXXXX - 2 states
        Piece::new(5, 1, 0b11 << 28, [0b11111, 0, 0, 0, 0, 0, 0, 0]),
        Piece::new(1, 5, 0b11 << 28, [0b1, 0b1, 0b1, 0b1, 0b1, 0, 0, 0]),
        //  XX
        // XX
        //  X - 8 states
        Piece::new(3, 3, 0b11111111 << 30, [0b110, 0b011, 0b010, 0, 0, 0, 0, 0]),
        Piece::new(3, 3, 0b11111111 << 30, [0b011, 0b110, 0b010, 0, 0, 0, 0, 0]),
        Piece::new(3, 3, 0b11111111 << 30, [0b010, 0b110, 0b011, 0, 0, 0, 0, 0]),
        Piece::new(3, 3, 0b11111111 << 30, [0b010, 0b011, 0b110, 0, 0, 0, 0, 0]),
        Piece::new(3, 3, 0b11111111 << 30, [0b010, 0b111, 0b001, 0, 0, 0, 0, 0]),
        Piece::new(3, 3, 0b11111111 << 30, [0b001, 0b111, 0b010, 0, 0, 0, 0, 0]),
        Piece::new(3, 3, 0b11111111 << 30, [0b010, 0b111, 0b100, 0, 0, 0, 0, 0]),
        Piece::new(3, 3, 0b11111111 << 30, [0b100, 0b111, 0b010, 0, 0, 0, 0, 0]),
        // XXXX
        // X    - 8 states
        Piece::new(4, 2, 0b11111111 << 38, [0b1111, 0b0001, 0, 0, 0, 0, 0, 0]),
        Piece::new(4, 2, 0b11111111 << 38, [0b1111, 0b1000, 0, 0, 0, 0, 0, 0]),
        Piece::new(4, 2, 0b11111111 << 38, [0b1000, 0b1111, 0, 0, 0, 0, 0, 0]),
        Piece::new(4, 2, 0b11111111 << 38, [0b0001, 0b1111, 0, 0, 0, 0, 0, 0]),
        Piece::new(2, 4, 0b11111111 << 38, [0b11, 0b10, 0b10, 0b10, 0, 0, 0, 0]),
        Piece::new(2, 4, 0b11111111 << 38, [0b11, 0b01, 0b01, 0b01, 0, 0, 0, 0]),
        Piece::new(2, 4, 0b11111111 << 38, [0b10, 0b10, 0b10, 0b11, 0, 0, 0, 0]),
        Piece::new(2, 4, 0b11111111 << 38, [0b01, 0b01, 0b01, 0b11, 0, 0, 0, 0]),
        // XXX
        //   XX - 8 states
        Piece::new(4, 2, 0b11111111 << 46, [0b1110, 0b0011, 0b0, 0, 0, 0, 0, 0]),
        Piece::new(4, 2, 0b11111111 << 46, [0b0111, 0b1100, 0b0, 0, 0, 0, 0, 0]),
        Piece::new(4, 2, 0b11111111 << 46, [0b0011, 0b1110, 0b0, 0, 0, 0, 0, 0]),
        Piece::new(4, 2, 0b11111111 << 46, [0b1100, 0b0111, 0b0, 0, 0, 0, 0, 0]),
        Piece::new(2, 4, 0b11111111 << 46, [0b10, 0b10, 0b11, 0b01, 0, 0, 0, 0]),
        Piece::new(2, 4, 0b11111111 << 46, [0b01, 0b01, 0b11, 0b10, 0, 0, 0, 0]),
        Piece::new(2, 4, 0b11111111 << 46, [0b10, 0b11, 0b01, 0b01, 0, 0, 0, 0]),
        Piece::new(2, 4, 0b11111111 << 46, [0b01, 0b11, 0b10, 0b10, 0, 0, 0, 0]),
        // XX
        // XXX - 8 states
        Piece::new(3, 2, 0b11111111 << 54, [0b110, 0b111, 0, 0, 0, 0, 0, 0]),
        Piece::new(3, 2, 0b11111111 << 54, [0b111, 0b110, 0, 0, 0, 0, 0, 0]),
        Piece::new(3, 2, 0b11111111 << 54, [0b011, 0b111, 0, 0, 0, 0, 0, 0]),
        Piece::new(3, 2, 0b11111111 << 54, [0b111, 0b011, 0, 0, 0, 0, 0, 0]),
        Piece::new(2, 3, 0b11111111 << 54, [0b11, 0b11, 0b01, 0, 0, 0, 0, 0]),
        Piece::new(2, 3, 0b11111111 << 54, [0b11, 0b11, 0b10, 0, 0, 0, 0, 0]),
        Piece::new(2, 3, 0b11111111 << 54, [0b01, 0b11, 0b11, 0, 0, 0, 0, 0]),
        Piece::new(2, 3, 0b11111111 << 54, [0b10, 0b11, 0b11, 0, 0, 0, 0, 0]),
        // XXX
        //  X
        //  X - 4 states
        Piece::new(3, 3, 0b1111 << 62, [0b111, 0b010, 0b010, 0, 0, 0, 0, 0]),
        Piece::new(3, 3, 0b1111 << 62, [0b010, 0b010, 0b111, 0, 0, 0, 0, 0]),
        Piece::new(3, 3, 0b1111 << 62, [0b001, 0b111, 0b001, 0, 0, 0, 0, 0]),
        Piece::new(3, 3, 0b1111 << 62, [0b100, 0b111, 0b100, 0, 0, 0, 0, 0]),
        // X X
        // XXX - 4 states
        Piece::new(3, 2, 0b1111 << 66, [0b101, 0b111, 0, 0, 0, 0, 0, 0]),
        Piece::new(3, 2, 0b1111 << 66, [0b111, 0b101, 0, 0, 0, 0, 0, 0]),
        Piece::new(2, 3, 0b1111 << 66, [0b11, 0b10, 0b11, 0, 0, 0, 0, 0]),
        Piece::new(2, 3, 0b1111 << 66, [0b11, 0b01, 0b11, 0, 0, 0, 0, 0]),
        // X
        // X
        // XXX - 4 states
        Piece::new(3, 3, 0b1111 << 70, [0b100, 0b100, 0b111, 0, 0, 0, 0, 0]),
        Piece::new(3, 3, 0b1111 << 70, [0b001, 0b001, 0b111, 0, 0, 0, 0, 0]),
        Piece::new(3, 3, 0b1111 << 70, [0b111, 0b100, 0b100, 0, 0, 0, 0, 0]),
        Piece::new(3, 3, 0b1111 << 70, [0b111, 0b001, 0b001, 0, 0, 0, 0, 0]),
        // X
        // XX
        //  XX - 4 states
        Piece::new(3, 3, 0b1111 << 74, [0b100, 0b110, 0b011, 0, 0, 0, 0, 0]),
        Piece::new(3, 3, 0b1111 << 74, [0b011, 0b110, 0b100, 0, 0, 0, 0, 0]),
        Piece::new(3, 3, 0b1111 << 74, [0b001, 0b011, 0b110, 0, 0, 0, 0, 0]),
        Piece::new(3, 3, 0b1111 << 74, [0b110, 0b011, 0b001, 0, 0, 0, 0, 0]),
        //  X
        // XXX
        //  X  - 1 state
        Piece::new(3, 3, 0b1 << 78, [0b010, 0b111, 0b010, 0, 0, 0, 0, 0]),
        //   X
        // XXXX - 8 states
        Piece::new(4, 2, 0b11111111 << 79, [0b1111, 0b0010, 0, 0, 0, 0, 0, 0]),
        Piece::new(4, 2, 0b11111111 << 79, [0b1111, 0b0100, 0, 0, 0, 0, 0, 0]),
        Piece::new(4, 2, 0b11111111 << 79, [0b0010, 0b1111, 0, 0, 0, 0, 0, 0]),
        Piece::new(4, 2, 0b11111111 << 79, [0b0100, 0b1111, 0, 0, 0, 0, 0, 0]),
        Piece::new(2, 4, 0b11111111 << 79, [0b01, 0b01, 0b11, 0b01, 0, 0, 0, 0]),
        Piece::new(2, 4, 0b11111111 << 79, [0b01, 0b11, 0b01, 0b01, 0, 0, 0, 0]),
        Piece::new(2, 4, 0b11111111 << 79, [0b10, 0b10, 0b11, 0b10, 0, 0, 0, 0]),
        Piece::new(2, 4, 0b11111111 << 79, [0b10, 0b11, 0b10, 0b10, 0, 0, 0, 0]),
        // XX
        //  X
        //  XX - 4 states
        Piece::new(3, 3, 0b1111 << 87, [0b110, 0b010, 0b011, 0, 0, 0, 0, 0]),
        Piece::new(3, 3, 0b1111 << 87, [0b011, 0b010, 0b110, 0, 0, 0, 0, 0]),
        Piece::new(3, 3, 0b1111 << 87, [0b001, 0b111, 0b100, 0, 0, 0, 0, 0]),
        Piece::new(3, 3, 0b1111 << 87, [0b100, 0b111, 0b001, 0, 0, 0, 0, 0]),
    ]
});

/// A move.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Move {
    pub piece: usize,
    pub pos: (i8, i8),
}

impl Move {
    pub fn new(piece: usize, pos: (i8, i8)) -> Self {
        Self { piece, pos }
    }
}

/// The game state
pub struct State {
    /// Occupation mask (the bitwise or of all the colormasks)
    occupied_mask: [u32; 20],
    /// Occupation masks for each player
    color_masks: [[u32; 20]; Player::N],
    /// Corner masks for each player
    corner_masks: [[u32; 20]; Player::N],
    /// Playable pieces for every player
    /// stored as a [u128] where the lower [PIECE_COUNT] bits
    /// represent whether a player has the piece
    /// on hand or not
    player_pieces: [u128; Player::N],
}

impl State {
    pub fn new() -> Self {
        // Starting corners for each player
        let corner_masks = [
            // Player 1 gets
            // 0b0000...0001
            // 0b0000...0000
            //    :   :   :
            // 0b0000...0000
            // 0b0000...0000
            {
                let mut board = [0; 20];
                board[0] = 1;
                board
            },
            // Player 2 gets
            // 0b0000...0000
            // 0b0000...0000
            //    :   :   :
            // 0b0000...0000
            // 0b0000...0001
            {
                let mut board = [0; 20];
                board[19] = 1;
                board
            },
            // Player 3 gets
            // 0b0000...0000
            // 0b0000...0000
            //    :   :   :
            // 0b0000...0000
            // 0b1000...0000
            {
                let mut board = [0; 20];
                board[19] = 1 << 19;
                board
            },
            // Player 4 gets
            // 0b1000...0000
            // 0b0000...0000
            //    :   :   :
            // 0b0000...0000
            // 0b0000...0000
            {
                let mut board = [0; 20];
                board[0] = 1 << 19;
                board
            },
        ];

        Self {
            occupied_mask: [0; 20],
            color_masks: [[0; 20]; Player::N],
            corner_masks,
            player_pieces: [(1 << PIECE_COUNT) - 1; Player::N], // Players start with all the pieces
        }
    }

    /// Get a checker for rows [offset+2, offset+4].
    /// If we have a "piece" stored with the same format in an __m256i, then
    /// none(piece & occupied) & some(piece & corners) means that the piece fits in those 8 rows
    #[inline]
    fn get_checker(
        &self,
        player: &Player,
        offset: usize,
    ) -> (
        __m256i, /* occupied */
        __m256i,
        /* corners */ __m256i, /* colors */
    ) {
        debug_assert!(offset >= 1);
        unsafe {
            (
                _mm256_loadu_si256(
                    self.occupied_mask.as_ptr().wrapping_add(offset) as *const __m256i
                ),
                _mm256_loadu_si256(
                    self.corner_masks[usize::from(player)]
                        .as_ptr()
                        .wrapping_add(offset) as *const __m256i,
                ),
                _mm256_loadu_si256(
                    self.color_masks[usize::from(player)]
                        .as_ptr()
                        // This is moved back 1 because the neighbor mask is expanded
                        .wrapping_add(offset - 1) as *const __m256i,
                ),
            )
        }
    }

    #[inline]
    fn check(
        (occupied, corners, color): (__m256i, __m256i, __m256i),
        (shape, neighbors): (__m256i, __m256i),
    ) -> bool {
        unsafe {
            // First check if its a valid corner
            _mm256_testz_si256(corners, shape) != 0
            // then check if this is unoccupied
            && _mm256_testz_si256(occupied, shape) == 0
            // finally check if the neighbors mask is empty
            && _mm256_testz_si256(color, neighbors) == 0
        }
    }

    fn get_moves_for_piece(&self, player: &Player, pieceid: usize) -> impl Iterator<Item = Move> {
        let piece = &PIECES[pieceid];
        // The number of rows we need to check
        let to_check = 20 - piece.width + 1;

        // There are up to 20^2 = 400 possible moves
        let mut moves: Vec<(i8, i8)> = Vec::with_capacity(to_check as usize * to_check as usize);

        let row_pair = 0;
        // Check
        let checker = unsafe {
            (
                ymm(self.occupied_mask[0..8].try_into().unwrap()),
                ymm(self.corner_masks[usize::from(player)][0..8]
                    .try_into()
                    .unwrap()),
                ymm(self.color_masks[usize::from(player)][0..8]
                    .try_into()
                    .unwrap()),
            )
        };

        let row1 = (piece.occupied_mask, unsafe {
            shift_up_1(piece.neighbor_mask)
        });

        let row2 = (
            unsafe { rotate_down_1(piece.occupied_mask) },
            piece.neighbor_mask,
        );

        for i in 0..to_check {
            if Self::check(checker, row1) {
                moves.push((i, 0));
            }

            if Self::check(checker, row2) {
                moves.push((i, 1));
            }
        }

        // Check two rows at a time
        for row_pair in 1..(to_check / 2) {
            // check every pair of rows
            let checker = self.get_checker(player, 2 * row_pair as usize);

            let row1 = (piece.occupied_mask, piece.neighbor_mask);

            let row2 = unsafe {
                (
                    rotate_down_1(piece.occupied_mask),
                    rotate_down_1(piece.neighbor_mask),
                )
            };

            for i in 0..to_check {
                if Self::check(checker, row1) {
                    moves.push((i, row_pair * 2));
                }

                if Self::check(checker, row2) {
                    moves.push((i, row_pair * 2 + 1));
                }
            }
        }

        if piece.height % 2 == 0 {
            // Check the final row
            let checker = self.get_checker(player, (to_check / 2) as usize);

            let row1 = (piece.occupied_mask, piece.neighbor_mask);

            for i in 0..to_check {
                if Self::check(checker, row1) {
                    moves.push((i, row_pair * 2));
                }
            }
        }

        moves.into_iter().map(move |pos| Move::new(pieceid, pos))
    }

    /// Get the possible moves for a player
    pub fn get_moves<'a>(&'a self, player: &'a Player) -> impl Iterator<Item = Move> + 'a {
        // All the different piece transforms for the player
        (0..PIECE_COUNT)
            .filter(move |f| (1 << *f) & self.player_pieces[usize::from(player)] != 0)
            .flat_map(move |piece| self.get_moves_for_piece(player, piece))
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
