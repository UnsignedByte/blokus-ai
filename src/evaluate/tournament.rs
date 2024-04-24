use super::Algorithm;
use crate::game::{Player, State};
use itertools::Itertools;
use std::iter::repeat;

/// Player in a tournament
struct Agent<'t> {
    algorithm: &'t mut dyn Algorithm,
    elo: f64,
}

/// Hosts a tournament with elo ratings
pub struct Tournament<'t> {
    /// AI Agents that will be playing in this tournament
    agents: Vec<Agent<'t>>,
}

impl<'t> Tournament<'t> {
    pub fn new(algorithms: Vec<&'t mut dyn Algorithm>) -> Self {
        Self {
            agents: algorithms
                .into_iter()
                .map(|algorithm| Agent { algorithm, elo: 0. })
                .collect(),
        }
    }

    /// Simulate one round robin round
    pub fn round_robin(&mut self) {
        for game in repeat(0..self.agents.len())
            .take(Player::N)
            .multi_cartesian_product()
        {
            debug_assert!(game.len() == Player::N);

            self.simulate_game(game.try_into().unwrap())
        }
    }

    /// Run a single game with 4 agents
    pub fn simulate_game(&mut self, agents: [usize; Player::N]) {
        let mut game = State::new(20, 20);
        let mut alive = true;
        // run as long as a player is still playing
        while alive {
            alive = false;
            for player in Player::iter() {
                let agent = &mut self.agents[agents[usize::from(player)]];
                if let Some(mv) = agent.algorithm.decide(&game, &Player::Player1) {
                    game.place_piece(&Player::Player1, &mv);
                    alive = true;
                }
            }
        }

        let scores = game.scores();

        let mut elo_diffs = [0.; Player::N];

        // calculate pairwise ELO
        for player in 0..Player::N {
            let agent = &self.agents[agents[player]];

            // We divide the K value by the number of opponents as each player has "played" 3 games
            const K: f64 = 32. / (Player::N - 1) as f64;

            for o_player in 0..Player::N {
                if player == o_player {
                    continue;
                }
                let o_agent = &self.agents[agents[o_player]];

                let s = match scores[player].cmp(&scores[o_player]) {
                    std::cmp::Ordering::Less => 0., // lost
                    std::cmp::Ordering::Equal => 0.5,
                    std::cmp::Ordering::Greater => 1.0, // we won
                };

                // Algorithm from https://en.wikipedia.org/wiki/Elo_rating_system
                let ea = 1. / (1.0 + f64::powf(10., (o_agent.elo - agent.elo) / 400.));

                elo_diffs[player] += K * (s - ea);
            }
        }

        // update all elos
        for player in 0..Player::N {
            self.agents[agents[player]].elo += elo_diffs[player];
        }
    }
}
