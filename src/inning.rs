use crate::at_bat::AtBat;

pub struct Inning {
    top: InningHalf,
    bot: InningHalf,
    is_top: bool,
}

pub struct InningHalf {
    number: u8,
    at_bats: Vec<AtBat>,
    outs: u8,
}

impl InningHalf {
    pub fn new(number: u8) -> Self {
        InningHalf {
            number,
            at_bats: Vec::new(),
            outs: 0,
        }
    }
}
