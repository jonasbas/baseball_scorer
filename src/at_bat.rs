use crate::at_bat::Outcome::*;
use crate::player::Player;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Outcome {
    TBD,
    StrikeOut,
    Single,
    Double,
    Triple,
    HomeRun,
    Walk,
    GroundOut,
    FlyOut,
}

#[derive(Debug, Clone)]
pub struct AtBat {
    player: Player,
    strikes: u8,
    balls: u8,
    pub outcome: Outcome,
}

impl AtBat {
    pub fn new(player: &Player) -> Self {
        AtBat {
            player: player.clone(),
            strikes: 0,
            balls: 0,
            outcome: TBD,
        }
    }

    fn score_strike(&mut self) {
        self.strikes += 1;

        if self.strikes >= 3 {
            self.outcome = StrikeOut;
        }
    }

    fn score_ball(&mut self) {
        self.balls += 1;

        if self.balls >= 4 {
            self.outcome = Walk;
        }
    }

    pub fn is_over(&self) -> bool {
        self.outcome != TBD
    }

    pub fn play(&mut self) {
        println!("Current Player: {:?}", self.player);

        while !self.is_over() {
            println!("Strikes: {}", self.strikes);
            println!("Balls: {}", self.balls);
            println!("");
            println!("Awaiting input...");
            let input: String = text_io::read!();
            self.match_input(input)
        }
    }

    fn match_input(&mut self, input: String) {
        match input.trim().to_lowercase().as_str() {
            "b" => self.score_ball(),
            "s" | "k" => self.score_strike(),
            "single" | "1" | "h1" | "hs" => self.outcome = Single,
            "double" | "2" | "h2" | "hd" | "d" => self.outcome = Double,
            "triple" | "3" | "h3" | "ht" | "t" => self.outcome = Triple,
            "home" | "homerun" | "4" | "h4" | "hr" => self.outcome = HomeRun,
            "go" | "g" => self.outcome = GroundOut,
            "fo" | "f" => self.outcome = FlyOut,
            _ => (),
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_create_new() {
        let player = Player::get_sample_player();
        let at_bat = AtBat::new(&player);

        assert_eq!(at_bat.balls, 0);
        assert_eq!(at_bat.strikes, 0);
        assert_eq!(at_bat.outcome, TBD);
    }

    #[test]
    fn should_score_strike() {
        let player = Player::get_sample_player();
        let mut at_bat = AtBat::new(&player);
        at_bat.score_strike();

        assert_eq!(at_bat.strikes, 1);
        at_bat.score_strike();

        assert_eq!(at_bat.strikes, 2);
        at_bat.score_strike();

        assert_eq!(at_bat.strikes, 3);
        assert_eq!(at_bat.outcome, StrikeOut);
    }

    #[test]
    fn should_score_ball() {
        let player = Player::get_sample_player();
        let mut at_bat = AtBat::new(&player);

        at_bat.score_ball();
        assert_eq!(at_bat.balls, 1);
        at_bat.score_ball();
        assert_eq!(at_bat.balls, 2);
        at_bat.score_ball();
        assert_eq!(at_bat.balls, 3);
        at_bat.score_ball();
        assert_eq!(at_bat.balls, 4);
        assert_eq!(at_bat.outcome, Walk)
    }
}
