use super::Heuristic;
use crate::game::Player;

/// The number of possible moves after a given move
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

/// The number of enemy possible moves after a given move
#[derive(Default)]
pub struct EnemyMoveCount;

unsafe impl Sync for EnemyMoveCount {}

impl Heuristic for EnemyMoveCount {
    fn name(&self) -> String {
        "Enemy Move Count".to_owned()
    }

    type Key = usize;

    fn evaluate(
        &self,
        _: &mut rand::rngs::ThreadRng,
        state: &crate::game::State,
        player: &Player,
    ) -> Self::Key {
        Player::iter()
            .filter(|p| p != player)
            .map(|p| state.get_moves(&p).len())
            .sum()
    }
}
