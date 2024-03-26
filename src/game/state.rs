use super::Mask;

/// The game state.
pub struct State {
    board: Mask,
}

impl State {
    pub fn new((w, h): (usize, usize)) -> Self {
        Self {
            board: Mask::new(w, vec![0; h]),
        }
    }

    /// Generate a bitmask from a 2D array of bytes.
    /// Each byte represents a cell on the board.
    fn gen_bitmask(&self, mask: Vec<Vec<u8>>) {}
}
