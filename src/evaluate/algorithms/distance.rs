use super::Algorithm;
use crate::game::{piece_dims, Player, State};
use rand::seq::SliceRandom;

/// Algorithm that sorts moves by distance to a position
pub enum Distance {
    TowardCorner,
    AwayFromCorner,
    TowardCenter,
    AwayFromCenter,
    TowardBestOpponent,
}

impl Distance {
    fn distance(&self, state: &State, mv: &crate::game::Move) -> i16 {
        // Here, we get all 4 corners of the bounding box around the piece
        let (w, h) = piece_dims(mv);
        let (w, h) = (w as i8, h as i8);
        let (x, y) = mv.pos;

        let player = &mv.player;

        // Four corners.
        let corners = [(x, y), (x, y + h), (x + w, y), (x + w, y + h)];

        // The position of the target (all coords multiplied by two)
        let (rx, ry) = match self {
            Distance::TowardCorner | Distance::AwayFromCorner => match player {
                Player::Player1 => (0, 0),
                Player::Player2 => (19 * 2, 0),
                Player::Player3 => (19 * 2, 19 * 2),
                Player::Player4 => (0, 19 * 2),
            },
            Distance::TowardCenter | Distance::AwayFromCenter => (19, 19),
            Distance::TowardBestOpponent => {
                // Loop through players and find the one with the best score
                let scores = state.scores();

                let best = Player::iter()
                    .filter(|p| p != player)
                    .max_by_key(|p| scores[usize::from(p)])
                    .unwrap();

                match best {
                    Player::Player1 => (0, 0),
                    Player::Player2 => (19 * 2, 0),
                    Player::Player3 => (19 * 2, 19 * 2),
                    Player::Player4 => (0, 19 * 2),
                }
            }
        };

        corners
            .iter()
            .map(|(cx, cy)| {
                // we double indices here because 9.5 is the true middle
                let (dx, dy) = (cx * 2 - rx, cy * 2 - ry);
                // increase size to avoid overflow
                let (dx, dy) = (dx as i16, dy as i16);

                let d = dx * dx + dy * dy;

                match self {
                    Distance::TowardBestOpponent
                    | Distance::TowardCorner
                    | Distance::TowardCenter => -d, // Here, we want nearby tiles so we invert the distance
                    Distance::AwayFromCorner | Distance::AwayFromCenter => d,
                }
            })
            .max()
            .unwrap()
    }
}

unsafe impl Send for Distance {}
unsafe impl Sync for Distance {}

impl Algorithm for Distance {
    fn name(&self) -> String {
        match self {
            Distance::TowardBestOpponent => "Toward Best Enemy",
            Distance::TowardCorner => "Toward Start",
            Distance::AwayFromCorner => "Away from Start",
            Distance::TowardCenter => "Toward Center",
            Distance::AwayFromCenter => "Away from Center",
        }
        .to_owned()
    }

    fn decide(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        state: &State,
        player: &Player,
    ) -> Option<crate::game::Move> {
        // we shuffle here so that ties are resolved randomly
        let mut moves = state.get_moves(player);
        moves.shuffle(rng);
        moves.into_iter().max_by_key(|mv| self.distance(state, mv))
    }
}
