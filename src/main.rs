use base::Base;
use text_io::read;

mod base;
mod game;
mod inning;
mod player;

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
