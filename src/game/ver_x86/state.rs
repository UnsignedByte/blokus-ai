use once_cell::sync::Lazy;

use super::{
    utils::{rotate_down_1, shift_left_1, shift_up_1, ymm},
    Piece,
};
use crate::game::{ver_x86::utils::ymm_str, Player};
use std::{
    arch::x86_64::*,
    array,
    cmp::min,
    fmt::{Debug, Display},
};

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
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
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
    /// this includes the neighbors of occupied cells,
    /// as these tiles are not playable by the same
    /// color
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
    pub fn new(w: usize, h: usize) -> Self {
        debug_assert!(w == 20 && h == 20);
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
                        .wrapping_add(offset) as *const __m256i,
                ),
            )
        }
    }

    #[inline]
    fn check((occupied, corners, color): (__m256i, __m256i, __m256i), shape: __m256i) -> bool {
        unsafe {
            // First check if its a valid corner
            // testz returns 0 if the result of the & is >0
            // and 1 if the result is 0
            _mm256_testz_si256(corners, shape) == 0
            // then check if this is unoccupied
            // we want the result to be 0
            && _mm256_testz_si256(occupied, shape) != 0
            // finally check if the neighbors mask is empty
            // we want the result to be 0
            && _mm256_testz_si256(color, shape) != 0
        }
    }

    fn get_moves_for_piece(&self, player: &Player, pieceid: usize) -> impl Iterator<Item = Move> {
        let piece = &PIECES[pieceid];
        // The number of rows we need to check
        let to_check = 20 - piece.width + 1;

        // There are up to 20^2 = 400 possible moves
        let mut moves: Vec<(i8, i8)> = Vec::with_capacity(to_check as usize * to_check as usize);

        if piece.height <= 4 {
            // The height of the piece is <= 4.
            // we can check the rows in groups of 4.
            // We do as follows:
            // Generate a checker for the first row in the gap
            // Rotate down by 1 and check again
            // repeat
            let check0to4 = self.get_checker(player, 0);
            let check4to8 = self.get_checker(player, 4);
            let check8to12 = self.get_checker(player, 8);
            let check12to16 = self.get_checker(player, 12);

            let mut shape = piece.occupied_mask;

            for offset in 0..4 {
                let mut y_shape = shape;
                for x in 0..(21 - piece.width) {
                    // 21 here because we need to check the last row
                    if Self::check(check0to4, y_shape) {
                        moves.push((x, offset));
                    }
                    if Self::check(check4to8, y_shape) {
                        moves.push((x, offset + 4));
                    }
                    if Self::check(check8to12, y_shape) {
                        moves.push((x, offset + 8));
                    }
                    if Self::check(check12to16, y_shape) {
                        moves.push((x, offset + 12));
                    }
                    y_shape = unsafe { shift_left_1(y_shape) };
                }
                shape = unsafe { rotate_down_1(shape) };
            }

            // this last one is special. The number of rows to check is dependent on the height of the piece
            // by here, the shape has shifted down 4 tiles already.

            for offset in 4..(9 - piece.height) {
                let mut y_shape = shape;
                for x in 0..(21 - piece.width) {
                    if Self::check(check12to16, y_shape) {
                        moves.push((x, offset + 12));
                    }
                    y_shape = unsafe { shift_left_1(y_shape) };
                }
                shape = unsafe { rotate_down_1(shape) };
            }
        } else {
            // This is the one case of the 5 high piece
            debug_assert!(piece.height == 5 && piece.width == 1);
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

    pub fn place_piece(&mut self, player: &Player, mv: &Move) {
        let piece = &PIECES[mv.piece];
        let (x, y) = mv.pos;
        // println!("Placing {} at {:?}", mv.piece, mv.pos);

        let pid = usize::from(player);

        // mm128 version of x position
        // let x_xmm = x as u128;
        // let x_xmm= unsafe { _mm_loadu_si128(std::ptr::addr_of!(x_xmm) as *const __m128i) };

        // Update the occupied mask
        // if y + 8 <= 20 {
        //     // we can safely update the mask here as y + 8 never goes out of bounds
        //     let mask = self.occupied_mask[y as usize..y as usize + 8]
        //         .try_into()
        //         .unwrap();
        //     let mask = unsafe { ymm(mask) };
        //     let new_mask = unsafe { _mm256_or_si256(mask, _mm256_sll_epi32(piece.occupied_mask, x_xmm)) };
        //     self.occupied_mask[y as usize..y as usize + 8].copy_from_slice(
        //         &unsafe { std::mem::transmute::<_, [u32; 8]>(new_mask) },
        //     );
        // } else {
        //     // here, we instead shift down the piece mask by the number of rows
        //     // Uses the fact that the 8th row is always empty.
        //     // If we are shifting by n, the permutation looks like this:
        //     // (7-n), (7-n-1), ..., 0, 7, 7, ..., 7

        //     // this permutation array is reversed for easy indexing
        //     let permutation: [i32; 8] = array::from_fn(|i|
        //         if i < y as usize {
        //             7
        //         } else {
        //             i as i32 - y as i32
        //         }
        //     );
        //     let shape = unsafe {
        //         _mm256_permutevar8x32_epi32(piece.occupied_mask, _mm256_set_epi32(
        //             permutation[7], permutation[6], permutation[5], permutation[4],
        //             permutation[3], permutation[2], permutation[1], permutation[0]
        //         ))
        //     };

        //     // Now we can hook up the masks just as we wanted
        //     let mask = self.occupied_mask[12..20]
        //     .try_into()
        //     .unwrap();
        //     let mask = unsafe { ymm(mask) };
        //     let new_mask = unsafe { _mm256_or_si256(mask, _mm256_sll_epi32(shape, x_xmm)) };
        //     self.occupied_mask[12..20].copy_from_slice(
        //         &unsafe { std::mem::transmute::<_, [u32; 8]>(new_mask) },
        //     );
        // }

        let occupied_mask =
            unsafe { std::mem::transmute::<__m256i, [u32; 8]>(piece.occupied_mask) };
        for i in 0..piece.height {
            self.occupied_mask[(y + i) as usize] |= occupied_mask[i as usize] << x;
        }

        // Update the color and corner masks
        // safety: we don't care about 1s in the overflowed borders because
        // if a neighbor is filled in the boarder, its neighbor inside the board will already be filled as well
        // if a corner is in the boarder, it will never be seen as we only check valid moves in the board
        if x == 0 {
            if y == 0 {
                for i in 0..piece.height as usize + 1 {
                    self.color_masks[pid][i] |= piece.neighbor_mask[i + 1] >> 1;
                    self.corner_masks[pid][i] |= piece.corner_mask[i + 1] >> 1;
                }
            } else {
                for i in 0..min(21 - y as usize, piece.height as usize + 2) {
                    self.color_masks[pid][y as usize + i - 1] |= piece.neighbor_mask[i] >> 1;
                    self.corner_masks[pid][y as usize + i - 1] |= piece.corner_mask[i] >> 1;
                }
            }
        } else if y == 0 {
            for i in 0..piece.height as usize + 1 {
                self.color_masks[pid][i] |= piece.neighbor_mask[i + 1] << (x - 1);
                self.corner_masks[pid][i] |= piece.corner_mask[i + 1] << (x - 1);
            }
        } else {
            for i in 0..min(21 - y as usize, piece.height as usize + 2) {
                self.color_masks[pid][y as usize + i - 1] |= piece.neighbor_mask[i] << (x - 1);
                self.corner_masks[pid][y as usize + i - 1] |= piece.corner_mask[i] << (x - 1);
            }
        }

        self.player_pieces[pid] &= !PIECES[mv.piece].id_mask;
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..20 {
            for col in 0..20 {
                let cell = (self.occupied_mask[row] >> col & 1) != 0;
                write!(f, "{}", if cell { 'X' } else { '-' })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Occupied Mask:")?;
        for x in self.occupied_mask.iter() {
            writeln!(f, "{:020b}", x)?;
        }
        for player in Player::iter() {
            let pid = usize::from(player);
            let color = player.color();
            writeln!(
                f,
                "Color Mask for player {}:",
                color.paint(format!("{}", pid))
            )?;
            self.color_masks[usize::from(player)].iter().for_each(|x| {
                writeln!(
                    f,
                    "{}",
                    color.paint(format!("{:020b}", x & ((1 << 20) - 1)))
                )
                .unwrap();
            });
            writeln!(
                f,
                "Corner Mask for player {}:",
                color.paint(format!("{}", pid))
            )?;
            self.corner_masks[usize::from(player)].iter().for_each(|x| {
                writeln!(
                    f,
                    "{}",
                    color.paint(format!("{:020b}", x & ((1 << 20) - 1)))
                )
                .unwrap();
            });
        }
        Ok(())
    }
}
