#[derive(Debug, PartialEq)]
pub struct Player {
    first_name: String,
    last_name: String,
}

impl Player {
    pub fn new(first_name: String, last_name: String) -> Self {
        Player {
            first_name,
            last_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Player;

    #[test]
    fn new_test() {
        let first_name = String::from("Timo");
        let last_name = String::from("Werner");
        let player = Player::new(first_name, last_name);

        assert_eq!(player.first_name, "Timo");
        assert_eq!(player.last_name, "Werner");
    }
}
