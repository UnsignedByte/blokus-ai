use crate::game::{piece_dims, Player};

use super::Algorithm;

/// Algorithm that sorts moves by distance to a position
pub enum Distance {
    ClosestToCorner,
    FarthestFromCorner,
    ClosestToCenter,
    FarthestFromCenter,
}

impl Algorithm for Distance {
    fn decide(
        &mut self,
        state: &crate::game::State,
        player: &crate::game::Player,
    ) -> Option<crate::game::Move> {
        state.get_moves(player).into_iter().min_by_key(|mv| {
            // Here, we get all 4 corners of the bounding box around the piece
            let (w, h) = piece_dims(mv);
            let (w, h) = (w as i8, h as i8);
            let (x, y) = mv.pos;

            // Four corners.
            let corners = [(x, y), (x, y + h), (x + w, y), (x + w, y + h)];

            let (rx, ry) = match self {
                Distance::ClosestToCorner | Distance::FarthestFromCorner => match player {
                    Player::Player1 => (0, 0),
                    Player::Player2 => (19 * 2, 0),
                    Player::Player3 => (19 * 2, 19 * 2),
                    Player::Player4 => (0, 19 * 2),
                },
                Distance::ClosestToCenter | Distance::FarthestFromCenter => (19, 19),
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
                        Distance::ClosestToCorner | Distance::ClosestToCenter => d,
                        Distance::FarthestFromCorner | Distance::FarthestFromCenter => -d, // Here, we want tiles far away so we invert the distance
                    }
                })
                .min()
        })
    }

    fn name(&self) -> String {
        match self {
            Distance::ClosestToCorner => "Closest to Start",
            Distance::FarthestFromCorner => "Farthest from Start",
            Distance::ClosestToCenter => "Closest to Center",
            Distance::FarthestFromCenter => "Farthest from Center",
        }
        .to_owned()
    }
}
