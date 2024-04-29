use crate::game::{Move, Player, State};

/// A heuristic can be implemented by an algorithm.
/// This allows us to automatically generate algorithm implementations via their heuristic.
pub trait Heuristic {
    /// The type returned by the heuristic
    type Key: Ord;

    /// Evaluate a state using this heuristic for a given player
    fn evaluate(&self, state: &State, player: &Player) -> Self::Key;

    /// Evaluate how the heuristic would change after a move
    /// Sometimes this should be implemented as it can be faster than
    /// placing the piece and then evaluating
    fn evaluate_move(&self, state: &State, player: &Player, mv: &Move) -> Self::Key {
        // By default, just place the piece and then evaluate
        let mut state = state.clone();
        state.place_piece(mv);
        self.evaluate(&state, player)
    }

    /// String name for the heuristic
    fn name(&self) -> String;
}
