use std::collections::VecDeque;

use rand::{Rng, seq::SliceRandom};

use super::card::Card;

#[derive(PartialEq, Debug)]
pub struct Pile(VecDeque<Card>);

impl Pile {
    pub fn from(cards: Vec<Card>) -> Self {
        Pile(VecDeque::from(cards))
    }

    pub fn pop_top_card(&mut self) -> Option<Card> {
        self.0.pop_back()
    }

    pub fn add_to_bottom(&mut self, cards: Vec<Card>) {
        for card in cards {
            self.0.push_front(card);
        }
    }

    pub fn count_cards(&self) -> u8 {
        self.0.len() as u8
    }

    pub fn shuffle<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.0.make_contiguous().shuffle(rng);
    }
}
