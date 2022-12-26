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
            _ => (),
        };
    }
}
