use crate::player::Player;

pub struct Team {
    name: String,
    pub batting_order: Vec<Player>,
    substitutes: Vec<Player>,
    pitcher: Option<Player>,
}

impl Team {
    pub fn new(name: String) -> Self {
        Team {
            name,
            batting_order: Vec::new(),
            substitutes: Vec::new(),
            pitcher: None,
        }
    }

    pub fn add_full_batting_order(&mut self, players: Vec<Player>) -> Result<(), &str> {
        if players.len() != 9 {
            return Err("Too many players in line up!");
        }
        self.batting_order = players;
        Ok(())
    }

    pub fn add_player_in_batting_order(&mut self, player: Player) {
        self.batting_order.push(player);
    }
}
