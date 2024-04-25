use super::Algorithm;
use rand::Rng;

/// Algorithm that stochastically chooses between two other algorithms.
/// `ratio * 100` percent of the time the algorithm will play according
/// to the roles of alg1, and will play alg2 otherwise.
pub struct Mix<Alg1, Alg2>
where
    Alg1: Algorithm,
    Alg2: Algorithm,
{
    alg1: Alg1,
    alg2: Alg2,
    /// Probability ratio between choosing alg1 and alg2
    ratio: f64,
}

unsafe impl<Alg1, Alg2> Sync for Mix<Alg1, Alg2>
where
    Alg1: Algorithm + Sync,
    Alg2: Algorithm + Sync,
{
}

impl<Alg1, Alg2> Mix<Alg1, Alg2>
where
    Alg1: Algorithm,
    Alg2: Algorithm,
{
    pub fn new(alg1: Alg1, alg2: Alg2, ratio: f64) -> Self {
        debug_assert!(ratio < 1. && ratio > 0.);
        Self { alg1, alg2, ratio }
    }
}

impl<Alg1, Alg2> Mix<Alg1, Alg2>
where
    Alg1: Algorithm + Default,
    Alg2: Algorithm + Default,
{
    pub fn new_ratio(ratio: f64) -> Self {
        debug_assert!(ratio < 1. && ratio > 0.);
        Self {
            alg1: Default::default(),
            alg2: Default::default(),
            ratio,
        }
    }
}

impl<Alg1, Alg2> Algorithm for Mix<Alg1, Alg2>
where
    Alg1: Algorithm,
    Alg2: Algorithm,
{
    fn decide(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        state: &crate::game::State,
        player: &crate::game::Player,
    ) -> Option<crate::game::Move> {
        match rng.gen_bool(self.ratio) {
            true => self.alg1.decide(rng, state, player),
            false => self.alg2.decide(rng, state, player),
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
