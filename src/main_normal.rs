use std::time::Instant;

use blokus_ai::game::{Mask, Piece, Player, State};
use once_cell::sync::Lazy;
use rand::seq::SliceRandom;

static PIECES: Lazy<[Vec<Piece>; 4]> = Lazy::new(|| {
    let blocks = vec![
        // 1 tile
        Mask::new(1, vec![0x1]),
        // 2 tiles
        Mask::new(2, vec![0x11]),
        // 3 tiles
        Mask::new(2, vec![0x11, 0x01]),
        Mask::new(3, vec![0x111]),
        // 4 tiles
        Mask::new(4, vec![0x1111]),
        Mask::new(3, vec![0x111, 0x001]),
        Mask::new(3, vec![0x110, 0x011]),
        Mask::new(2, vec![0x11, 0x11]),
        Mask::new(3, vec![0x111, 0x010]),
        // 5 tiles
        Mask::new(3, vec![0x011, 0x110, 0x010]),
        Mask::new(5, vec![0x11111]),
        Mask::new(4, vec![0x1111, 0x1000]),
        Mask::new(4, vec![0x0111, 0x1100]),
        Mask::new(3, vec![0x111, 0x110]),
        Mask::new(3, vec![0x111, 0x010, 0x010]),
        Mask::new(3, vec![0x111, 0x101]),
        Mask::new(3, vec![0x111, 0x100, 0x100]),
        Mask::new(3, vec![0x001, 0x011, 0x110]),
        Mask::new(3, vec![0x010, 0x111, 0x010]),
        Mask::new(4, vec![0x1111, 0x0100]),
        Mask::new(3, vec![0x110, 0x010, 0x011]),
    ];

    // Uses a hack to generate the pieces for all 4 players.
    // Given a piece that looks like
    // 010
    // 111
    // for example, note that shifting each row to the left by one
    // gives the piece
    // 020
    // 222
    // which is the same piece for player 2.
    // This is done for all 4 players.

    [
        blocks.clone().into_iter().map(Piece::new).collect(),
        blocks
            .clone()
            .into_iter()
            .map(|block| block << 1)
            .map(Piece::new)
            .collect(),
        blocks
            .clone()
            .into_iter()
            .map(|block| block << 2)
            .map(Piece::new)
            .collect(),
        blocks
            .into_iter()
            .map(|block| block << 3)
            .map(Piece::new)
            .collect(),
    ]
});

fn main() {
    let mut rng = rand::thread_rng();

    let mut avg_move_t = 0u128;
    let mut avg_place_t = 0u128;
    let mut avg_fanout = 0usize;

    let mut turns = 0u32;
    let mut pieces: [bool; 400 * 21 * 8] = [false; 400 * 21 * 8];

    loop {
        let mut game = State::new(20, 20, &PIECES);

        loop {
            let mut played = false;
            for player in Player::iter() {
                // fill the pieces buffer with zeros
                pieces.fill(false);
                let now = Instant::now();
                let moves = game.get_moves(&player);
                // faster way to filter only unique moves
                let moves = moves.filter(|m| {
                    let piece = usize::from(m.piece.piece);
                    let ver = m.piece.version;
                    let (x, y) = m.pos;
                    let uid = ((x * 20 + y) * 21 + piece) * 8 + ver;
                    let seen = pieces[uid];
                    pieces[uid] = true;
                    !seen
                });
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

                turns += 1;
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
