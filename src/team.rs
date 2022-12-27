use crate::player::Player;

#[derive(Debug)]
pub enum Location {
    Home,
    Away,
}

#[derive(Debug)]
pub struct Team {
    name: String,
    pub batting_order: Vec<Player>,
    substitutes: Vec<Player>,
    pitcher: Option<Player>,
    location: Location,
}

impl Team {
    pub fn new(name: String, location: Location) -> Self {
        Team {
            name,
            batting_order: Vec::new(),
            substitutes: Vec::new(),
            pitcher: None,
            location,
        }
    }

    pub fn add_player_in_batting_order(&mut self, player: &Player) {
        self.batting_order.push(player.clone());
    }

    pub fn create_team_with_players(location: Location) -> Self {
        println!("Team creation started...");
        println!("Enter team name:");
        let name: String = text_io::read!();
        let mut team = Team::new(name, location);
        for index in 1..=9 {
            println!("Player in line up spot {index}");
            println!("First name:");
            let first_name: String = text_io::read!();

            println!("Last name:");
            let last_name: String = text_io::read!();

            let player = Player::new(first_name, last_name, crate::player::Position::Catcher);
            //TODO: Postion implementieren

            println!("Player {:?} created!", player);
            team.add_player_in_batting_order(&player);
        }

        team
    }

    pub fn create_full_sample_team(location: Location) -> Self {
        println!("Team creation started...");
        println!("Enter team name:");
        let name: String = text_io::read!();
        let mut team = Team::new(name, location);

        for _ in 1..=9 {
            team.add_player_in_batting_order(&Player::get_sample_player());
        }

        println!("{team:?} created");
        team
    }
}
