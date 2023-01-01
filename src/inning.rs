use crate::{at_bat::AtBat, base::Base, player::Player, score::Score};

#[derive(Debug)]
pub struct Inning {
    top: Option<InningHalf>,
    bot: Option<InningHalf>,
    pub is_top: bool,
}

impl Inning {
    pub fn new() -> Self {
        Inning {
            top: None,
            bot: None,
            is_top: true,
        }
    }

    pub fn set_top(&mut self, half: InningHalf) {
        self.top = Some(half);
    }

    pub fn set_bot(&mut self, half: InningHalf) {
        self.bot = Some(half);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Half {
    Top,
    Bottom,
}

#[derive(Debug, Clone)]
pub struct InningHalf {
    number: u8,
    half: Half,
    pub at_bats: Vec<AtBat>,
    first_base: Base,
    second_base: Base,
    third_base: Base,
    pub outs: u8,
}

impl InningHalf {
    pub fn new(number: u8, half: Half) -> Self {
        InningHalf {
            number,
            half,
            at_bats: Vec::new(),
            first_base: Base::new(),
            second_base: Base::new(),
            third_base: Base::new(),
            outs: 0,
        }
    }

    pub fn is_over(&self) -> bool {
        self.outs >= 3
    }

    pub fn move_bases(&mut self) -> usize {
        let mut scoring_runs = 0;
        if !self.third_base.is_empty() {
            println!(
                "Player {} is on third base. What happens?",
                self.third_base.get_player()
            );
            scoring_runs += self.move_third_base();
        }

        if !self.second_base.is_empty() {
            println!(
                "Player {} is on second base. What happens?",
                self.second_base.get_player()
            );
            scoring_runs += self.move_second_base();
        }

        if !self.first_base.is_empty() {
            println!(
                "Player {} is on first base. What happens?",
                self.first_base.get_player()
            );
            scoring_runs += self.move_first_base();
        }

        scoring_runs
    }

    //TODO: Refactor into one function if possible
    pub fn resolve_single(&mut self, player: &Player) -> usize {
        let mut scoring_runs = 0;
        let first_base_backup = self.first_base.clone();
        let second_base_backup = self.second_base.clone();
        let third_base_backup = self.third_base.clone();

        scoring_runs += self.move_bases();
        if !self.first_base.is_empty() {
            println!("First base has to be empty for a player to score a single!");
            self.first_base = first_base_backup;
            self.second_base = second_base_backup;
            self.third_base = third_base_backup;
            //TODO: REDO
        } else {
            self.first_base = Base::new();
            self.first_base.reach_base(player);
        }

        scoring_runs
    }

    pub fn resolve_double(&mut self, player: &Player) -> usize {
        let mut scoring_runs = 0;
        let first_base_backup = self.first_base.clone();
        let second_base_backup = self.second_base.clone();
        let third_base_backup = self.third_base.clone();

        scoring_runs += self.move_bases();
        if !self.second_base.is_empty() && !self.first_base.is_empty() {
            println!("First and second base need to be empty for a player to score a double!");
            self.first_base = first_base_backup;
            self.second_base = second_base_backup;
            self.third_base = third_base_backup;
            //TODO: REDO
        } else {
            self.first_base = Base::new();
            self.first_base.reach_base(player);
        }

        scoring_runs
    }

    pub fn resolve_triple(&mut self, player: &Player) -> usize {
        let mut scoring_runs = 0;
        let first_base_backup = self.first_base.clone();
        let second_base_backup = self.second_base.clone();
        let third_base_backup = self.third_base.clone();

        scoring_runs += self.move_bases();
        if !self.third_base.is_empty()
            && !self.second_base.is_empty()
            && !self.first_base.is_empty()
        {
            println!(
                "First, second and third base need to be empty for a player to score a double!"
            );
            self.first_base = first_base_backup;
            self.second_base = second_base_backup;
            self.third_base = third_base_backup;
            //TODO: REDO
        } else {
            self.first_base = Base::new();
            self.first_base.reach_base(player);
        }

        scoring_runs
    }

    fn move_third_base(&mut self) -> usize {
        let input: String = text_io::read!();
        let mut runs_scored = 0;

        match input.to_lowercase().as_str() {
            "home" | "run" | "scored" => {
                runs_scored += 1;
                self.third_base = Base::new();
            }
            "thrown" | "out" | "o" => {
                self.third_base = Base::new();
            }
            "nothing" | "stay" => (),
            _ => (), //TODO: repeat match
        }

        runs_scored
    }

    fn move_second_base(&mut self) -> usize {
        let input: String = text_io::read!();
        let mut runs_scored = 0;

        match input.to_lowercase().as_str() {
            "home" | "run" | "scored" => {
                if self.third_base.is_empty() {
                    runs_scored += 1;
                    self.second_base = Base::new();
                }
            }
            "third" | "3" => {
                if self.third_base.is_empty() {
                    self.third_base = self.second_base.clone();
                    self.second_base = Base::new();
                }
            }
            "thrown" | "out" | "o" => {
                self.second_base = Base::new();
            }
            "nothing" | "stay" => (),
            _ => (), //TODO: repeat match
        }

        runs_scored
    }

    fn move_first_base(&mut self) -> usize {
        let input: String = text_io::read!();
        let mut runs_scored = 0;

        match input.to_lowercase().as_str() {
            "home" | "run" | "scored" => {
                if self.third_base.is_empty() && self.second_base.is_empty() {
                    runs_scored += 1;
                    self.second_base = Base::new();
                }
            }
            "third" | "3" => {
                if self.third_base.is_empty() && self.second_base.is_empty() {
                    self.third_base = self.second_base.clone();
                    self.second_base = Base::new();
                }
            }
            "second" | "2" => {
                if self.second_base.is_empty() {
                    self.second_base = self.first_base.clone();
                    self.first_base = Base::new();
                }
            }
            "thrown" | "out" | "o" => {
                self.second_base = Base::new();
            }
            "nothing" | "stay" => (),
            _ => (), //TODO: repeat match
        }

        runs_scored
    }
}
