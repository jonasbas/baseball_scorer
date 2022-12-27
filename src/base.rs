use crate::player::Player;

#[derive(Clone)]
pub struct Base {
    occupied: Option<Player>,
}

impl Base {
    pub fn new() -> Self {
        Base { occupied: None }
    }

    pub fn reach_base(&mut self, player: Player) {
        if self.occupied != None {
            panic!("base is not empty"); //TODO: no panic please
        }
        self.occupied = Some(player);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let new_base = Base::new();
        assert_eq!(new_base.occupied, None, "New Base is not empty!");
    }

    #[test]
    fn reach_base() {}
}
