use super::Heuristic;

#[derive(Default)]
pub struct Random;

unsafe impl Sync for Random {}

impl Heuristic for Random {
    fn name(&self) -> String {
        "Random".to_owned()
    }

    type Key = ();

    fn key(
        &self,
        _: &crate::game::State,
        _: &crate::game::Player,
        _: &crate::game::Move,
    ) -> Self::Key {
        // Random heuristic, none are better than others.
    }
}
