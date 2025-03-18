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
