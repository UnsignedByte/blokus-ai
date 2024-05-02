use super::Algorithm;
use crate::game::{Player, State};
use colored::Colorize;
use itertools::Itertools;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    array,
    cmp::min,
    collections::HashMap,
    fmt::Display,
    iter::repeat,
    path::PathBuf,
    time::{Duration, Instant},
};

/// Type alias for the serialization of the tournament statistics
type Store = HashMap<String, (f64, usize, usize, Duration)>;

/// Player in a tournament
/// Contains statistics about the player
struct Agent {
    algorithm: Box<dyn Algorithm + Sync + Send>,
    elo: f64,
    games_played: usize,
    cumulative_points: usize,
    elapsed: Duration,
}

impl Agent {
    pub fn new(algorithm: Box<dyn Algorithm + Sync + Send>, elo: f64) -> Self {
        Self {
            algorithm,
            elo,
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
    /// Range of ELO values that agents can play against
    elo_range: f64,
}

impl Tournament {
    pub fn new(
        elo_floor: f64,
        starting_elo: f64,
        elo_range: f64,
        algorithms: Vec<Box<dyn Algorithm + Sync + Send>>,
        load: Option<std::fs::File>,
    ) -> Result<Self, serde_json::Error> {
        // Load Store from file
        let load: Option<Store> = load.map(serde_json::from_reader).transpose()?;
        let load = load.unwrap_or_default();
        Ok(Self {
            elo_floor,
            elo_range,
            agents: algorithms
                .into_iter()
                .map(|alg| match load.get(&alg.name()).cloned() {
                    Some((elo, games_played, cumulative_points, elapsed)) => Agent {
                        algorithm: alg,
                        elo,
                        games_played,
                        cumulative_points,
                        elapsed,
                    },
                    None => Agent::new(alg, starting_elo),
                })
                .collect(),
        })
    }

    pub fn save(&self, path: PathBuf) -> Result<(), serde_json::Error> {
        let store = Store::from(self);
        let file = std::fs::File::create(path).unwrap();
        serde_json::to_writer(file, &store)
    }

    /// Simulate one round robin round
    pub fn round_robin(&mut self) {
        let games: Vec<_> = repeat(0..self.agents.len())
            .take(Player::N)
            .multi_cartesian_product()
            .map(|agents| <[usize; Player::N]>::try_from(agents).unwrap())
            .collect();

        let scores: Vec<_> = games
            .into_iter()
            .filter_map(|agents| self.simulate_game(agents).map(|scores| (agents, scores)))
            .collect();

        for (agents, score) in scores {
            self.update_elo(agents, score)
        }
    }

    /// Play one game for each agent
    /// Each agent chooses opponents with similar ELO
    pub fn stochastic_round(&mut self) {
        let scores: Vec<_> = (0..self.agents.len())
            .into_par_iter()
            .map(|i| self.random_game(i))
            .filter_map(|agents| self.simulate_game(agents).map(|scores| (agents, scores)))
            .collect();

        for (agents, score) in scores {
            self.update_elo(agents, score)
        }
    }

    pub fn play_least_played(&mut self, count: usize) {
        // Find the agent with the least games played
        let least_played = self
            .agents
            .iter()
            .enumerate()
            .min_by_key(|(_, a)| a.games_played)
            .unwrap()
            .0;

        // Play games with the least played agent `count` times
        let scores: Vec<_> = (0..count)
            .into_par_iter()
            .map(|_| self.random_game(least_played))
            .filter_map(|agents| self.simulate_game(agents).map(|scores| (agents, scores)))
            .collect();

        for (agents, score) in scores {
            self.update_elo(agents, score)
        }
    }

    /// Have a single agent choose random opponents to play against
    /// that have similar ELO
    pub fn random_game(&self, i: usize) -> [usize; Player::N] {
        let mut rng = rand::thread_rng();
        let agent = &self.agents[i];
        let elo = agent.elo;
        // Find all agents within the elo range
        let opponents = {
            let mut elo_range = self.elo_range;
            loop {
                let opponents: Vec<_> = self
                    .agents
                    .iter()
                    .enumerate()
                    .filter(|(_, a)| (a.elo - elo).abs() < elo_range)
                    .map(|(i, _)| i)
                    .collect();

                if opponents.len() > Player::N {
                    break opponents;
                }

                // If we don't have enough opponents, increase the range
                elo_range *= 2.;
            }
        };

        let mut players: [usize; Player::N] = array::from_fn(|i| match i {
            0 => i,
            _ => *opponents.choose(&mut rng).unwrap(),
        });

        // shuffle the players
        players.shuffle(&mut rng);
        players
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
        let mut move_number = 0;
        // run as long as a player is still playing
        while alive {
            alive = false;
            move_number += 1;
            for player in Player::iter() {
                let pid = usize::from(player);

                let agent = &self.agents[agents[pid]];
                let now = Instant::now();
                if let Some(mv) = agent
                    .algorithm
                    .decide(&mut rng, &game, &player, move_number)
                {
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

            // The K value (taken from USCF) is 800 / (N_e + m),
            // where N_e is the effective number of games played
            // and m is the number of games played in this tournament (1 as we update every game)
            let k: f64 = 800. / (min(agent.games_played, 30) as f64 + 1.);
            // We divide the K value by the number of opponents as each player has "played" 3 games
            let k: f64 = k / (Player::N - 1) as f64;

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

                elo_diffs[player] += k * (s - ea);
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
        // sort agents by ELO
        for agent in self
            .agents
            .iter()
            .sorted_by(|a, b| a.elo.partial_cmp(&b.elo).unwrap())
        {
            writeln!(
                f,
                "{: <w$}{: <15.4}{: <15.4}{: <15.4}{: <15.4}",
                agent.algorithm.name().yellow(),
                agent.elo,
                (agent.cumulative_points as f32) / (agent.games_played as f32),
                (agent.elapsed.as_millis() as f32) / (agent.games_played as f32),
                agent.games_played,
            )?;
        }
        Ok(())
    }
}

impl From<&Tournament> for Store {
    fn from(value: &Tournament) -> Self {
        value
            .agents
            .iter()
            .map(|agent| {
                (
                    agent.algorithm.name(),
                    (
                        agent.elo,
                        agent.games_played,
                        agent.cumulative_points,
                        agent.elapsed,
                    ),
                )
            })
            .collect()
    }
}
