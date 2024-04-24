use super::Algorithm;
use crate::game::{Player, State};
use itertools::Itertools;
use std::iter::repeat;

/// Player in a tournament
struct Agent {
    algorithm: Box<dyn Algorithm>,
    elo: f64,
}

/// Hosts a tournament with elo ratings
pub struct Tournament {
    /// AI Agents that will be playing in this tournament
    agents: Vec<Agent>,
}

impl Tournament {
    pub fn new(algorithms: Vec<Box<dyn Algorithm>>) -> Self {
        Self {
            agents: algorithms
                .into_iter()
                .map(|algorithm| Agent { algorithm, elo: 0. })
                .collect(),
        }
    }

    /// Get the elo stats for each player
    pub fn scores(&self) -> Vec<(String, f64)> {
        self.agents
            .iter()
            .map(|agent| (agent.algorithm.name(), agent.elo))
            .collect()
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
        // skip the game if all the players are the same, as elo will never change
        if agents.iter().all_equal() {
            return;
        }

        let mut game = State::new(20, 20);
        let mut alive = true;
        // run as long as a player is still playing
        while alive {
            alive = false;
            for player in Player::iter() {
                let agent = &mut self.agents[agents[usize::from(player)]];
                if let Some(mv) = agent.algorithm.decide(&game, &player) {
                    game.place_piece(&player, &mv);
                    alive = true;
                }
            }
        }

        let scores = game.scores();
        // println!(
        //     "Game {:?} had scores {:?}",
        //     agents
        //         .iter()
        //         .map(|v| self.agents[*v].algorithm.name())
        //         .collect::<Vec<_>>(),
        //     scores
        // );

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
