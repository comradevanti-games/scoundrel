use std::fmt::{self};
use std::io::Write;

use crate::domain::card::{Rank, Suite};
use crate::domain::{card::Card, game::Game, pile::Pile};
extern crate termion;

impl fmt::Display for Suite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char = match self {
            Suite::Hearts => 'H',
            Suite::Clubs => 'C',
            Suite::Spades => 'S',
            Suite::Diamonds => 'D',
        };
        write!(f, "{}", char)
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suite)
    }
}

fn print_pile<W: Write>(stdout: &mut W, x: u16, y: u16, pile: &Pile) {
    write!(
        stdout,
        "{}{}",
        termion::cursor::Goto(x, y),
        pile.0.first().unwrap()
    )
    .unwrap();
}

pub fn print_game<W: Write>(stdout: &mut W, game: &Game) {
    print_pile(stdout, 2, 2, &game.dungeon);
}
