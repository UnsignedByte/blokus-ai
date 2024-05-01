use super::Heuristic;
use crate::game::Player;

/// Algorithms that only look at the number of
/// possible moves after a given move
#[derive(Default)]
pub struct MoveCount;

unsafe impl Sync for MoveCount {}

impl Heuristic for MoveCount {
    fn name(&self) -> String {
        "Move Count".to_owned()
    }

    type Key = usize;

    fn evaluate(
        &self,
        _: &mut rand::rngs::ThreadRng,
        state: &crate::game::State,
        player: &Player,
    ) -> Self::Key {
        state.get_moves(player).len()
    }
}
