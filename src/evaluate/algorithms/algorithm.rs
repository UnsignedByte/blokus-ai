use crate::game::{Move, Player, State};

/// Abstraction of an algorithm for blokus
pub trait Algorithm {
    /// Make a decision given a current state for a player
    fn decide(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        state: &State,
        player: &Player,
        move_number: usize,
    ) -> Option<Move>;

    /// String name for the algorithm
    fn name(&self) -> String;
}
