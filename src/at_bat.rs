use crate::at_bat::Outcome::*;
use crate::player::Player;

enum Outcome {
    StrikeOut,
    Single,
    Double,
    Triple,
    HomeRun,
    Walk,
}

pub struct AtBat {
    player: Player,
    strikes: u8,
    balls: u8,
    outcome: Option<Outcome>,
}

impl AtBat {
    pub fn new(player: Player) -> Self {
        AtBat {
            player,
            strikes: 0,
            balls: 0,
            outcome: None,
        }
    }

    pub fn score_strike(&mut self) {
        self.strikes += 1;

        if self.strikes >= 3 {
            self.outcome = Some(StrikeOut);
        }
    }

    pub fn score_ball(&mut self) {
        self.balls += 1;

        if self.balls >= 4 {
            self.outcome = Some(Walk);
        }
    }
}
