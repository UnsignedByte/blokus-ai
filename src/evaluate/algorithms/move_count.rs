use crate::game::Player;

use super::Algorithm;

/// Algorithms that only look at the number of
/// possible moves after a given move
pub enum MoveCount {
    /// Maximize moves for myself
    MaximizeSelf,
    /// Minimize moves for others
    MinimizeOthers,
}

impl Algorithm for MoveCount {
    fn decide(
        &mut self,
        state: &crate::game::State,
        player: &crate::game::Player,
    ) -> Option<crate::game::Move> {
        let moves = state.get_moves(player);

        match self {
            MoveCount::MaximizeSelf => moves.into_iter().max_by_key(|mv| {
                let mut tstate = state.clone();
                tstate.place_piece(player, mv);
                tstate.get_moves(player).len()
            }),
            MoveCount::MinimizeOthers => moves.into_iter().min_by_key(|mv| {
                let mut tstate = state.clone();
                tstate.place_piece(player, mv);
                Player::iter()
                    .map(|p| {
                        if p == *player {
                            0
                        } else {
                            tstate.get_moves(&p).len()
                        }
                    })
                    .sum::<usize>()
            }),
        }
    }

    fn name(&self) -> String {
        match self {
            MoveCount::MaximizeSelf => "Maximize My Moves",
            MoveCount::MinimizeOthers => "Minimize Other's moves",
        }
        .to_owned()
    }
}
