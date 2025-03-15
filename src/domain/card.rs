#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Suite {
    Hearts,
    Clubs,
    Spades,
    Diamonds,
}

#[derive(PartialEq, Debug, Clone, Copy)]
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

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Card {
    pub suite: Suite,
    pub rank: Rank,
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
