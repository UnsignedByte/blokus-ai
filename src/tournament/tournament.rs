use super::Algorithm;

/// Player in a tournament
struct Agent<'t> {
    algorithm: &'t mut dyn Algorithm,
    score: f64,
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
                .map(|algorithm| Agent {
                    algorithm,
                    score: 0.,
                })
                .collect(),
        }
    }
}
