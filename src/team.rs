use crate::player::Player;

pub struct Team {
    name: String,
    batting_order: Vec<Player>,
    substitutes: Vec<Player>,
    pitcher: Option<Player>,
    current_batting_spot: u8,
}

impl Team {
    pub fn new(name: String) -> Self {
        Team {
            name,
            batting_order: Vec::new(),
            substitutes: Vec::new(),
            pitcher: None,
            current_batting_spot: 1,
        }
    }

    pub fn add_player_in_batting_order(&mut self, player: Player) {
        self.batting_order.push(player);
    }
}
