use super::card::Card;

#[derive(PartialEq, Debug)]
pub struct Pile(pub Vec<Card>);

impl Pile {
    pub fn peek_top_card(&self) -> Option<&Card> {
        return self.0.first();
    }
}
