use super::Algorithm;
use rand::seq::SliceRandom;

#[derive(Default)]
pub struct Random {
    rng: rand::rngs::ThreadRng,
}

impl Algorithm for Random {
    fn decide(
        &mut self,
        state: &crate::game::State,
        player: &crate::game::Player,
    ) -> Option<crate::game::Move> {
        state.get_moves(player).choose(&mut self.rng).cloned()
    }

    fn name(&self) -> String {
        "Random".to_owned()
    }
}
