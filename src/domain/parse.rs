use std::str::FromStr;

use super::card::{Card, Rank, Suite};

#[derive(PartialEq, Debug)]
pub struct ParseRankError;

impl FromStr for Rank {
    type Err = ParseRankError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            "9" => Ok(Self::Nine),
            "10" => Ok(Self::Ten),
            "J" => Ok(Self::Jack),
            "Q" => Ok(Self::Queen),
            "K" => Ok(Self::King),
            "A" => Ok(Self::Ace),
            _ => Err(ParseRankError),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ParseSuiteError;

impl FromStr for Suite {
    type Err = ParseSuiteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "D" => Ok(Self::Diamonds),
            "C" => Ok(Self::Clubs),
            "S" => Ok(Self::Spades),
            "H" => Ok(Self::Hearts),
            _ => Err(ParseSuiteError),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ParseCardError {
    Suite(ParseSuiteError),
    Rank(ParseRankError),
}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rank_s, suite_s) = s.split_at(s.len() - 1);
        let rank = rank_s.parse().map_err(ParseCardError::Rank)?;
        let suite = suite_s.parse().map_err(ParseCardError::Suite)?;
        Ok(Card { suite, rank })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("5D", Card {rank: Rank::Five, suite: Suite::Diamonds})]
    #[case("2C", Card {rank: Rank::Two, suite: Suite::Clubs})]
    #[case("10S", Card {rank: Rank::Ten, suite: Suite::Spades})]
    #[case("QH", Card {rank: Rank::Queen, suite: Suite::Hearts})]
    fn should_parse_card(#[case] s: &str, #[case] expected: Card) {
        let actual: Card = s.parse().unwrap();
        assert_eq!(expected, actual)
    }
}
