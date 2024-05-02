use blokus_ai::evaluate::{
    Distance, EnemyMoveCount, GreedyMax, GreedyMin, MiniMax, Mix, MonteCarlo, MoveCount, Opening,
    Random, Rollout, Score, Tournament,
};
use std::{path::PathBuf, time::Instant};

fn main() {
    let tpath = PathBuf::from("tournament.json");

    let mut tournament = Tournament::new(
        100.,
        1200.,
        200.,
        vec![
            Box::new(Random),
            Box::new(GreedyMin::<Score>::default()),
            Box::new(GreedyMin::<MoveCount>::default()),
            Box::new(Mix::<GreedyMax<Score>, Random>::new_ratio(0.5)),
            Box::new(Mix::<GreedyMax<MoveCount>, Random>::new_ratio(0.5)),
            Box::new(GreedyMin::<EnemyMoveCount>::default()),
            Box::new(Distance::TowardCenter),
            Box::new(Distance::AwayFromCenter),
            Box::new(Distance::TowardCorner),
            Box::new(Distance::AwayFromCorner),
            Box::new(Distance::TowardBestOpponent),
            Box::new(GreedyMax::<Score>::default()),
            Box::new(GreedyMax::<MoveCount>::default()),
            Box::new(GreedyMax::new(Rollout::new(25))),
            // Box::new(MonteCarlo::new(1000, f64::sqrt(2.))),
            Box::new(Mix::<GreedyMax<Score>, GreedyMax<MoveCount>>::new_ratio(
                0.5,
            )),
            Box::new(Mix::<GreedyMax<Score>, GreedyMax<MoveCount>>::new_ratio(
                0.25,
            )),
            Box::new(Mix::<GreedyMax<Score>, GreedyMax<MoveCount>>::new_ratio(
                0.25,
            )),
            Box::new(Mix::<GreedyMin<EnemyMoveCount>, GreedyMax<MoveCount>>::new_ratio(0.5)),
            Box::new(Mix::new(
                Distance::TowardBestOpponent,
                GreedyMax::<Score>::default(),
                0.5,
            )),
            Box::new(Opening::new(
                Distance::TowardCenter,
                GreedyMax::<Score>::default(),
                5,
            )),
            Box::new(Opening::new(
                Distance::TowardCenter,
                GreedyMax::<Score>::default(),
                6,
            )),
            Box::new(Opening::new(
                Distance::TowardCenter,
                GreedyMax::<Score>::default(),
                4,
            )),
            Box::new(Opening::new(
                Distance::TowardCenter,
                Mix::<GreedyMax<Score>, GreedyMax<MoveCount>>::new_ratio(0.75),
                5,
            )),
            Box::new(Opening::new(
                Distance::TowardCenter,
                Mix::<GreedyMax<Score>, GreedyMax<MoveCount>>::new_ratio(0.5),
                5,
            )),
            Box::new(Opening::new(
                Distance::TowardCenter,
                MiniMax::<2, MoveCount>::default(),
                5,
            )),
            Box::new(MiniMax::<2, MoveCount>::default()),
            Box::new(MiniMax::<3, Score>::default()),
        ],
        std::fs::File::open(tpath.clone()).ok(),
    )
    .expect("Failed to load tournament");

    loop {
        let now = Instant::now();
        tournament.play_least_played(250);
        println!("Round took {} s", now.elapsed().as_secs());

        println!("{}", tournament);
        tournament
            .save(tpath.clone())
            .expect("Failed to save tournament");
    }
}

#[cfg(test)]
mod test {
    use blokus_ai::game::ver_1::State as State1;
    use blokus_ai::game::ver_3::State as State3;
    use blokus_ai::game::Player;

    #[test]
    fn test_move_counts_match() {
        // make sure move counts match across algorithms
        // only checking version 2 and 3 because they have the same indexing method and also don't have 5 tall/wide pieces

        let game1 = State1::new(20, 20);
        let game3 = State3::new(20, 20);

        for player in Player::iter() {
            let moves1 = game1.get_moves(&player);
            let moves3 = game3.get_moves(&player);

            println!("{:?}", (moves1.len(), moves3.len()));

            assert!(moves1.len() == moves3.len());
        }
    }
}
