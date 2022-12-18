use base::Base;
use text_io::read;

mod at_bat;
mod base;
mod game;
mod inning;
mod player;
mod team;

fn main() {
    let test_base = Base::new();
    loop {
        println!("Input bitte");
        let test: String = read!();
        println!("{}", test);
    }
}

fn input_stuff_main_loop() {
    println!("Tada");
}
