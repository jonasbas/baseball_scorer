use crate::at_bat::{AtBat, Outcome};
use crate::inning::{Half, Inning, InningHalf};
use crate::score::Score;
use crate::team::Team;

pub struct Gamestate {
    away: Option<Team>,
    home: Option<Team>,
    home_order: usize,
    away_order: usize,
    innings: Vec<Inning>,
    current_inning: u8,
    score: Score,
}

impl Gamestate {
    pub fn new() -> Self {
        Gamestate {
            away: None,
            home: None,
            home_order: 0,
            away_order: 0,
            innings: Vec::new(),
            current_inning: 1,
            score: Score::new(),
        }
    }

    pub fn set_home_team(&mut self, team: Team) {
        self.home = Some(team);
    }

    pub fn set_away_team(&mut self, team: Team) {
        self.away = Some(team);
    }

    pub fn start_game(&mut self) {
        self.game_loop();
    }

    fn game_loop(&mut self) {
        let mut current_inning: Inning = Inning::new();
        let mut is_top: bool = true;
        let mut half_type: Half = Half::Top;

        while !self.is_over(half_type) {
            let mut half = InningHalf::new(self.current_inning, half_type);
            println!("{half:?}");

            while !half.is_over() {
                println!("Outs: {}", half.outs);
                let mut current_player =
                    &self.away.as_ref().unwrap().batting_order[self.away_order];

                if half_type == Half::Bottom {
                    current_player = &self.home.as_ref().unwrap().batting_order[self.home_order];
                    self.home_order = (self.home_order + 1) % 9;
                } else {
                    self.away_order = (self.away_order + 1) % 9;
                }

                let mut current_at_bat = AtBat::new(current_player);
                let result = current_at_bat.play();

                half.at_bats.push(current_at_bat);
            }

            //Ende der Inning Hälfte
            if is_top {
                current_inning.set_top(half);
                is_top = !is_top;
                half_type = Half::Bottom;
            } else {
                current_inning.set_bot(half);
                is_top = !is_top;
                half_type = Half::Top;
                self.current_inning += 1;
            }
        }
    }

    fn is_over(&self, half: Half) -> bool {
        if self.current_inning >= 9 && half == Half::Bottom && !self.score.is_tied() {
            println!("Game over!");
            true
        } else {
            false
        }
        //TODO: checken ob Game vorbei ist
    }
}
