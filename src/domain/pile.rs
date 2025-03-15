use std::collections::VecDeque;

use rand::{Rng, seq::SliceRandom};

use super::card::Card;

#[derive(PartialEq, Debug)]
pub struct Pile(pub VecDeque<Card>);

impl Pile {
    pub fn peek_top_card(&self) -> Option<&Card> {
        self.0.back()
    }

    pub fn pop_top_card(&mut self) -> Option<Card> {
        self.0.pop_back()
    }

    pub fn count_cards(&self) -> u8 {
        self.0.len() as u8
    }

    pub fn shuffle<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.0.make_contiguous().shuffle(rng);
    }
}
