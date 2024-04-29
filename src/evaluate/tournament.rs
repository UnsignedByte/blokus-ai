use super::Algorithm;
use crate::game::{Player, State};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    array,
    fmt::Display,
    iter::repeat,
    time::{Duration, Instant},
};

/// Player in a tournament
/// Contains statistics about the player
struct Agent {
    algorithm: Box<dyn Algorithm + Sync>,
    elo: f64,
    games_played: usize,
    cumulative_points: usize,
    elapsed: Duration,
}

impl Agent {
    pub fn new(algorithm: Box<dyn Algorithm + Sync>) -> Self {
        Self {
            algorithm,
            elo: 0.,
            games_played: 0,
            cumulative_points: 0,
            elapsed: Duration::default(),
        }
    }
}

impl Display for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}:", self.algorithm.name(),)?;
        writeln!(
            f,
            "{: ^20}{: ^20}{: ^20}{: ^20}",
            "ELO", "Avg Pts", "Games Played", "Total Pts"
        )?;
        writeln!(
            f,
            "{: ^20.4}{: ^20.4}{: ^20.4}{: ^20.4}",
            self.elo,
            (self.cumulative_points as f32) / (self.games_played as f32),
            self.games_played,
            self.cumulative_points
        )?;
        Ok(())
    }
}

/// Struct representing a single player's performance in a game
/// Contains scores as well as extra info
pub struct GameStats {
    score: u8,
    elapsed: Duration,
}

/// Hosts a tournament with elo ratings
pub struct Tournament {
    /// AI Agents that will be playing in this tournament
    agents: Vec<Agent>,
    /// Elo floor
    elo_floor: f64,
}

impl Tournament {
    pub fn new(elo_floor: f64, algorithms: Vec<Box<dyn Algorithm + Sync>>) -> Self {
        Self {
            elo_floor,
            agents: algorithms.into_iter().map(Agent::new).collect(),
        }
    }

    /// Simulate one round robin round
    pub fn round_robin(&mut self) {
        let games: Vec<_> = repeat(0..self.agents.len())
            .take(Player::N)
            .multi_cartesian_product()
            .map(|agents| <[usize; Player::N]>::try_from(agents).unwrap())
            .collect();

        let scores: Vec<_> = games
            .into_par_iter()
            .filter_map(|agents| self.simulate_game(agents).map(|scores| (agents, scores)))
            .collect();

        for (agents, score) in scores {
            self.update_elo(agents, score)
        }
    }

    /// Run a single game with 4 agents
    pub fn simulate_game(&self, agents: [usize; Player::N]) -> Option<[GameStats; Player::N]> {
        // skip the game if all the players are the same, as elo will never change
        #[cfg(not(debug_assertions))]
        if agents.iter().all_equal() {
            return None;
        }

        // create a new rng
        let mut rng = rand::thread_rng();

        let mut game = State::new(20, 20);
        let mut alive = true;
        let mut times = [Duration::default(); Player::N];
        // run as long as a player is still playing
        while alive {
            alive = false;
            for player in Player::iter() {
                let pid = usize::from(player);

                let agent = &self.agents[agents[pid]];
                let now = Instant::now();
                if let Some(mv) = agent.algorithm.decide(&mut rng, &game, &player) {
                    game.place_piece(&mv);
                    alive = true;
                }
                times[pid] += now.elapsed();
                #[cfg(debug_assertions)]
                println!("Player {} played:\n{:?}", pid, game);
            }
        }

        let scores = game.scores();

        Some(array::from_fn(|pid| GameStats {
            score: scores[pid],
            elapsed: times[pid],
        }))
        // println!(
        //     "Game {:?} had scores {:?}",
        //     agents
        //         .iter()
        //         .map(|v| self.agents[*v].algorithm.name())
        //         .collect::<Vec<_>>(),
        //     scores
        // );
    }

    pub fn update_elo(&mut self, agents: [usize; Player::N], stats: [GameStats; Player::N]) {
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

                let s = match stats[player].score.cmp(&stats[o_player].score) {
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
            let agent = &mut self.agents[agents[player]];
            agent.elo += elo_diffs[player];

            // ELO floor of 100
            if agent.elo < self.elo_floor {
                agent.elo = self.elo_floor;
            }
            agent.cumulative_points += stats[player].score as usize;
            agent.games_played += 1;
            agent.elapsed += stats[player].elapsed;
        }
    }
}

impl Display for Tournament {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // maximum width of names
        let w = self
            .agents
            .iter()
            .map(|agent| agent.algorithm.name().len())
            .max()
            .unwrap()
            + 2;
        writeln!(
            f,
            "{: <w$}{: <15}{: <15}{: <15}{: <15}",
            "Algorithm", "ELO", "Avg Pts", "Avg ms / Game", "Games Played",
        )?;
        for agent in &self.agents {
            writeln!(
                f,
                "{: <w$}{: <15.4}{: <15.4}{: <15.4}{: <15.4}",
                agent.algorithm.name(),
                agent.elo,
                (agent.cumulative_points as f32) / (agent.games_played as f32),
                (agent.elapsed.as_millis() as f32) / (agent.games_played as f32),
                agent.games_played,
            )?;
        }
        Ok(())
    }
}
