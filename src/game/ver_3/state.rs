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

#[derive(Copy, Clone, PartialEq, Eq)]
enum SubsquareMaskTyp {
    OccupiedOrColor,
    Validcorners,
}

fn subsquare_str(mask: &[u16; 400]) -> String {
    let mut f = String::new();
    use std::fmt::Write;
    for y in 0..20 {
        for row in 0..4 {
            for x in (0..20).rev() {
                // get the row of this mask
                let row = (mask[y * 20 + x] >> (row * 4)) & 0xf;
                write!(f, "{:04b} ", row).unwrap();
            }
            writeln!(f).unwrap();
        }
        writeln!(f).unwrap();
    }
    f
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

    /// Set a single (x, y) bit in the board, updating all the subsquares containing it
    fn set_bit(&mut self, mask_typ: SubsquareMaskTyp, x: usize, y: usize, v: bool) {
        let mut_mask = match mask_typ {
            SubsquareMaskTyp::OccupiedOrColor => &mut self.occupied_or_color,
            SubsquareMaskTyp::Validcorners => &mut self.valid_corners,
        };

        // All masks starting in this area are affected:
        // XXXX
        // XXXX
        // XXXX
        // *XXX
        for start_y in (max(3, y) - 3)..y + 1 {
            let rel_y = y - start_y;
            for start_x in (max(3, x) - 3)..x + 1 {
                let rel_x = x - start_x;
                let idx = start_y * 20 + start_x;

                let bit = 1 << (rel_y * 4 + rel_x);

                if v {
                    mut_mask[idx] |= bit;
                } else {
                    mut_mask[idx] &= !bit;
                }
            }
        }
    }

    /// Make sure all 16 subsquares containing this bit agree with each other
    fn check_bit(&self, mask_typ: SubsquareMaskTyp, x: usize, y: usize) -> Result<(), String> {
        let mask = match mask_typ {
            SubsquareMaskTyp::OccupiedOrColor => &self.occupied_or_color,
            SubsquareMaskTyp::Validcorners => &self.valid_corners,
        };
        let mut expected = None;
        // All masks starting in this area contain the bit:
        // XXXX
        // XXXX
        // XXXX
        // *XXX
        for start_y in (max(3, y) - 3)..min(20, y + 1) {
            let rel_y = y - start_y;
            for start_x in (max(3, x) - 3)..min(20, x + 1) {
                let rel_x = x - start_x;
                let idx = start_y * 20 + start_x;
                let mask_v = (mask[idx] >> (rel_y * 4 + rel_x)) & 1;

                match expected {
                    Some(expected) => {
                        if mask_v != expected {
                            return Err(format!(
                                "Expected {} at ({}, {}) in {} mask mismatch at ({}, {}) mask",
                                expected,
                                x,
                                y,
                                match mask_typ {
                                    SubsquareMaskTyp::OccupiedOrColor => "Occupied/Color",
                                    SubsquareMaskTyp::Validcorners => "Corner",
                                },
                                start_x,
                                start_y
                            ));
                        }
                    }
                    None => expected = Some(mask_v),
                }
            }
        }

        Ok(())
    }

    /// Make sure the entire subsquare mask is conformal
    fn check(&self) -> Result<(), String> {
        for y in 0..23 {
            for x in 0..23 {
                self.check_bit(SubsquareMaskTyp::OccupiedOrColor, x, y)
                    .map_err(|msg| {
                        format!(
                            "{}\nOccupied/Color Mask:\n{}",
                            msg,
                            subsquare_str(&self.occupied_or_color)
                        )
                    })?;
                self.check_bit(SubsquareMaskTyp::Validcorners, x, y)
                    .map_err(|msg| {
                        format!("{}\nMasks:\n{}", msg, subsquare_str(&self.valid_corners))
                    })?;
            }
        }
        Ok(())
    }
}

impl Default for Subsquares {
    fn default() -> Self {
        // Valid corners should start empty as nothing is valid
        // but occupied_or_color should have ones for every cell that is "out of bounds".
        let mut occupied_or_color = [0u16; 400];

        for y in 17..20 {
            let n_empty_rows = 20 - y;
            // every bit is full except for the empty rows
            let mask = (u16::MAX) ^ ((1 << (n_empty_rows * 4)) - 1);
            for x in 0..20 {
                occupied_or_color[y * 20 + x] = mask;
            }
        }
        for x in 17..20 {
            let n_empty_cols = 20 - x;
            let rep_mask = 0xf ^ ((1 << n_empty_cols) - 1);
            // Repeat the rep mask 4 times
            let mask = rep_mask | rep_mask << 4 | rep_mask << 8 | rep_mask << 12;
            for y in 0..20 {
                occupied_or_color[y * 20 + x] |= mask;
            }
        }

        Subsquares {
            occupied_or_color,
            valid_corners: [0u16; 400],
        }
    }
}

impl Debug for Subsquares {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Occupied/Color Mask:")?;
        for y in 0..20 {
            for row in 0..4 {
                // rev bc little endian
                for x in (0..20).rev() {
                    // get the row of this mask
                    let row = (self.occupied_or_color[y * 20 + x] >> (row * 4)) & 0xf;
                    write!(f, "{:04b} ", row)?;
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }

        writeln!(f, "Corner Mask:")?;
        for y in 0..20 {
            for row in 0..4 {
                for x in (0..20).rev() {
                    // get the row of this mask
                    let row = (self.valid_corners[y * 20 + x] >> (row * 4)) & 0xf;
                    write!(f, "{:04b} ", row)?;
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

/// The game state
pub struct State {
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
        // this implementation is tailored for 20x20
        assert!(w == 20 && h == 20);
        let mut subsquares: [Subsquares; Player::N] = array::from_fn(|_| Subsquares::default());

        subsquares[0].set_bit(SubsquareMaskTyp::Validcorners, 0, 0, true);
        subsquares[1].set_bit(SubsquareMaskTyp::Validcorners, 19, 0, true);
        subsquares[2].set_bit(SubsquareMaskTyp::Validcorners, 19, 19, true);
        subsquares[3].set_bit(SubsquareMaskTyp::Validcorners, 0, 19, true);

        let s = Self {
            subsquares,
            player_pieces: [(1 << PIECE_COUNT) - 1; Player::N], // Players start with all the pieces
        };

        // check if on debug
        #[cfg(debug_assertions)]
        s.check();

        s
    }

    /// Get the possible moves for a player
    pub fn get_moves<'a>(&'a self, player: &'a Player) -> Vec<Move> {
        let mut moves = Vec::with_capacity(1000);

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
                self.subsquares[usize::from(player)].test_piece(
                    &mut moves,
                    piece,
                    pieces[piece].as_u16,
                );
            }
        }
        moves
    }

    pub fn place_piece(&mut self, player: &Player, mv: &Move) {
        // a piece placed at a position
        let piece = &PIECES[mv.piece];

        // println!(
        //     "Placing at {:?}:\n{}\n{}",
        //     &mv.pos,
        //     piece
        //         .neighbor_mask
        //         .iter()
        //         .map(|v| format!("{:06b}", v))
        //         .fold(String::new(), |acc, v| format!("{}\n{}", acc, v)),
        //     piece
        //         .corner_mask
        //         .iter()
        //         .map(|v| format!("{:06b}", v))
        //         .fold(String::new(), |acc, v| format!("{}\n{}", acc, v))
        // );
        let (x, y) = mv.pos;

        let pid = usize::from(player);

        let (w, h) = (piece.width, piece.height);

        // Each piece has a 4x4 mask and a 6x6 neighbor mask
        // So therefore it influences up to a 12x12 square of 144 different masks
        // We need to update the occupied_or_color and corner masks for each of these subsquares
        // XXXXXXXXXXXX
        // XXXXXXXXXXXX
        // XXXXXXXXXXXX
        // XXX??????XXX
        // XXX?****?XXX
        // XXX?****?XXX
        // XXX?****?XXX
        // XXX?****?XXX
        // XXX??????XXX
        // XXXXXXXXXXXX
        // XXXXXXXXXXXX
        // XXXXXXXXXXXX
        // + h + 1 here because the neighbor map reaches to y + h + 1
        for offset_y in max(0, y - 4)..min(20, y + h + 1) {
            // same calculations for y
            // Range of rows of the piece mask that will be used (y)
            let piece_y_range = (
                max(0, offset_y - y + 1) as usize, // + 1 bc neighbor / corner masks start at -1
                (min(0, offset_y - y + 1) + 4) as usize,
            );

            if piece_y_range.1 <= piece_y_range.0 {
                continue;
            }

            let piece_y_w = piece_y_range.1 - piece_y_range.0;

            // where in this 4x4 mask does the piece mask begin
            let mask_y_start = max(0, y - 1 - offset_y) as usize;

            // + w + 1 here because the neighbor map reaches to x + w + 1
            for offset_x in max(0, x - 4)..min(20, x + w + 1) {
                // Range of columns of the piece mask that will be used (x)
                let piece_x_range = (
                    max(0, offset_x - x + 1) as usize, // + 1 bc neighbor / corner masks start at -1
                    (min(0, offset_x - x + 1) + 4) as usize,
                );

                if piece_x_range.1 <= piece_x_range.0 {
                    continue;
                }

                // Mask for number of cols used
                let piece_x_w = (1 << (piece_x_range.1 - piece_x_range.0)) - 1;

                // where in this 4x4 mask does the piece mask begin
                let mask_x_start = max(0, x - 1 - offset_x) as usize;

                // which 4x4 mask we are at
                let mask_idx = offset_y as usize * 20 + offset_x as usize;

                debug_assert!(mask_idx < 400);

                for row in 0..min(piece.height as usize + 2, piece_y_w) {
                    // where in the mask we are
                    let mask_y = mask_y_start + row;
                    // Shift right to discard lowest cols
                    let mut nmask = piece.neighbor_mask[row + piece_y_range.0] >> piece_x_range.0;
                    // Keep only (range high - range low) cols
                    nmask &= piece_x_w;
                    let nmask = nmask as u16;
                    // shift left by y * 4 + x as this is a 4x4 mask in a u16
                    self.subsquares[pid].occupied_or_color[mask_idx] |=
                        nmask << (mask_y * 4 + mask_x_start);

                    // now do the same for corners
                    let mut cmask = piece.corner_mask[row + piece_y_range.0] >> piece_x_range.0;
                    // Keep only (range high - range low) cols
                    cmask &= piece_x_w;
                    let cmask = cmask as u16;
                    self.subsquares[pid].valid_corners[mask_idx] |=
                        cmask << (mask_y * 4 + mask_x_start);
                }
            }
        }

        // Now, deal with the occupation masks of all the other players
        // Each piece has a 4x4 mask and a 4x4 occupation mask
        // So therefore it influences up to a 10x10 square of 100 different masks
        // We need to update the occupied_or_color
        // XXXXXXXXXX
        // XXXXXXXXXX
        // XXX****XXX
        // XXX****XXX
        // XXX****XXX
        // XXX****XXX
        // XXXXXXXXXX
        // XXXXXXXXXX
        // masks for x -
        let x_masks = [
            0b0001000100010001, // 1 only
            0b0011001100110011, // 2 only
            0b0111011101110111, // 3 only
            0b1111111111111111, // 4
        ];
        for opid in 0..4 {
            if opid == pid {
                continue;
            }
            // 0..2
            // y = 1
            for offset_y in max(0, y - 3)..min(20, y + h) {
                // same calculations for y
                // Range of rows of the piece mask that will be used (y)
                let piece_y_range = (
                    max(0, offset_y - y) as usize,
                    (min(0, offset_y - y) + 4) as usize,
                );

                if piece_y_range.1 <= piece_y_range.0 {
                    continue;
                }

                // where in this 4x4 mask does the piece mask begin
                let mask_y_start = max(0, y - offset_y);

                for offset_x in max(0, x - 3)..min(20, x + w) {
                    // Range of columns of the piece mask that will be used (x)
                    let piece_x_range = (
                        max(0, offset_x - x) as usize,
                        (min(0, offset_x - x) + 4) as usize,
                    );

                    if piece_x_range.1 <= piece_x_range.0 {
                        continue;
                    }

                    // where in this 4x4 mask does the piece mask begin
                    let mask_x_start = max(0, x - offset_x);

                    // which 4x4 mask we are at
                    let mask_idx = offset_y as usize * 20 + offset_x as usize;

                    // Mask for number of cols used
                    let piece_x_w = x_masks[piece_x_range.1 - piece_x_range.0 - 1];
                    // mask for the number of rows used
                    let piece_y_w = ((1 << ((piece_y_range.1 - piece_y_range.0) * 4)) - 1) as u16;

                    let mut mask = piece.as_u16;
                    mask >>= piece_y_range.0 * 4 + piece_x_range.0;
                    // cancel out the extra cols
                    mask &= piece_x_w;
                    // cancel out the extra rows
                    mask &= piece_y_w;

                    // shift the mask so its aligned with the current 4x4 mask
                    mask <<= mask_y_start * 4 + mask_x_start;

                    self.subsquares[opid].occupied_or_color[mask_idx] |= mask;
                }
            }
        }

        // check if on debug
        #[cfg(debug_assertions)]
        self.check()
    }

    // Check to make sure the game masks are conformal
    fn check(&self) {
        for player in Player::iter() {
            self.subsquares[usize::from(player)]
                .check()
                .unwrap_or_else(|msg| {
                    println!("{}", msg);
                    panic!()
                });
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..20 {
            for y in 0..20 {
                let cell = (self.subsquares[0].occupied_or_color[20 * y + x] & 1) != 0;
                write!(f, "{}", if cell { 'X' } else { '-' })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // for player in Player::iter() {
        //     let pid = usize::from(player);
        //     let color = player.color();
        //     writeln!(
        //         f,
        //         "Occupied/Color Mask for player {}:",
        //         color.paint(format!("{}", pid))
        //     )?;
        //     self.subsquares[usize::from(player)].iter().for_each(|x| {
        //         writeln!(
        //             f,
        //             "{}",
        //             color.paint(format!("{:020b}", x & ((1 << 20) - 1)))
        //         )
        //         .unwrap();
        //     });
        //     writeln!(
        //         f,
        //         "Corner Mask for player {}:",
        //         color.paint(format!("{}", pid))
        //     )?;
        //     self.corner_masks[usize::from(player)].iter().for_each(|x| {
        //         writeln!(
        //             f,
        //             "{}",
        //             color.paint(format!("{:020b}", x & ((1 << 20) - 1)))
        //         )
        //         .unwrap();
        //     });
        // }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_place_corner_one() {
        // Take the
        // X
        // piece
        let piece = 0;

        // Place it in the top right at (0, 0)
        let mut game = State::new(20, 20);
        game.check();
        game.place_piece(&Player::Player1, &Move::new(piece, (0, 0)));
        // Make sure all the masks are valid
        game.check();

        // The masks at the top right should look like
        // (0, 0):
        // 0011
        // 0001
        // 0000
        // 0000
        // (1, 0):
        // 0001
        // 0000
        // 0000
        // 0000
        // (0, 1):
        // 0001
        // 0000
        // 0000
        // 0000
        assert!(game.subsquares[0].occupied_or_color[0] == 0b0000000000010011);
        assert!(game.subsquares[0].occupied_or_color[1] == 1);
        assert!(game.subsquares[0].occupied_or_color[20] == 1);
        assert!(game.subsquares[1].occupied_or_color[0] == 1);
        assert!(game.subsquares[2].occupied_or_color[0] == 1);
        assert!(game.subsquares[3].occupied_or_color[0] == 1);
        // Corner mask should look like
        // (0, 0):
        // 0001
        // 0010
        // 0000
        // 0000
        println!("{}", subsquare_str(&game.subsquares[0].valid_corners));
        assert!(game.subsquares[0].valid_corners[0] == 0b00100001);
        assert!(game.subsquares[0].valid_corners[1] == 0b00010000);
        assert!(game.subsquares[0].valid_corners[20] == 0b0010);
        assert!(game.subsquares[0].valid_corners[21] == 1);
    }

    #[test]
    fn check_place_off_corner_one() {
        // Take the
        // X
        // piece
        let piece = 0;

        // Place it in the top right at (1, 1)
        let mut game = State::new(20, 20);
        game.check();
        game.place_piece(&Player::Player1, &Move::new(piece, (1, 1)));
        // Make sure all the masks are valid
        game.check();

        // The masks at the top right should look like
        // (0, 0):
        // 0010
        // 0111
        // 0010
        // 0000
        assert!(game.subsquares[0].occupied_or_color[0] == 0b001001110010);
        assert!(game.subsquares[0].occupied_or_color[1] == 0b000100110001);
        assert!(game.subsquares[0].occupied_or_color[2] == 0b00010000);
        assert!(game.subsquares[0].occupied_or_color[20] == 0b00100111);
        assert!(game.subsquares[0].occupied_or_color[21] == 0b00010011);
        assert!(game.subsquares[0].occupied_or_color[22] == 1);
        assert!(game.subsquares[0].occupied_or_color[40] == 0b0010);
        assert!(game.subsquares[0].occupied_or_color[41] == 1);
        assert!(game.subsquares[1].occupied_or_color[0] == 0b00100000);
        assert!(game.subsquares[2].occupied_or_color[0] == 0b00100000);
        assert!(game.subsquares[3].occupied_or_color[0] == 0b00100000);
        // Corner mask should look like
        // (0, 0):
        // 0101
        // 0000
        // 0101
        // 0000
        println!("{}", subsquare_str(&game.subsquares[0].valid_corners));
        assert!(game.subsquares[0].valid_corners[0] == 0b010100000101);
        assert!(game.subsquares[0].valid_corners[1] == 0b001000000010);
        assert!(game.subsquares[0].valid_corners[2] == 0b000100000001);
        assert!(game.subsquares[0].valid_corners[20] == 0b01010000);
        assert!(game.subsquares[0].valid_corners[21] == 0b00100000);
        assert!(game.subsquares[0].valid_corners[22] == 0b00010000);
        assert!(game.subsquares[0].valid_corners[40] == 0b0101);
        assert!(game.subsquares[0].valid_corners[41] == 0b0010);
        assert!(game.subsquares[0].valid_corners[42] == 1);
    }
    #[test]
    fn check_place_bl_one() {
        // Take the
        // X
        // piece
        let piece = 0;

        // Place it in the bottom left at (19, 19)
        let mut game = State::new(20, 20);
        game.check();
        game.place_piece(&Player::Player1, &Move::new(piece, (19, 19)));
        // Make sure all the masks are valid
        game.check();

        // The masks at the bottom left should look like
        // (0, 0):
        // 1110000
        // 1111000
        // 1111100
        // 1111111
        // 1111111
        // 1111111
        println!("{}", subsquare_str(&game.subsquares[0].occupied_or_color));
        assert!(game.subsquares[0].occupied_or_color[399] == u16::MAX);
        assert!(game.subsquares[0].occupied_or_color[398] == u16::MAX);
        assert!(game.subsquares[0].occupied_or_color[397] == u16::MAX - 1);
        assert!(game.subsquares[0].occupied_or_color[396] == u16::MAX - 3);
        assert!(game.subsquares[0].occupied_or_color[395] == u16::MAX - 7);
        assert!(game.subsquares[0].occupied_or_color[379] == u16::MAX);
        assert!(game.subsquares[0].occupied_or_color[378] == u16::MAX - 1);
        assert!(game.subsquares[0].occupied_or_color[377] == 0b1111111111101100);
        assert!(game.subsquares[0].occupied_or_color[376] == 0b1111111111001000);
        assert!(game.subsquares[0].occupied_or_color[375] == 0b1111111110000000);
        assert!(game.subsquares[0].occupied_or_color[359] == u16::MAX - 1);
        assert!(game.subsquares[0].occupied_or_color[358] == 0b1111111111101100);
        assert!(game.subsquares[0].occupied_or_color[357] == 0b1111111011001000);
        // The masks at the bottom left should look like
        // (0, 0):
        // 0000000
        // 0010100
        // 0000000
        // 0010100
        // 0000000
        // 0000000
        println!("{}", subsquare_str(&game.subsquares[0].valid_corners));
        assert!(game.subsquares[0].valid_corners[399] == 0b00100000);
        assert!(game.subsquares[0].valid_corners[398] == 0b01010000);
        assert!(game.subsquares[0].valid_corners[397] == 0b10100000);
        assert!(game.subsquares[0].valid_corners[396] == 0b01000000);
        assert!(game.subsquares[0].valid_corners[395] == 0b10000000);
        assert!(game.subsquares[0].valid_corners[379] == 0b001000000010);
        assert!(game.subsquares[0].valid_corners[378] == 0b010100000101);
        assert!(game.subsquares[0].valid_corners[377] == 0b101000001010);
        assert!(game.subsquares[0].valid_corners[376] == 0b010000000100);
        assert!(game.subsquares[0].valid_corners[375] == 0b100000001000);
        assert!(game.subsquares[0].valid_corners[359] == 0b0010000000100000);
        assert!(game.subsquares[0].valid_corners[358] == 0b0101000001010000);
        assert!(game.subsquares[0].valid_corners[357] == 0b1010000010100000);
        assert!(game.subsquares[0].valid_corners[356] == 0b0100000001000000);
        assert!(game.subsquares[0].valid_corners[339] == 0b001000000000);
    }

    #[test]
    #[allow(non_snake_case)]
    fn check_place_corner_L() {
        // Take the
        // XX
        //  X
        // piece
        let piece = 5;

        // Place it in the top right at (0, 0)
        let mut game = State::new(20, 20);
        game.check();
        game.place_piece(&Player::Player1, &Move::new(piece, (0, 0)));
        // Make sure all the masks are valid
        game.check();
        // The masks at the top right should look like
        // (0, 0):
        // 0111
        // 0011
        // 0001
        // 0000
        println!(
            "{}",
            PIECES[piece]
                .neighbor_mask
                .iter()
                .map(|v| format!("{:06b}", v))
                .fold(String::new(), |acc, v| format!("{}\n{}", acc, v))
        );
        println!("{}", subsquare_str(&game.subsquares[0].occupied_or_color));
        assert!(game.subsquares[0].occupied_or_color[0] == 0b000100110111);
        assert!(game.subsquares[0].occupied_or_color[1] == 0b00010011);
        assert!(game.subsquares[0].occupied_or_color[20] == 0b00010011);
        assert!(game.subsquares[0].occupied_or_color[21] == 1);
        assert!(game.subsquares[1].occupied_or_color[0] == 0b00010011);
        assert!(game.subsquares[2].occupied_or_color[0] == 0b00010011);
        assert!(game.subsquares[3].occupied_or_color[0] == 0b00010011);
        // Corner mask should look like
        // (0, 0):
        // 0001
        // 0100
        // 0010
        // 0000
        println!("{}", subsquare_str(&game.subsquares[0].valid_corners));
        assert!(game.subsquares[0].valid_corners[0] == 0b001001000001);
        assert!(game.subsquares[0].valid_corners[1] == 0b000100100000);
        assert!(game.subsquares[0].valid_corners[2] == 0b00010000);
        assert!(game.subsquares[0].valid_corners[20] == 0b00100100);
        assert!(game.subsquares[0].valid_corners[21] == 0b00010010);
        assert!(game.subsquares[0].valid_corners[22] == 0b00000001);
        assert!(game.subsquares[0].valid_corners[40] == 0b0010);
        assert!(game.subsquares[0].valid_corners[41] == 0b0001);
    }
}
