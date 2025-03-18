use std::io::{Write, stdin, stdout};

use domain::game::Game;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};
use ui::{
    navigation::*,
    print::{print_game, print_game_over, print_win},
};

extern crate termion;

mod domain {
    pub mod card;
    pub mod game;
    pub mod pile;
    pub mod room;
}

mod ui {
    pub mod navigation;
    pub mod print;
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    let mut rng = rand::rng();
    let mut game = Game::start_new(&mut rng);
    let mut selected_slot = 0_usize;

    print_game(&mut stdout, &game, selected_slot).unwrap();
    stdout.flush().unwrap();

    for key in stdin.keys() {
        match key.as_ref().unwrap() {
            Key::Char('q') => break,
            Key::Char(',') => game.try_avoid(&mut rng),
            Key::Char('0') => game.interact_slot(selected_slot),
            Key::Right => selected_slot = go_right(selected_slot),
            Key::Left => selected_slot = go_left(selected_slot),
            _ => {}
        }

        write!(stdout, "{}", termion::clear::All).unwrap();

        match game.check_game_over() {
            Some(domain::game::GameOverState::Death) => print_game_over(&mut stdout).unwrap(),
            Some(domain::game::GameOverState::Win) => print_win(&mut stdout).unwrap(),
            None => print_game(&mut stdout, &game, selected_slot).unwrap(),
        }

        stdout.flush().unwrap();
    }
}
