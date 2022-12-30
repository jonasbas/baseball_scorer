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

    pub fn move_bases(&mut self) {
        //TODO: Actaully move the players
        if !self.third_base.is_empty() {
            println!(
                "Player {} is on third base. What happens?",
                self.third_base.get_player()
            );
        }

        if !self.second_base.is_empty() {
            println!(
                "Player {} is on second base. What happens?",
                self.second_base.get_player()
            );
        }

        if !self.first_base.is_empty() {
            println!(
                "Player {} is on first base. What happens?",
                self.first_base.get_player()
            );
        }
    }
}
