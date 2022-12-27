use crate::at_bat::Outcome::*;
use crate::player::Player;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Outcome {
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
    pub outcome: Option<Outcome>,
}

impl AtBat {
    pub fn new(player: &Player) -> Self {
        AtBat {
            player: player.clone(),
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

    pub fn is_over(&self) -> bool {
        if self.outcome.is_some() {
            return true;
        }
        false
    }

    pub fn play(&mut self) -> Result<(), &str> {
        println!("Current Player: {:?}", self.player);

        while !self.is_over() {
            println!("Strikes: {}", self.strikes);
            println!("Balls: {}", self.balls);
            println!("");
            println!("Awaiting input...");
            let input: String = text_io::read!();
            self.match_input(input)
        }

        Ok(())
    }

    fn match_input(&mut self, input: String) {
        match input.trim().to_lowercase().as_str() {
            "b" => self.score_ball(),
            "s" | "k" => self.score_strike(),
            "single" | "1" | "h1" | "hs" => self.outcome = Some(Single),
            "double" | "2" | "h2" | "hd" | "d" => self.outcome = Some(Double),
            "triple" | "3" | "h3" | "ht" | "t" => self.outcome = Some(Triple),
            "home" | "homerun" | "4" | "h4" | "hr" => self.outcome = Some(HomeRun),
            "go" | "g" => self.outcome = Some(GroundOut),
            "fo" | "f" => self.outcome = Some(FlyOut),
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
        assert_eq!(at_bat.outcome, None);
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
        assert_eq!(at_bat.outcome.unwrap(), StrikeOut);
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
        assert_eq!(at_bat.outcome.unwrap(), Walk)
    }
}
