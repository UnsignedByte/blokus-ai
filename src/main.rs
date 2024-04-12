use std::time::Instant;

use blokus_ai::game::{Player, State};
use rand::seq::SliceRandom;

fn main() {
    let mut rng = rand::thread_rng();

    let mut avg_move_t = 0u128;
    let mut avg_place_t = 0u128;
    let mut avg_fanout = 0usize;

    let mut turns = 0u32;
    let mut pieces: [bool; 400 * 21 * 8] = [false; 400 * 21 * 8];

    loop {
        let mut game = State::new(20, 20);

        loop {
            let mut played = false;
            for player in Player::iter() {
                turns += 1;

                // fill the pieces buffer with zeros
                pieces.fill(false);
                let now = Instant::now();
                let moves = game.get_moves(&player);
                // faster way to filter only unique moves
                // let moves = moves.filter(|m| {
                //     let piece = usize::from(m.piece.piece);
                //     let ver = m.piece.version;
                //     let (x, y) = m.pos;
                //     let uid = ((x * 20 + y) * 21 + piece) * 8 + ver;
                //     let seen = pieces[uid];
                //     pieces[uid] = true;
                //     !seen
                // });
                let moves: Vec<_> = moves.collect();
                let elapsed = now.elapsed();
                avg_move_t += elapsed.as_nanos();

                // println!("Calculation took {} ns", elapsed.as_nanos());
                // println!("Player {} has {} moves", player, moves.len());
                avg_fanout += moves.len();

                if moves.is_empty() {
                    continue;
                }

                // Choose a random move
                let move_ = moves.choose(&mut rng).unwrap();

                let now = Instant::now();
                game.place_piece(&player, move_);
                let elapsed = now.elapsed();
                avg_place_t += elapsed.as_nanos();
                // println!("{:?}", game);
                played = true;
            }

            if !played {
                break;
            }
        }

        println!(
            "Average move calculation time:\n\t{} ns = {} millis",
            avg_move_t as f64 / turns as f64,
            avg_move_t as f64 / turns as f64 / 1000000.
        );

        println!(
            "Average move execution time:\n\t{} ns = {} millis",
            avg_place_t as f64 / turns as f64,
            avg_place_t as f64 / turns as f64 / 1000000.
        );

        println!(
            "Average fanout: {} over {} turns",
            avg_fanout as f64 / turns as f64,
            turns
        );
    }
}
