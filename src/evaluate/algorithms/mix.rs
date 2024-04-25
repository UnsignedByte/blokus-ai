use super::Algorithm;
use rand::Rng;
use std::clone::Clone;

/// Algorithm that stochastically chooses between two other algorithms.
/// `ratio * 100` percent of the time the algorithm will play according
/// to the roles of alg1, and will play alg2 otherwise.
#[derive(Clone)]
pub struct Mix<Alg1, Alg2>
where
    Alg1: Algorithm + Clone,
    Alg2: Algorithm + Clone,
{
    rng: rand::rngs::ThreadRng,
    alg1: Alg1,
    alg2: Alg2,
    /// Probability ratio between choosing alg1 and alg2
    ratio: f64,
}

impl<Alg1, Alg2> Mix<Alg1, Alg2>
where
    Alg1: Algorithm + Clone,
    Alg2: Algorithm + Clone,
{
    pub fn new(alg1: Alg1, alg2: Alg2, ratio: f64) -> Self {
        debug_assert!(ratio < 1. && ratio > 0.);
        Self {
            alg1,
            alg2,
            ratio,
            rng: rand::thread_rng(),
        }
    }
}

impl<Alg1, Alg2> Mix<Alg1, Alg2>
where
    Alg1: Algorithm + Clone + Default,
    Alg2: Algorithm + Clone + Default,
{
    pub fn new_ratio(ratio: f64) -> Self {
        debug_assert!(ratio < 1. && ratio > 0.);
        Self {
            alg1: Default::default(),
            alg2: Default::default(),
            ratio,
            rng: rand::thread_rng(),
        }
    }
}

impl<Alg1, Alg2> Algorithm for Mix<Alg1, Alg2>
where
    Alg1: Algorithm + Clone,
    Alg2: Algorithm + Clone,
{
    fn decide(
        &mut self,
        state: &crate::game::State,
        player: &crate::game::Player,
    ) -> Option<crate::game::Move> {
        match self.rng.gen_bool(self.ratio) {
            true => self.alg1.decide(state, player),
            false => self.alg2.decide(state, player),
        }
    }

    fn name(&self) -> String {
        format!(
            "{}% {}, {}% {}",
            (self.ratio * 100.) as usize,
            self.alg1.name(),
            100 - (self.ratio * 100.) as usize,
            self.alg2.name()
        )
    }
}
