use game::Gamestate;
use team::Team;

mod at_bat;
mod base;
mod game;
mod inning;
mod player;
mod score;
mod team;

fn main() {
    let home_team = Team::create_full_sample_team(team::Location::Home);

    let away_team = Team::create_full_sample_team(team::Location::Away);

    let mut game = Gamestate::new();
    game.set_home_team(home_team);
    game.set_away_team(away_team);

    game.start_game();
}
