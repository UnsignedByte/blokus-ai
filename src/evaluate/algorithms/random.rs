use super::Algorithm;
use rand::seq::SliceRandom;

#[derive(Default)]
pub struct Random;

unsafe impl Sync for Random {}

impl Algorithm for Random {
    fn decide(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        state: &crate::game::State,
        player: &crate::game::Player,
    ) -> Option<crate::game::Move> {
        state.get_moves(player).choose(rng).cloned()
    }

    fn name(&self) -> String {
        "Random".to_owned()
    }
}
