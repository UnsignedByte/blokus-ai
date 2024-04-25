use crate::game::{Move, Player, State};
use rand::seq::SliceRandom;

/// Abstraction of an algorithm for blokus
pub trait Algorithm {
    /// Make a decision given a current state for a player
    fn decide(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        state: &State,
        player: &Player,
    ) -> Option<Move>;

    /// String name for the algorithm
    fn name(&self) -> String;
}

/// A heuristic can be implemented by an algorithm.
/// This allows us to automatically generate algorithm implementations via their heuristic.
pub trait Heuristic {
    /// The type returned by the heuristic
    type Key: Ord;

    /// The key for this heuristic.
    /// This will be maximized by the algorithm
    fn key(&self, state: &State, player: &Player, mv: &Move) -> Self::Key;

    /// String name for the heuristic
    fn name(&self) -> String;
}

/// Default Algorithm implementation for a heuristic
impl<H> Algorithm for H
where
    H: Heuristic,
{
    fn decide(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        state: &State,
        player: &Player,
    ) -> Option<Move> {
        // we shuffle here so that ties are resolved randomly
        let mut moves = state.get_moves(player);
        moves.shuffle(rng);
        moves
            .into_iter()
            .max_by_key(|mv| self.key(state, player, mv))
    }

    fn name(&self) -> String {
        Heuristic::name(self)
    }
}
