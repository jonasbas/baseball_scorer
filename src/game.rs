use crate::inning::Inning;
use crate::score::Score;
use crate::team::Team;

pub struct Gamestate {
    away: Option<(Team, u8)>,
    home: Option<(Team, u8)>,
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
        self.home = Some((team, 0));
        Ok(())
    }

    pub fn set_away_team(&mut self, team: Team) -> Result<(), &str> {
        self.away = Some((team, 0));
        Ok(())
    }

    fn new_inning(&mut self) -> Result<(), &str> {
        let new_inning_number: u8 = self.innings.len() as u8 + 1;
        let tmp = Inning::new(new_inning_number);
        self.current_inning = Some(tmp);
        Ok(())
    }

    // pub fn start_game(&mut self) -> Result<(), &str> {
    //     if self.away.is_none() || self.home.is_none() {
    //         return Err("both teams have to be set!");
    //     }
    //     self.new_inning()?;
    //
    //     let current_player = &self.away.unwrap().0.batting_order[0];
    //     self.current_inning
    //         .unwrap()
    //         .top
    //         .start_new_at_bat(current_player, &self.score);
    //
    //     Ok(())
    // }
}
