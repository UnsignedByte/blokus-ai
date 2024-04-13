use once_cell::sync::Lazy;

use super::{
    utils::{rotate_down_1, shift_left_1, shift_up_1, ymm},
    Piece,
};
use crate::game::{ver_3::utils::ymm_str, Player};
use std::{
    arch::x86_64::*,
    array,
    cmp::{max, min},
    fmt::{Debug, Display},
};

const PIECE_COUNT: usize = 89;

pub static PIECES: Lazy<[Piece; PIECE_COUNT]> = Lazy::new(|| {
    [
        // 1 tile pieces
        // X - no transformations
        Piece::new(1, 1, 0b1, [0b1, 0, 0, 0]),
        // 2 tile pieces
        // XX - 2 states
        Piece::new(2, 1, 0b11 << 1, [0b11, 0, 0, 0]),
        Piece::new(1, 2, 0b11 << 1, [0b1, 0b1, 0, 0]),
        // 3 tile pieces
        // XXX - 2 states
        Piece::new(3, 1, 0b11 << 3, [0b111, 0, 0, 0]),
        Piece::new(1, 3, 0b11 << 3, [0b1, 0b1, 0b1, 0]),
        // XX
        // X  - 4 states
        Piece::new(2, 2, 0b1111 << 5, [0b11, 0b1, 0, 0]),
        Piece::new(2, 2, 0b1111 << 5, [0b11, 0b10, 0, 0]),
        Piece::new(2, 2, 0b1111 << 5, [0b1, 0b11, 0, 0]),
        Piece::new(2, 2, 0b1111 << 5, [0b10, 0b11, 0, 0]),
        // 4 tile pieces
        // XXXX - 2 states
        Piece::new(4, 1, 0b11 << 9, [0b1111, 0, 0, 0]),
        Piece::new(1, 4, 0b11 << 9, [0b1, 0b1, 0b1, 0b1]),
        // XXX
        // X   - 8 states
        Piece::new(3, 2, 0b11111111 << 11, [0b111, 0b1, 0, 0]),
        Piece::new(3, 2, 0b11111111 << 11, [0b111, 0b100, 0, 0]),
        Piece::new(3, 2, 0b11111111 << 11, [0b1, 0b111, 0, 0]),
        Piece::new(3, 2, 0b11111111 << 11, [0b100, 0b111, 0, 0]),
        Piece::new(2, 3, 0b11111111 << 11, [0b11, 0b10, 0b10, 0]),
        Piece::new(2, 3, 0b11111111 << 11, [0b11, 0b1, 0b1, 0]),
        Piece::new(2, 3, 0b11111111 << 11, [0b1, 0b1, 0b11, 0]),
        Piece::new(2, 3, 0b11111111 << 11, [0b10, 0b10, 0b11, 0]),
        // XX
        //  XX - 4 states
        Piece::new(3, 2, 0b1111 << 19, [0b110, 0b011, 0, 0]),
        Piece::new(3, 2, 0b1111 << 19, [0b011, 0b110, 0, 0]),
        Piece::new(2, 3, 0b1111 << 19, [0b10, 0b11, 0b01, 0]),
        Piece::new(2, 3, 0b1111 << 19, [0b01, 0b11, 0b10, 0]),
        // XXX
        //  X - 4 states
        Piece::new(3, 2, 0b1111 << 23, [0b111, 0b010, 0, 0]),
        Piece::new(3, 2, 0b1111 << 23, [0b010, 0b111, 0, 0]),
        Piece::new(2, 3, 0b1111 << 23, [0b10, 0b11, 0b10, 0]),
        Piece::new(2, 3, 0b1111 << 23, [0b01, 0b11, 0b01, 0]),
        // XX
        // XX - 1 state
        Piece::new(2, 2, 0b1 << 27, [0b11, 0b11, 0, 0]),
        // 5 tile pieces
        //  XX
        // XX
        //  X - 8 states
        Piece::new(3, 3, 0b11111111 << 28, [0b110, 0b011, 0b010, 0]),
        Piece::new(3, 3, 0b11111111 << 28, [0b011, 0b110, 0b010, 0]),
        Piece::new(3, 3, 0b11111111 << 28, [0b010, 0b110, 0b011, 0]),
        Piece::new(3, 3, 0b11111111 << 28, [0b010, 0b011, 0b110, 0]),
        Piece::new(3, 3, 0b11111111 << 28, [0b010, 0b111, 0b001, 0]),
        Piece::new(3, 3, 0b11111111 << 28, [0b001, 0b111, 0b010, 0]),
        Piece::new(3, 3, 0b11111111 << 28, [0b010, 0b111, 0b100, 0]),
        Piece::new(3, 3, 0b11111111 << 28, [0b100, 0b111, 0b010, 0]),
        // XXXX
        // X    - 8 states
        Piece::new(4, 2, 0b11111111 << 36, [0b1111, 0b0001, 0, 0]),
        Piece::new(4, 2, 0b11111111 << 36, [0b1111, 0b1000, 0, 0]),
        Piece::new(4, 2, 0b11111111 << 36, [0b1000, 0b1111, 0, 0]),
        Piece::new(4, 2, 0b11111111 << 36, [0b0001, 0b1111, 0, 0]),
        Piece::new(2, 4, 0b11111111 << 36, [0b11, 0b10, 0b10, 0b10]),
        Piece::new(2, 4, 0b11111111 << 36, [0b11, 0b01, 0b01, 0b01]),
        Piece::new(2, 4, 0b11111111 << 36, [0b10, 0b10, 0b10, 0b11]),
        Piece::new(2, 4, 0b11111111 << 36, [0b01, 0b01, 0b01, 0b11]),
        // XXX
        //   XX - 8 states
        Piece::new(4, 2, 0b11111111 << 44, [0b1110, 0b0011, 0b0, 0]),
        Piece::new(4, 2, 0b11111111 << 44, [0b0111, 0b1100, 0b0, 0]),
        Piece::new(4, 2, 0b11111111 << 44, [0b0011, 0b1110, 0b0, 0]),
        Piece::new(4, 2, 0b11111111 << 44, [0b1100, 0b0111, 0b0, 0]),
        Piece::new(2, 4, 0b11111111 << 44, [0b10, 0b10, 0b11, 0b01]),
        Piece::new(2, 4, 0b11111111 << 44, [0b01, 0b01, 0b11, 0b10]),
        Piece::new(2, 4, 0b11111111 << 44, [0b10, 0b11, 0b01, 0b01]),
        Piece::new(2, 4, 0b11111111 << 44, [0b01, 0b11, 0b10, 0b10]),
        // XX
        // XXX - 8 states
        Piece::new(3, 2, 0b11111111 << 52, [0b110, 0b111, 0, 0]),
        Piece::new(3, 2, 0b11111111 << 52, [0b111, 0b110, 0, 0]),
        Piece::new(3, 2, 0b11111111 << 52, [0b011, 0b111, 0, 0]),
        Piece::new(3, 2, 0b11111111 << 52, [0b111, 0b011, 0, 0]),
        Piece::new(2, 3, 0b11111111 << 52, [0b11, 0b11, 0b01, 0]),
        Piece::new(2, 3, 0b11111111 << 52, [0b11, 0b11, 0b10, 0]),
        Piece::new(2, 3, 0b11111111 << 52, [0b01, 0b11, 0b11, 0]),
        Piece::new(2, 3, 0b11111111 << 52, [0b10, 0b11, 0b11, 0]),
        // XXX
        //  X
        //  X - 4 states
        Piece::new(3, 3, 0b1111 << 60, [0b111, 0b010, 0b010, 0]),
        Piece::new(3, 3, 0b1111 << 60, [0b010, 0b010, 0b111, 0]),
        Piece::new(3, 3, 0b1111 << 60, [0b001, 0b111, 0b001, 0]),
        Piece::new(3, 3, 0b1111 << 60, [0b100, 0b111, 0b100, 0]),
        // X X
        // XXX - 4 states
        Piece::new(3, 2, 0b1111 << 64, [0b101, 0b111, 0, 0]),
        Piece::new(3, 2, 0b1111 << 64, [0b111, 0b101, 0, 0]),
        Piece::new(2, 3, 0b1111 << 64, [0b11, 0b10, 0b11, 0]),
        Piece::new(2, 3, 0b1111 << 64, [0b11, 0b01, 0b11, 0]),
        // X
        // X
        // XXX - 4 states
        Piece::new(3, 3, 0b1111 << 68, [0b100, 0b100, 0b111, 0]),
        Piece::new(3, 3, 0b1111 << 68, [0b001, 0b001, 0b111, 0]),
        Piece::new(3, 3, 0b1111 << 68, [0b111, 0b100, 0b100, 0]),
        Piece::new(3, 3, 0b1111 << 68, [0b111, 0b001, 0b001, 0]),
        // X
        // XX
        //  XX - 4 states
        Piece::new(3, 3, 0b1111 << 72, [0b100, 0b110, 0b011, 0]),
        Piece::new(3, 3, 0b1111 << 72, [0b011, 0b110, 0b100, 0]),
        Piece::new(3, 3, 0b1111 << 72, [0b001, 0b011, 0b110, 0]),
        Piece::new(3, 3, 0b1111 << 72, [0b110, 0b011, 0b001, 0]),
        //  X
        // XXX
        //  X  - 1 state
        Piece::new(3, 3, 0b1 << 76, [0b010, 0b111, 0b010, 0]),
        //   X
        // XXXX - 8 states
        Piece::new(4, 2, 0b11111111 << 77, [0b1111, 0b0010, 0, 0]),
        Piece::new(4, 2, 0b11111111 << 77, [0b1111, 0b0100, 0, 0]),
        Piece::new(4, 2, 0b11111111 << 77, [0b0010, 0b1111, 0, 0]),
        Piece::new(4, 2, 0b11111111 << 77, [0b0100, 0b1111, 0, 0]),
        Piece::new(2, 4, 0b11111111 << 77, [0b01, 0b01, 0b11, 0b01]),
        Piece::new(2, 4, 0b11111111 << 77, [0b01, 0b11, 0b01, 0b01]),
        Piece::new(2, 4, 0b11111111 << 77, [0b10, 0b10, 0b11, 0b10]),
        Piece::new(2, 4, 0b11111111 << 77, [0b10, 0b11, 0b10, 0b10]),
        // XX
        //  X
        //  XX - 4 states
        Piece::new(3, 3, 0b1111 << 85, [0b110, 0b010, 0b011, 0]),
        Piece::new(3, 3, 0b1111 << 85, [0b011, 0b010, 0b110, 0]),
        Piece::new(3, 3, 0b1111 << 85, [0b001, 0b111, 0b100, 0]),
        Piece::new(3, 3, 0b1111 << 85, [0b100, 0b111, 0b001, 0]),
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

#[derive(Copy, Clone)]
pub struct Subsquares {
    /// Entry at index x contains a 4x4 mask, corresponding to a move at (x // 20, x % 20).
    occupied_or_color: [u16; 400],
    valid_corners: [u16; 400],
}

impl Subsquares {
    pub fn test_piece(&self, moves: &mut Vec<Move>, piece_id: usize, piece: u16) {
        unsafe {
            let piece = _mm256_set1_epi16(piece as i16);
            let zero = _mm256_setzero_si256();

            for i in 0..25 {
                let occupied_or_color = _mm256_loadu_si256(std::ptr::addr_of!(
                    self.occupied_or_color[i * 16]
                ) as *const __m256i);
                let valid_corners = _mm256_loadu_si256(std::ptr::addr_of!(
                    self.valid_corners[i * 16]
                ) as *const __m256i);
                let ok1 = _mm256_movemask_epi8(_mm256_cmpeq_epi16(
                    _mm256_and_si256(piece, occupied_or_color),
                    zero,
                )) as u32;
                let not_ok2 = _mm256_movemask_epi8(_mm256_cmpeq_epi16(
                    _mm256_and_si256(piece, valid_corners),
                    zero,
                )) as u32;

                let mut ok = (ok1 & !not_ok2) as u32; // pairs of 2 bits in here
                let mut move_index = i * 16;

                while ok != 0 {
                    let skip = ok.trailing_zeros();

                    move_index += (skip >> 1) as usize;
                    ok >>= skip;
                    ok >>= 2;

                    moves.push(Move::new(
                        piece_id,
                        ((move_index % 20) as i8, (move_index / 20) as i8),
                    ));
                    move_index += 1;
                }
            }
        }
    }
}

impl Default for Subsquares {
    fn default() -> Self {
        Subsquares {
            occupied_or_color: [0u16; 400],
            valid_corners: [0u16; 400],
        }
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
    subsquares: [Subsquares; Player::N],
    /// Playable pieces for every player
    /// stored as a [u128] where the lower [PIECE_COUNT] bits
    /// represent whether a player has the piece
    /// on hand or not
    player_pieces: [u128; Player::N],
}

type Checker = (__m256i, __m256i, __m256i);

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

    /// Generate a bunch of masks for each 4x4 sub-square
    pub fn gen_subsquares(&self) -> [Subsquares; Player::N] {
        fn get_bits(val: u32, x: u32) -> u16 {
            (((val | !((1u32 << 20) - 1)) >> x) & 0xf) as u16
        }

        fn fill(data: &[u32; 20], result: &mut [u16; 400]) {
            for y in 0..20usize {
                for x in 0..20u32 {
                    result[y * 20 + x as usize] |= get_bits(data[y], x)
                        | (get_bits(*data.get(y + 1).unwrap_or(&u32::MAX), x) << 4)
                        | (get_bits(*data.get(y + 2).unwrap_or(&u32::MAX), x) << 8)
                        | (get_bits(*data.get(y + 3).unwrap_or(&u32::MAX), x) << 12);
                }
            }
        }

        let mut result = [Subsquares::default(); Player::N];

        for i in 0..4 {
            fill(&self.occupied_mask, &mut result[i].occupied_or_color);
            fill(&self.color_masks[i], &mut result[i].occupied_or_color);
            fill(&self.corner_masks[i], &mut result[i].valid_corners);
        }

        result
    }

    /// Get a checker for rows [offset+2, offset+4].
    /// If we have a "piece" stored with the same format in an __m256i, then
    /// none(piece & occupied) & some(piece & corners) means that the piece fits in those 8 rows
    #[inline]
    fn get_checker(&self, player: &Player, offset: usize) -> Checker {
        unsafe {
            (
                // _mm256_or_si256(
                _mm256_loadu_si256(
                    self.occupied_mask.as_ptr().wrapping_add(offset) as *const __m256i
                ),
                _mm256_loadu_si256(
                    self.color_masks[usize::from(player)]
                        .as_ptr()
                        // This is moved back 1 because the neighbor mask is expanded
                        .wrapping_add(offset) as *const __m256i,
                ),
                // ),
                _mm256_loadu_si256(
                    self.corner_masks[usize::from(player)]
                        .as_ptr()
                        .wrapping_add(offset) as *const __m256i,
                ),
            )
        }
    }

    #[inline]
    fn check((occupied, colors, corners): Checker, shape: __m256i) -> bool {
        unsafe {
            // check if this is unoccupied
            // we want the result to be 0
            _mm256_testz_si256(colors, shape) != 0 &&
            // check if its a valid corner
            // testz returns 0 if the result of the & is >0
            // and 1 if the result is 0
            _mm256_testz_si256(corners, shape) == 0 &&
            // check if this is unoccupied
            // we want the result to be 0
            _mm256_testz_si256(occupied, shape) != 0
        }
    }

    /// Get the possible moves for a player
    pub fn get_moves<'a>(&'a self, player: &'a Player) -> Vec<Move> {
        let mut moves = Vec::with_capacity(1000);

        let subsquares: &[Subsquares; 4] = &self.gen_subsquares();

        let pieces = &*PIECES;
        // All the different piece transforms for the player
        for piece in (0..PIECE_COUNT)
            .filter(move |f| (1 << *f) & self.player_pieces[usize::from(player)] != 0)
        {
            // Only works for pieces that fit in a 4x4
            if pieces[piece].height <= 4 && pieces[piece].width <= 4 {
                /*
                                // CORRECTNESS TEST:
                                let mut a1 = vec![];
                                let mut a2 = vec![];

                                subsquares[usize::from(player)].test_piece(&mut a2, piece, pieces[piece].as_u16);
                                self.get_moves_for_piece(&mut a1, pieces, player, piece);

                                a1.sort();
                                a2.sort();
                */
                subsquares[usize::from(player)].test_piece(&mut moves, piece, pieces[piece].as_u16);
            }
        }
        moves
    }

    pub fn place_piece(&mut self, player: &Player, mv: &Move) {
        // a piece placed at a position
        let piece = &PIECES[mv.piece];
        let (x, y) = mv.pos;

        // Each piece has a 4x4 mask and a 6x6 neighbor mask
        // So therefore it influences up to a 9x9 square of 81 different masks
        // We need to update the occupied_or_color and corner masks for each of these subsquares
        for offset_x in max(0, x - 4)..min(20 - 3, x + 4) {
            for offset_y in max(0, x - 4)..min(20 - 3, x + 4) {
                let overlap_x = (
                    max(max(0, x - 1), offset_x) - x + 1,
                    min(min(x + 5, 20), offset_x + 6) - x + 1,
                );
                let overlap_y = (
                    (max(max(0, y - 1), offset_y) - y + 1) as usize,
                    (min(min(y + 5, 20), offset_y + 6) - y + 1) as usize,
                );

                let mut neighbors = [0u8; 6];
                // filter only the overlapping part
                neighbors[overlap_y.0..overlap_y.1]
                    .copy_from_slice(&piece.neighbor_mask[overlap_y.0..overlap_y.1]);

                for i in overlap_y.0..overlap_y.1 {
                    neighbors[i] >>= overlap_x.0;
                    neighbors[i] &= (1 << (overlap_x.1 - overlap_x.0)) - 1;
                }
            }
        }
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
