use std::io::{self, Write};

use termion::color::{self};
use termion::{cursor, style};

use crate::domain::card::{Rank, Suite};
use crate::domain::{card::Card, game::Game};
extern crate termion;

const HEART: char = '♥';

const CLUBS: char = '♣';

const SPADE: char = '♠';

const DIAMOND: char = '◆';

fn make_suite_text(suite: &Suite) -> String {
    let s = match suite {
        Suite::Hearts => format!("{}{}", color::Fg(color::Red), HEART),
        Suite::Clubs => format!("{}{}", color::Fg(color::Blue), CLUBS),
        Suite::Spades => format!("{}{}", color::Fg(color::LightBlack), SPADE),
        Suite::Diamonds => format!("{}{}", color::Fg(color::Yellow), DIAMOND),
    };

    format!("{}{}", s, style::Reset)
}

fn make_rank_text(rank: &Rank) -> &str {
    match rank {
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
    }
}

fn make_card_text(card: &Card) -> String {
    format!(
        "{}{}",
        make_rank_text(&card.rank),
        make_suite_text(&card.suite)
    )
}

fn print_empty_slot_with_content<W: Write>(
    f: &mut W,
    x: u16,
    y: u16,
    content: &String,
) -> io::Result<()> {
    write!(f, "{}┌   ┐", cursor::Goto(x, y))?;
    write!(f, "{} {}", cursor::Goto(x, y + 1), content)?;
    write!(f, "{}└   ┘", cursor::Goto(x, y + 3))
}

fn print_card_with_content<W: Write>(
    f: &mut W,
    x: u16,
    y: u16,
    content: &String,
) -> io::Result<()> {
    write!(f, "{}┌───┐", cursor::Goto(x, y))?;
    write!(f, "{}│{}", cursor::Goto(x, y + 1), content)?;
    write!(f, "{}│", cursor::Goto(x + 4, y + 1))?;
    write!(f, "{}│   │", cursor::Goto(x, y + 2))?;
    write!(f, "{}└───┘", cursor::Goto(x, y + 3))
}

fn print_empty_card<W: Write>(f: &mut W, x: u16, y: u16) -> io::Result<()> {
    print_card_with_content(f, x, y, &"".to_string())
}

fn print_card<W: Write>(f: &mut W, x: u16, y: u16, card: &Card) -> io::Result<()> {
    print_card_with_content(f, x, y, &make_card_text(card))
}

fn print_pile<W: Write>(f: &mut W, x: u16, y: u16, count: u8) -> io::Result<()> {
    if count > 1 {
        print_empty_card(f, x + 1, y)?;
    }

    let content = count.to_string();
    if count > 0 {
        print_card_with_content(f, x, y, &content)
    } else {
        print_empty_slot_with_content(f, x, y, &content)
    }
}

pub fn print_game<W: Write>(f: &mut W, game: &Game) -> io::Result<()> {
    print_pile(f, 2, 2, game.dungeon.count_cards())?;

    for (i, card) in game.room.iter().enumerate() {
        let x = (9 + 5 * i) as u16;
        card.as_ref()
            .map(|it| print_card(f, x, 2, it))
            .unwrap_or(Ok(()))?;
    }

    print_pile(f, 32, 2, game.discard_count)?;

    write!(f, "{}Health: {}", cursor::Goto(2, 10), &game.health)
}
