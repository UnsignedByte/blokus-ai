use super::Heuristic;
use crate::game::{Player, State};
use rand::seq::{IteratorRandom, SliceRandom};

/// Algorithm that rolls out random games and returns the number of wins
pub struct Rollout {
    /// Number of games to simulate
    pub simulations: usize,
}

impl Rollout {
    pub fn new(simulations: usize) -> Self {
        Self { simulations }
    }
}

/// Run a random rollout and return whether this player won or not
fn random_rollout(rng: &mut rand::rngs::ThreadRng, state: &State, player: Player) -> bool {
    let mut state = state.clone();
    let mut player = player;
    let mut dones = [false; Player::N];
    loop {
        if dones.iter().all(|u| *u) {
            // All players are done, the game is over
            break;
        }

        if dones[usize::from(player)] {
            player = player.next();
            continue;
        }
        let moves = state.get_moves(&player);
        if moves.is_empty() {
            dones[usize::from(player)] = true;
            player = player.next();
            continue;
        }

        let mv = moves.choose(rng).unwrap();
        state.place_piece(mv);

        player = player.next();
    }

    // Check if the player won
    let scores = state.scores();
    let max_score = scores.iter().max().unwrap();
    // Get all players with the max score and choose one randomly
    // (because ties are broken randomly)
    let (winner, _) = scores
        .iter()
        .enumerate()
        .filter(|(_, &s)| s == *max_score)
        .choose(rng)
        .unwrap();

    winner == usize::from(player)
}

unsafe impl Sync for Rollout {}

impl Heuristic for Rollout {
    fn name(&self) -> String {
        format!("{}-Game Rollout", self.simulations).to_owned()
    }

    type Key = usize;

    fn evaluate(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        state: &crate::game::State,
        player: &Player,
    ) -> Self::Key {
        let mut wins = 0;
        for _ in 0..self.simulations {
            wins += random_rollout(rng, state, *player) as usize
        }
        wins
    }
}
