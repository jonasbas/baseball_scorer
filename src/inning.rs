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
    at_bats: Vec<AtBat>,
    bases: Vec<Base>,
    pub outs: u8,
}

impl InningHalf {
    pub fn new(number: u8, half: Half) -> Self {
        InningHalf {
            number,
            half,
            at_bats: Vec::new(),
            bases: vec![Base::new(), Base::new(), Base::new()],
            outs: 0,
        }
    }

    pub fn is_over(&self) -> bool {
        self.outs >= 3
    }

    pub fn start_new_at_bat(&mut self, player: &Player, score: &Score) -> Result<(), &str> {
        if self.is_over() {
            return Err("inning half is over");
        }

        let current_at_bat = AtBat::new(player);

        Ok(())
    }
}
