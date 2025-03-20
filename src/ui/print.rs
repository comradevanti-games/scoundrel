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

fn print_empty_slot<W: Write>(f: &mut W, x: u16, y: u16) -> io::Result<()> {
    print_empty_slot_with_content(f, x, y, &"   ".to_string())
}

fn print_card_with_content<W: Write>(f: &mut W, x: u16, y: u16, content: &str) -> io::Result<()> {
    write!(f, "{}┌───┐", cursor::Goto(x, y))?;
    write!(f, "{}│    ", cursor::Goto(x, y + 1))?;
    write!(f, "{}{}", cursor::Goto(x + 1, y + 1), content)?;
    write!(f, "{}│", cursor::Goto(x + 4, y + 1))?;
    write!(f, "{}│   │", cursor::Goto(x, y + 2))?;
    write!(f, "{}└───┘", cursor::Goto(x, y + 3))
}

fn print_empty_card<W: Write>(f: &mut W, x: u16, y: u16) -> io::Result<()> {
    print_card_with_content(f, x, y, "")
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

fn print_maybe_card<W: Write>(f: &mut W, x: u16, y: u16, card: Option<&Card>) -> io::Result<()> {
    match card {
        Some(it) => print_card(f, x, y, it),
        None => print_empty_slot(f, x, y),
    }
}

static CONTROLS: &str = "\
Controls:\n\r\
Navigate: ← →\n\r\
Avoid room: ,\n\r\
Interact with card: 0\n\r\
Quit: q";

static HOW_TO_PLAY: &str = "\
How to play: http://stfj.net/art/2011/Scoundrel.pdf
";

pub fn print_game<W: Write>(f: &mut W, game: &Game, selected_slot: usize) -> io::Result<()> {
    print_pile(f, 2, 2, game.dungeon.count_cards())?;

    for (i, card) in game.room.iter().enumerate() {
        let x = (9 + 5 * i) as u16;

        let is_selected = selected_slot == i;
        let y = if is_selected && card.is_some() { 1 } else { 2 };

        print_maybe_card(f, x, y, card.as_ref())?;

        if is_selected {
            write!(f, "{}▀▀▀▀▀", cursor::Goto(x, 6))?;
        }
    }

    print_pile(f, 32, 2, game.discard_count)?;

    print_maybe_card(f, 2, 7, game.equipped.as_ref())?;

    for (i, card) in game.slain.iter().enumerate() {
        let x = (9 + i) as u16;
        print_card(f, x, 7, card)?;
    }

    write!(f, "{}Health: {}", cursor::Goto(2, 12), &game.health)?;
    write!(f, "{}{}", cursor::Goto(0, 20), CONTROLS)?;
    write!(f, "{}{}", cursor::Goto(0, 26), HOW_TO_PLAY)
}

static GAME_OVER: &str = "GAME OVER";

pub fn print_game_over<W: Write>(f: &mut W) -> io::Result<()> {
    write!(f, "{}{}", cursor::Goto(5, 5), GAME_OVER)
}

static WIN: &str = "YOU WIN";

pub fn print_win<W: Write>(f: &mut W) -> io::Result<()> {
    write!(f, "{}{}", cursor::Goto(5, 5), WIN)
}
