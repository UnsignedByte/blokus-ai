use dyn_clone::DynClone;
use crate::game::{Move, Player, State};

/// Abstraction of an algorithm for blokus
pub trait Algorithm : DynClone {
    /// Make a decision given a current state for a player
    fn decide(&mut self, state: &State, player: &Player) -> Option<Move>;

    /// String name for the algorithm
    fn name(&self) -> String;
}

dyn_clone::clone_trait_object!(Algorithm);
