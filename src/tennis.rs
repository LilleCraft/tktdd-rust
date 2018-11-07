/* TYPES */
#[derive(Debug)]
pub enum Player {
    PlayerOne,
    PlayerTwo,
}

/* Surely not the best choice */
pub type Point = i32;

pub struct PointsData {
    player_one: Point,
    player_two: Point,
}

/* Surely incomplete */
pub enum Score {
    Points(PointsData),
    Deuce,
    Game(Player),
}

/* IMPELMENT TOOLING TRAITS */
impl ToString for Player {
    fn to_string(&self) -> String {
        match self {
            Player::PlayerOne => String::from("Player 1"),
            Player::PlayerTwo => String::from("Player 2"),
        }
    }
}
impl PartialEq for Player {
    fn eq(&self, other: &Player) -> bool {
        self.to_string() == other.to_string()
    }
}
impl Clone for Player {
    fn clone(&self) -> Player {
        match self {
            Player::PlayerOne => Player::PlayerOne,
            Player::PlayerTwo => Player::PlayerTwo,
        }
    }
}

/* Surely incomplete */
impl ToString for Score {
    fn to_string(&self) -> String {
        String::from("replace this with your code")
    }
}

/* IMPLEMENT TOOLING FUNCTIONS */
pub fn other_player(p: &Player) -> Player {
    if let Player::PlayerOne = p {
        Player::PlayerTwo
    } else {
        Player::PlayerOne
    }
}

pub fn new_game() -> Score {
    Score::Points(PointsData {
        player_one: 0,
        player_two: 0,
    })
}

/* An exemple how to use option to avoid null values */
pub fn increment_point(point: Point) -> Option<Point> {
    match point {
        0 => Some(15),
        15 => Some(30),
        30 => Some(40),
        _ => None, /* Outch ! How int could solve Advantage and End of game ? */
    }
}
/* An exemple how to extract values from Option<T> value*/
pub fn read_from_option_point(op: Option<Point>) -> Point {
    op.unwrap_or(0)
}

/* IMPELMENT TRANSITIONS */
pub fn score_when_deuce(winner: Player) -> Score {
    Score::Game(winner)
}

pub fn score_when_advantage(advantaged_player: Player, winner: Player) -> Score {
    // Declare the type of a function telling the compilater that you will implement it later !
    unimplemented!()
}

pub fn score_when_forty(current_at_forty: Player, winner: Player) -> Score {
    unimplemented!()
}

pub fn score_when_game(winner: Player) -> Score {
    unimplemented!()
}

pub fn score(current_score: Score, winner: Player) -> Score {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_one_to_string() {
        let p1 = Player::PlayerOne.to_string();
        assert_eq!("Player 1", p1);
    }

    #[test]
    fn player_one_other_player_is_player_two() {
        let p = Player::PlayerOne;
        assert_eq!(Player::PlayerTwo, other_player(&p));
    }

    #[test]
    fn given_deuce_when_player1_wins() {
        let p = Player::PlayerOne;
        assert_eq!("Advantage Player 1", score_when_deuce(p).to_string());
    }

    #[test]
    fn given_advantage_when_advantaged_player_wins_then_score_is_game_to_the_advantaged_player() {
        let advantaged_player = Player::PlayerOne;
        let winner = advantaged_player.clone();
        assert_eq!(
            "Game Player 1",
            score_when_advantage(advantaged_player, winner).to_string()
        );
    }

    #[test]
    fn given_advantage_when_the_other_player_wins_then_score_is_deuce() {
        let advantaged_player = Player::PlayerOne;
        let winner = other_player(&advantaged_player);
        assert_eq!(
            "Deuce",
            score_when_advantage(advantaged_player, winner).to_string()
        );
    }

    #[test]
    fn given_player_40_when_player_at_40_wins_then_score_is_game_for_this_player() {
        unimplemented!()
    }

    #[test]
    fn given_player_40_other_30_when_other_wins_then_score_is_deuce() {
        unimplemented!()
    }

    #[test]
    fn given_player_40_other_15_when_other_wins_then_score_is_40_30() {
        unimplemented!()
    }

    #[test]
    fn given_player_15_other_15_when_player_wins_then_score_is_30_15() {
        unimplemented!()
    }

    #[test]
    fn given_player_0_other_15_when_other_wins_then_score_is_0_30() {
        unimplemented!()
    }

    #[test]
    fn given_player_30_other_15_when_player_wins_then_score_is_40_15() {
        unimplemented!()
    }
}
