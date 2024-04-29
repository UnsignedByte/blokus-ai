use super::{Algorithm, Heuristic};
use crate::game::{Move, Player, State};
use rand::seq::SliceRandom;

pub struct GreedyMax<H: Heuristic> {
    heuristic: H,
}

impl<H> Default for GreedyMax<H>
where
    H: Heuristic + Default,
{
    fn default() -> Self {
        Self {
            heuristic: Default::default(),
        }
    }
}

/// Default Algorithm implementation for a heuristic
impl<H> Algorithm for GreedyMax<H>
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
            .max_by_key(|mv| self.heuristic.evaluate_move(state, player, mv))
    }

    fn name(&self) -> String {
        format!("Max by {}", self.heuristic.name())
    }
}

pub struct GreedyMin<H: Heuristic> {
    heuristic: H,
}

impl<H> Default for GreedyMin<H>
where
    H: Heuristic + Default,
{
    fn default() -> Self {
        Self {
            heuristic: Default::default(),
        }
    }
}

/// Default Algorithm implementation for a heuristic
impl<H> Algorithm for GreedyMin<H>
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
            .min_by_key(|mv| self.heuristic.evaluate_move(state, player, mv))
    }

    fn name(&self) -> String {
        format!("Min by {}", self.heuristic.name())
    }
}
