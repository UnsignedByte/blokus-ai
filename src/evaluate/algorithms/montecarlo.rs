use super::Algorithm;
use crate::game::{Move, Player, State};
use rand::{
    rngs::ThreadRng,
    seq::{IteratorRandom, SliceRandom},
};

/// An initialized node in the Monte Carlo tree
struct Branch {
    state: State,
    player: Player,
    children: Vec<Node>,
    /// Move that led to this node
    mv: Option<Move>,
    wins: usize,
    visits: usize,
    /// How many players did not play in their last turn
    done: u8,
}

/// Get the winner at a given state
/// Ties are broken randomly
fn get_winner(rng: &mut rand::rngs::ThreadRng, state: &State) -> Player {
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

    Player::from(winner)
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

    get_winner(rng, &state) == player
}

enum Node {
    /// A branch node
    Branch(Branch),
    /// A node that has not been initialized yet, contains only the move that led to it
    /// Leaf node
    Leaf(Option<Move>),
}

impl Node {
    /// Create a root node
    /// Returns [None] if the player has no moves
    pub fn root(state: State, player: Player) -> Option<Self> {
        // figure out how many moves each player has
        let mut done = 0;
        let mut children = None;
        for p in Player::iter() {
            let moves = state.get_moves(&p);
            if p == player {
                if moves.is_empty() {
                    return None;
                }
                children = Some(moves.iter().cloned().map(|m| Node::Leaf(Some(m))).collect());
            }
            done += moves.is_empty() as u8;
        }

        Some(Self::Branch(Branch {
            state,
            player,
            children: children.unwrap(),
            mv: None,
            wins: 0,
            visits: 0,
            done,
        }))
    }

    /// Get the UCT value of this node
    #[inline]
    pub fn score(&self, c: f64, parent_visits: f64) -> f64 {
        match self {
            Node::Branch(Branch { wins, visits, .. }) => {
                *wins as f64 / *visits as f64 + c * (parent_visits / *visits as f64).sqrt()
            }
            // Unvisited nodes always take precedence
            Node::Leaf(_) => f64::INFINITY,
        }
    }

    pub fn rollout(&mut self, rng: &mut ThreadRng, c: f64) -> bool {
        match self {
            Node::Branch(Branch {
                state,
                player,
                children,
                wins,
                visits,
                done,
                ..
            }) => {
                // This node has already been initialized
                // Here, we choose a child based on the UCT algorithm described in
                // https://en.wikipedia.org/wiki/Monte_Carlo_tree_search

                if *done >= Player::N as u8 {
                    // All players are done, the game is over
                    return get_winner(rng, state) == *player;
                }
                // shuffle the children to avoid bias
                children.shuffle(rng);

                let best = children
                    .iter_mut()
                    .map(|n| {
                        let score = n.score(c, *visits as f64);
                        (score, n)
                    })
                    .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
                    .unwrap()
                    .1;

                let rollout = if let Node::Leaf(mv) = best {
                    // Initialize the node
                    let mut nstate = state.clone();
                    // place the move if it exists
                    if let Some(mv) = mv {
                        nstate.place_piece(mv);
                    }
                    let nplayer = player.next();
                    let mut ndone = *done;

                    let nmoves = nstate.get_moves(&nplayer);
                    let nmoves = if nmoves.is_empty() {
                        ndone += 1;
                        // The player has no moves, create a leaf with an empty move
                        vec![Node::Leaf(None)]
                    } else {
                        ndone = 0;
                        nmoves
                            .iter()
                            .cloned()
                            .map(|m| Node::Leaf(Some(m)))
                            .collect()
                    };

                    let rollout = random_rollout(rng, &nstate, nplayer);

                    *best = Node::Branch(Branch {
                        state: nstate,
                        player: nplayer,
                        mv: mv.clone(),
                        children: nmoves,
                        wins: rollout as usize,
                        visits: 1,
                        done: ndone,
                    });

                    rollout
                } else {
                    best.rollout(rng, c)
                };

                *visits += 1;
                *wins += rollout as usize;

                // Propogate the win upward
                rollout
            }
            Node::Leaf(_) => unreachable!("Cannot rollout an uninitialized node"),
        }
    }
}

pub struct MonteCarlo {
    /// Number of simulations to run
    simulations: usize,
    /// Constant for the UCT algorithm
    c: f64,
}

impl MonteCarlo {
    pub fn new(simulations: usize, c: f64) -> Self {
        Self { simulations, c }
    }
}

impl Algorithm for MonteCarlo {
    fn decide(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        state: &State,
        player: &Player,
    ) -> Option<Move> {
        if let Some(mut root) = Node::root(state.clone(), *player) {
            for _ in 0..self.simulations {
                root.rollout(rng, self.c);
            }

            let Node::Branch(Branch { mut children, .. }) = root else {
                unreachable!("Root node is not a branch node");
            };

            // shuffle the children to avoid bias
            children.shuffle(rng);

            // Find the node with the most visits
            children
                .iter()
                .max_by_key(|n| match n {
                    Node::Branch(Branch { visits, .. }) => *visits,
                    Node::Leaf(_) => 0,
                })
                .and_then(|best| match best {
                    Node::Branch(Branch { mv, .. }) => mv.clone(),
                    Node::Leaf(mv) => mv.clone(),
                })
        } else {
            None
        }
    }

    fn name(&self) -> String {
        "Monte Carlo".to_owned()
    }
}
