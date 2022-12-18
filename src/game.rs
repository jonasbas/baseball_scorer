use crate::base::Base;
use crate::inning::InningTracker;

pub struct Gamestate {
    first_base: Base,
    second_base: Base,
    third_base: Base,
    inning: InningTracker,
}

impl Gamestate {
    pub fn new() -> Self {
        Gamestate {
            first_base: Base::new(),
            second_base: Base::new(),
            third_base: Base::new(),
            inning: InningTracker::new(),
        }
    }
}
