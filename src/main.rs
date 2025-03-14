use std::io::{Write, stdin, stdout};

use domain::game::Game;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};
use ui::print::print_game;

extern crate termion;

mod domain {
    pub mod card;
    pub mod game;
    pub mod pile;
}

mod ui {
    pub mod print;
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    let mut rng = rand::rng();
    let game = Game::start_new(&mut rng);

    print_game(&mut stdout, &game);
    stdout.flush().unwrap();

    for key in stdin.keys() {
        match key.as_ref().unwrap() {
            Key::Char('q') => break,
            _ => {}
        }
    }
}
