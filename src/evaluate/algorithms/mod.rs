mod algorithm;
mod distance;
mod greedy;
mod heuristics;
mod minimax;
mod mix;
mod montecarlo;
mod random;

pub use algorithm::Algorithm;
pub use distance::Distance;
pub use greedy::{GreedyMax, GreedyMin};
pub use heuristics::*;
pub use minimax::MiniMax;
pub use mix::{Mix, Opening};
pub use montecarlo::MonteCarlo;
pub use random::Random;
