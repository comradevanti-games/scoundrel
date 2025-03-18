use super::{card::Card, pile::Pile};

#[derive(PartialEq, Debug)]
pub struct Room {
    slots: [Option<Card>; 4],
}

impl Room {
    pub fn empty() -> Self {
        Room {
            slots: [None, None, None, None],
        }
    }

    pub fn populate_from(&mut self, pile: &mut Pile) {
        for slot in 0..4 {
            if self.slots[slot].is_some() {
                continue;
            }
            self.slots[slot] = pile.pop_top_card()
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Option<Card>> {
        self.slots.iter()
    }

    pub fn clear(&mut self) -> Vec<Card> {
        let room: Vec<Card> = self
            .slots
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default();
        self.slots.fill(None);
        room
    }

    pub fn count_occupied_slots(&self) -> usize {
        self.slots.iter().filter(|it| it.is_some()).count()
    }

    pub fn is_full(&self) -> bool {
        self.count_occupied_slots() == 4
    }

    pub fn take_card(&mut self, slot: usize) -> Option<Card> {
        let card = self.slots[slot];
        self.slots[slot] = None;
        card
    }
}
