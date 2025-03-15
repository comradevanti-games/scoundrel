use std::fs::write;
use std::io::{self, Write};

use termion::color::{self};
use termion::{cursor, style};

use crate::domain::card::{Rank, Suite};
use crate::domain::{card::Card, game::Game, pile::Pile};
extern crate termion;

const HEART: char = '♥';

const CLUBS: char = '♣';

const SPADE: char = '♠';

const DIAMOND: char = '◆';

fn print_suite<W: Write>(f: &mut W, x: u16, y: u16, suite: &Suite) -> io::Result<()> {
    write!(f, "{}", cursor::Goto(x, y))?;

    match suite {
        Suite::Hearts => write!(f, "{}{}", color::Fg(color::Red), HEART),
        Suite::Clubs => write!(f, "{}{}", color::Fg(color::Blue), CLUBS),
        Suite::Spades => write!(f, "{}{}", color::Fg(color::LightBlack), SPADE),
        Suite::Diamonds => write!(f, "{}{}", color::Fg(color::Yellow), DIAMOND),
    }?;

    write!(f, "{}", style::Reset)
}

fn print_rank<W: Write>(f: &mut W, x: u16, y: u16, rank: &Rank) -> io::Result<()> {
    write!(f, "{}", cursor::Goto(x, y))?;

    let s = match rank {
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

fn print_card<W: Write>(f: &mut W, x: u16, y: u16, content: &str) -> io::Result<()> {
    assert!(content.len() <= 2);

    write!(f, "{}┌──┐", cursor::Goto(x, y))?;
    write!(f, "{}│{: >2}│", cursor::Goto(x, y + 1), content)?;
    write!(f, "{}└──┘", cursor::Goto(x, y + 2))
}

fn print_pile<W: Write>(f: &mut W, x: u16, y: u16, pile: &Pile) -> io::Result<()> {
    let count = pile.count_cards();
    print_card(f, x, y, &count.to_string())
}

pub fn print_game<W: Write>(f: &mut W, game: &Game) -> io::Result<()> {
    print_pile(f, 2, 2, &game.dungeon)?;
    write!(f, "{}Health: {}", cursor::Goto(2, 10), &game.health)
}
