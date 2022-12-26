use crate::at_bat::AtBat;
use crate::inning::Inning;
use crate::score::Score;
use crate::team::Team;

pub struct Gamestate {
    away: Option<Team>,
    home: Option<Team>,
    innings: Vec<Inning>,
    current_inning: Option<Inning>,
    score: Score,
}

impl Gamestate {
    pub fn new() -> Self {
        Gamestate {
            away: None,
            home: None,
            innings: Vec::new(),
            current_inning: None,
            score: Score::new(),
        }
    }

    pub fn set_home_team(&mut self, team: Team) -> Result<(), &str> {
        self.home = Some(team);
        Ok(())
    }

    pub fn set_away_team(&mut self, team: Team) -> Result<(), &str> {
        self.away = Some(team);
        Ok(())
    }

    fn new_inning(&mut self) -> Result<(), &str> {
        let new_inning_number: u8 = self.innings.len() as u8 + 1;
        let tmp = Inning::new(new_inning_number);
        self.current_inning = Some(tmp);
        Ok(())
    }

    pub fn start_game(&mut self) -> Result<(), &str> {
        if self.away.is_none() || self.home.is_none() {
            return Err("both teams have to be set!");
        }
        if self.new_inning().is_err() {
            return Err("");
        }
        let this_score = Score::new();

        let current_player = self.away.as_ref().unwrap().batting_order.first().unwrap();

        let current_inning = self.current_inning.as_ref().unwrap();

        let mut current_at_bat = AtBat::new(&current_player);

        let result = current_at_bat.play();

        Ok(())
    }
}
