use crate::{at_bat::AtBat, base::Base, player::Player, score::Score};

pub struct Inning {
    pub top: InningHalf,
    pub bot: InningHalf,
    is_top: bool,
}

pub enum Half {
    Top,
    Bottom,
}

pub struct InningHalf {
    number: u8,
    half: Half,
    at_bats: Vec<AtBat>,
    bases: Vec<Base>,
    outs: u8,
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

    fn is_over(&self) -> bool {
        self.outs == 3
    }

    pub fn start_new_at_bat(&mut self, player: &Player, score: &Score) -> Result<(), &str> {
        if self.is_over() {
            return Err("inning half is over");
        }

        let current_at_bat = AtBat::new(player);

        Ok(())
    }
}

impl Inning {
    pub fn new(number: u8) -> Self {
        Inning {
            top: InningHalf::new(number, Half::Top),
            bot: InningHalf::new(number, Half::Bottom),
            is_top: true,
        }
    }
}
