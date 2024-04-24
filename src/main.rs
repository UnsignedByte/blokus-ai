use blokus_ai::evaluate::{Distance, Greedy, Mix, Random, Tournament};
use std::fmt::Write;
use std::time::Instant;

fn main() {
    let mut tournament = Tournament::new(vec![
        Box::new(Random::default()),
        Box::new(Mix::<Greedy, Random>::new_ratio(0.5)),
        Box::new(Greedy::default()),
        Box::new(Distance::ClosestToCenter),
        // Box::new(Distance::FarthestFromCenter),
        Box::new(Distance::ClosestToCorner),
        // Box::new(Distance::FarthestFromCorner),
    ]);

    loop {
        let now = Instant::now();
        tournament.round_robin();
        println!("Round robin took {} ms", now.elapsed().as_millis());

        let scores = tournament
            .scores()
            .into_iter()
            .fold(String::new(), |mut acc, (name, elo)| {
                writeln!(acc, "{}: {}", name, elo).unwrap();
                acc
            });

        println!("{}", scores);
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
