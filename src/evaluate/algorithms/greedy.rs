use super::Algorithm;
use crate::game::piece_size;
use rand::seq::SliceRandom;

#[derive(Default, Clone)]
pub struct Greedy {
    rng: rand::rngs::ThreadRng,
}

impl Algorithm for Greedy {
    fn decide(
        &mut self,
        state: &crate::game::State,
        player: &crate::game::Player,
    ) -> Option<crate::game::Move> {
        // we shuffle here so that the max-size move is random
        let mut moves = state.get_moves(player);
        moves.shuffle(&mut self.rng);
        moves.into_iter().max_by_key(piece_size)
    }

    fn name(&self) -> String {
        "Greedy".to_owned()
    }
}
