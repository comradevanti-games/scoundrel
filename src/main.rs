use std::io::stdout;

use domain::game::Game;
use termion::raw::IntoRawMode;
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

    let mut rng = rand::rng();
    let game = Game::start_new(&mut rng);

    print_game(&mut stdout, &game);
}
