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

    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    let mut rng = rand::rng();
    let mut game = Game::start_new(&mut rng);

    print_game(&mut stdout, &game).unwrap();
    stdout.flush().unwrap();

    for key in stdin.keys() {
        match key.as_ref().unwrap() {
            Key::Char('q') => break,
            Key::Char(',') => game.try_avoid(&mut rng),
            _ => {}
        }

        write!(stdout, "{}", termion::clear::All).unwrap();
        print_game(&mut stdout, &game).unwrap();
        stdout.flush().unwrap();
    }
}
