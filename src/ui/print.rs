use std::fmt::{self};

use termion::color::{self};
use termion::{cursor, style};

use crate::domain::card::{Rank, Suite};
use crate::domain::{card::Card, game::Game, pile::Pile};
extern crate termion;

const HEART: char = '♥';

const CLUBS: char = '♣';

const SPADE: char = '♠';

const DIAMOND: char = '◆';

impl fmt::Display for Suite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suite::Hearts => write!(f, "{}{}", color::Fg(color::Red), HEART),
            Suite::Clubs => write!(f, "{}{}", color::Fg(color::Blue), CLUBS),
            Suite::Spades => write!(f, "{}{}", color::Fg(color::LightBlack), SPADE),
            Suite::Diamonds => write!(f, "{}{}", color::Fg(color::Yellow), DIAMOND),
        }
        .and(write!(f, "{}", style::Reset))
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

impl fmt::Display for Pile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.peek_top_card().unwrap())
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", cursor::Goto(2, 2), &self.dungeon)?;
        write!(f, "{}Health: {}", cursor::Goto(2, 10), &self.health)
    }
}
