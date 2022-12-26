use game::Gamestate;
use player::Player;
use team::Team;
use text_io::read;

mod at_bat;
mod base;
mod game;
mod inning;
mod player;
mod score;
mod team;

fn main() {
    let mut home_team = Team::new(String::from("Heimat"));
    home_team.add_player_in_batting_order(&Player::get_sample_player());

    let mut away_team = Team::new(String::from("Weg"));
    away_team.add_player_in_batting_order(&Player::get_sample_player());

    let mut game = Gamestate::new();
    game.set_home_team(home_team);
    game.set_away_team(away_team);

    game.start_game();
}
