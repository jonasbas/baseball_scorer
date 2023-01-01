pub struct Score {
    runs_home_team: u8,
    runs_away_team: u8,
}

impl Score {
    pub fn new() -> Self {
        Score {
            runs_home_team: 0,
            runs_away_team: 0,
        }
    }

    pub fn score_runs_home_team(&mut self, runs: usize) {
        self.runs_home_team += 1;
    }

    pub fn score_runs_away_team(&mut self, runs: usize) {
        self.runs_away_team += 1;
    }

    pub fn is_tied(&self) -> bool {
        self.runs_home_team == self.runs_away_team
    }
}
