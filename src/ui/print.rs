use std::io::Write;

use crate::domain::{card::Card, game::Game, pile::Pile};
extern crate termion;

fn print_card<W: Write>(stdout: &mut W, x: u16, y: u16, card: &Card) {
    write!(stdout, "{}{}", termion::cursor::Goto(x, y), card).unwrap();
}

fn print_pile<W: Write>(stdout: &mut W, x: u16, y: u16, pile: &Pile) {
    print_card(stdout, x, y, pile.first().unwrap());
}

pub fn print_game<W: Write>(stdout: &mut W, game: &Game) {
    print_pile(stdout, 2, 2, &game.dungeon);
}
