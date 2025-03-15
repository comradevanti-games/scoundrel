use super::card::Card;

#[derive(PartialEq, Debug)]
pub struct Pile(pub Vec<Card>);

impl Pile {
    pub fn peek_top_card(&self) -> Option<&Card> {
        return self.0.first();
    }

    pub fn pop_top_card(&mut self) -> Option<Card> {
        self.0.pop()
    }

    pub fn count_cards(&self) -> u8 {
        self.0.len() as u8
    }
}
