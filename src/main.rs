use std::time::Instant;

use blokus_ai::game::{Player, State};
use itertools::MultiUnzip;
use rand::seq::SliceRandom;

struct Stats {
    move_ns: u128,
    place_ns: u128,
    fanout: usize,
}

fn geometric_mean(values: &[f64]) -> f64 {
    // Take the avg of logarithms
    let avg: f64 = values.iter().copied().map(f64::ln).sum();
    // Take the exponent of the avg
    f64::exp(avg / values.len() as f64)
}

fn arithmetic_mean(values: &[f64]) -> f64 {
    values.iter().copied().sum::<f64>() / values.len() as f64
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut stats: Vec<Stats> = Vec::new();

    let debug = cfg!(debug_assertions);

    loop {
        let mut game = State::new(20, 20);

        loop {
            let mut played = false;
            for player in Player::iter() {
                let now = Instant::now();
                let moves = game.get_moves(&player);
                let moves: Vec<_> = moves;
                let move_elapsed = now.elapsed();

                if debug {
                    println!("Calculation took {} ns", move_elapsed.as_nanos());
                    println!("Player {} has {} moves", player, moves.len());
                }

                if moves.is_empty() {
                    continue;
                }

                // Choose a random move
                let move_ = moves.choose(&mut rng).unwrap();

                let now = Instant::now();
                game.place_piece(&player, move_);
                let place_elapsed = now.elapsed();
                if debug {
                    println!("{:?}", game);
                }
                played = true;

                stats.push(Stats {
                    move_ns: move_elapsed.as_nanos(),
                    place_ns: place_elapsed.as_nanos(),
                    fanout: moves.len(),
                })
            }

            if !played {
                break;
            }
        }

        let stats: (Vec<_>, Vec<_>, Vec<_>) = stats
            .iter()
            .map(|s| (s.move_ns as f64, s.place_ns as f64, s.fanout as f64))
            .multiunzip();

        println!(
            "Average move calculation time:\n\tArithmetic:{} micros",
            arithmetic_mean(&stats.0) / 1000.,
        );

        println!(
            "Average place calculation time:\n\tArithmetic:{} micros",
            arithmetic_mean(&stats.1) / 1000.,
        );

        println!(
            "Average fanout:\n\tGeometric:{}\n\tArithmetic:{}",
            geometric_mean(&stats.2),
            arithmetic_mean(&stats.2)
        );

        // don't loop forever if on debug mode
        if debug {
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use blokus_ai::game::ver_1::Move as Move1;
    use blokus_ai::game::ver_1::State as State1;
    use blokus_ai::game::ver_3::Move as Move3;
    use blokus_ai::game::ver_3::State as State3;
    use blokus_ai::game::Player;

    #[test]
    fn test_move_counts_match() {
        // make sure move counts match across algorithms
        // only checking version 2 and 3 because they have the same indexing method and also don't have 5 tall/wide pieces

        let mut game1 = State1::new(20, 20);
        let mut game3 = State3::new(20, 20);

        for player in Player::iter() {
            let moves1 = game1.get_moves(&player);
            let moves3 = game3.get_moves(&player);

            println!("{:?}", (moves1.len(), moves3.len()));

            assert!(moves1.len() == moves3.len());
        }

        // // Place piece
        // // XX
        // //  X
        // game1.place_piece(&Player::Player1, &Move1::new(PieceTransformID { piece: (), version: () }, (0, 0)));
        // game3.place_piece(&Player::Player1, &Move3::new(5, (0, 0)));

        // for player in Player::iter() {
        //     let moves2 = game1.get_moves(&player);
        //     let moves3 = game3.get_moves(&player);

        //     assert!(moves2.len() == moves3.len());
        // }
    }
}
