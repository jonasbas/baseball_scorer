#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    first_name: String,
    last_name: String,
    positon: Position,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Position {
    Catcher,
    FirstBase,
    SecondBase,
    ThirdBase,
    ShortStop,
    LeftField,
    CenterField,
    RightField,
    Pitcher,
    DesignatedHitter,
}

impl Player {
    pub fn new(first_name: String, last_name: String, positon: Position) -> Self {
        Player {
            first_name,
            last_name,
            positon,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::Position::*;

    #[test]
    fn new_test() {
        let first_name = String::from("Timo");
        let last_name = String::from("Werner");
        let player = Player::new(first_name, last_name, Catcher);

        assert_eq!(player.first_name, "Timo", "Player name should be Timo!");
        assert_eq!(player.last_name, "Werner", "Player name should be Werner!");
        assert_eq!(player.positon, Catcher, "Player position should be Catcher");
    }
}
