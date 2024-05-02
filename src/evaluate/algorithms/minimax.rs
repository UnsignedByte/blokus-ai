use super::{Algorithm, Heuristic};
use crate::game::{Move, Player, State};
use rand::seq::SliceRandom;

/// The minimax algorithm
/// Depth is the number of moves to look ahead
#[derive(Clone)]
pub struct MiniMax<const DEPTH: usize, H: Heuristic> {
    heuristic: H,
}

impl<const DEPTH: usize, H: Heuristic> MiniMax<DEPTH, H> {
    pub fn new(heuristic: H) -> Self {
        Self { heuristic }
    }
}

impl<const DEPTH: usize, H> Default for MiniMax<DEPTH, H>
where
    H: Heuristic + Default,
{
    fn default() -> Self {
        Self {
            heuristic: Default::default(),
        }
    }
}

fn minimax<H: Heuristic>(
    rng: &mut rand::rngs::ThreadRng,
    depth: usize,
    // Player being evaluated
    evaluating_player: &Player,
    // Player that is playing right now
    player: &Player,
    state: &State,
    heuristic: &H,
) -> Option<H::Key> {
    let moves = state.get_moves(player);
    match depth {
        0 => None,
        1 => {
            // Don't do any more search, just maximize or minimize by the heuristic
            // we use evaluate_move as it may be faster
            let moves = moves
                .into_iter()
                .map(|mv| heuristic.evaluate_move(rng, state, evaluating_player, &mv));
            match player == evaluating_player {
                true => moves.max(),
                false => moves.min(),
            }
        }
        _ => {
            let moves = moves
                .into_iter()
                // If there are never any more moves then this would be empty
                .flat_map(move |mv| {
                    let mut nstate = state.clone();
                    nstate.place_piece(&mv);

                    minimax(
                        rng,
                        depth - 1,
                        evaluating_player,
                        &player.next(),
                        &nstate,
                        heuristic,
                    )
                });
            // Recurse
            match player == evaluating_player {
                true => moves.max(),
                false => moves.min(),
            }
        }
    }
}

impl<const DEPTH: usize, H: Heuristic> Algorithm for MiniMax<DEPTH, H> {
    fn name(&self) -> String {
        format!("MiniMax {} Depth {}", self.heuristic.name(), DEPTH)
    }

    fn decide(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        state: &State,
        player: &Player,
        _: usize,
    ) -> Option<Move> {
        // Find the move that maximizes the minimax algorithm
        let mut moves = state.get_moves(player);
        moves.shuffle(rng); // Shuffle so that ties are resolved randomly
        moves.into_iter().max_by_key(|mv| {
            let mut nstate = state.clone();
            nstate.place_piece(mv);
            minimax(
                rng,
                DEPTH - 1,
                player,
                &player.next(),
                &nstate,
                &self.heuristic,
            )
            .unwrap_or(
                // If there are no moves, then the game is over so the score is just the current score
                self.heuristic.evaluate(rng, &nstate, player),
            )
        })
    }
}
