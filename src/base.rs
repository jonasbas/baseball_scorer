use crate::player::Player;

#[derive(Debug, Clone)]
pub struct Base {
    occupied: Option<Player>,
}

impl Base {
    pub fn new() -> Self {
        Base { occupied: None }
    }

    pub fn set_player(&mut self, player: &Player) {
        self.occupied = Some(player.clone());
    }

    pub fn is_empty(&self) -> bool {
        self.occupied.is_none()
    }

    pub fn get_player(&self) -> &Player {
        self.occupied.as_ref().expect("Base is empty!")
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
}
