use crate::base::Base;
use crate::inning::Inning;

pub struct Gamestate {
    first_base: Base,
    second_base: Base,
    third_base: Base,
    inning: Vec<Inning>,
}

impl Gamestate {
    pub fn new() -> Self {
        Gamestate {
            first_base: Base::new(),
            second_base: Base::new(),
            third_base: Base::new(),
            inning: Vec::new(),
        }
    }
}
