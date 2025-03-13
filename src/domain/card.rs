use std::fmt::{self};

#[derive(PartialEq, Debug, Clone)]
pub enum Suite {
    Hearts,
    Clubs,
    Spades,
    Diamonds,
}

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

#[derive(PartialEq, Debug, Clone)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
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

#[derive(PartialEq, Debug, Clone)]
pub struct Card {
    pub suite: Suite,
    pub rank: Rank,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suite)
    }
}

impl Rank {
    pub fn of(self, suite: Suite) -> Card {
        return Card { suite, rank: self };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_card() {
        let actual = Rank::Two.of(Suite::Hearts);
        let expected = Card {
            suite: Suite::Hearts,
            rank: Rank::Two,
        };
        assert_eq!(expected, actual)
    }
}
