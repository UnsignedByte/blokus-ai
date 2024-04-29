use super::Heuristic;
use crate::game::piece_size;

#[derive(Default)]
pub struct Score;
unsafe impl Sync for Score {}

impl Heuristic for Score {
    type Key = u8;

    fn evaluate(&self, state: &crate::game::State, player: &crate::game::Player) -> Self::Key {
        state.scores()[usize::from(player)]
    }

    fn evaluate_move(
        &self,
        state: &crate::game::State,
        player: &crate::game::Player,
        mv: &crate::game::Move,
    ) -> Self::Key {
        // Place the piece that gives me the highest score
        state.scores()[usize::from(player)]
            + if mv.player == *player {
                piece_size(mv) // I moved, so my score went up
            } else {
                0 // I didn't move
            }
    }

    fn name(&self) -> String {
        "Score".to_owned()
    }
}
