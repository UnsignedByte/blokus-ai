use super::Algorithm;
use rand::seq::SliceRandom;

#[derive(Default)]
pub struct Random;

unsafe impl Sync for Random {}

impl Algorithm for Random {
    fn name(&self) -> String {
        "Random".to_owned()
    }

    fn decide(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        state: &crate::game::State,
        player: &crate::game::Player,
    ) -> Option<crate::game::Move> {
        let moves = state.get_moves(player);
        moves.choose(rng).cloned()
    }
}
