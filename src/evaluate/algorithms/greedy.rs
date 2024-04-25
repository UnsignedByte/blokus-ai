use super::Heuristic;
use crate::game::piece_size;

#[derive(Default)]
pub struct Greedy;
unsafe impl Sync for Greedy {}

impl Heuristic for Greedy {
    type Key = u8;

    fn key(
        &self,
        _: &crate::game::State,
        _: &crate::game::Player,
        mv: &crate::game::Move,
    ) -> Self::Key {
        piece_size(mv)
    }

    fn name(&self) -> String {
        "Greedy".to_owned()
    }
}
