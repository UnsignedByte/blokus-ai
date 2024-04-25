use super::Algorithm;
use crate::game::{Player, State};
use itertools::Itertools;
use std::iter::repeat;

use rayon::prelude::*;

/// Player in a tournament
#[derive(Clone)]
struct Agent {
    algorithm: Box<dyn Algorithm>,
    elo: f64,
}

/// Hosts a tournament with elo ratings
#[derive(Clone)]
pub struct Tournament {
    /// AI Agents that will be playing in this tournament
    agents: Vec<Agent>,
}


// TODO: Better way of doing this to placate Rayon?
unsafe impl Send for Tournament {}
unsafe impl Sync for Tournament {}


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
        let mut games = repeat(0..self.agents.len())
            .take(Player::N)
            .multi_cartesian_product()
            .map(|a| (a.try_into().unwrap(), self.clone()))
            .collect::<Vec<([usize; 4], Tournament)>>();

        let games = games.par_iter_mut()
            // skip the game if all the players are the same, as elo will never change
            .filter(|(agents, _)| !agents.iter().all_equal())
            .map(|(agents, tournament)| {
                (agents.clone(), tournament.simulate_game(agents.clone().try_into().unwrap()))
            }).collect::<Vec<_>>();

        for (agents, final_state) in games {
            self.update_elos(agents.try_into().unwrap(), &final_state);
        }
    }

    /// Run a single game with 4 agents and return the final state
    pub fn simulate_game(&mut self, agents: [usize; Player::N]) -> State {
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
                #[cfg(debug_assertions)]
                println!("Player {} played:\n{:?}", usize::from(player), game);
            }
        }

        game

        // println!(
        //     "Game {:?} had scores {:?}",
        //     agents
        //         .iter()
        //         .map(|v| self.agents[*v].algorithm.name())
        //         .collect::<Vec<_>>(),
        //     scores
        // );

    }

    fn update_elos(&mut self, agents: [usize; Player::N], state: &State) {
        let scores = state.scores();
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
