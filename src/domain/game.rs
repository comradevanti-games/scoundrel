use std::cmp;

use lazy_static::lazy_static;
use rand::{Rng, seq::SliceRandom};

use super::{
    card::{Card, Rank, Suite},
    pile::Pile,
};

#[derive(PartialEq, Debug)]
pub struct Game {
    pub dungeon: Pile,
    pub slain: Vec<Card>,
    pub room: [Option<Card>; 4],
    pub health: u8,
    pub discard_count: u8,
    pub equipped: Option<Card>,
    pub already_avoided: bool,
    already_healed: bool,
}

lazy_static! {
    static ref INITIAL_DUNGEON: Vec<Card> = vec![
        Rank::Ace.of(super::card::Suite::Clubs),
        Rank::Two.of(super::card::Suite::Clubs),
        Rank::Three.of(super::card::Suite::Clubs),
        Rank::Four.of(super::card::Suite::Clubs),
        Rank::Five.of(super::card::Suite::Clubs),
        Rank::Six.of(super::card::Suite::Clubs),
        Rank::Seven.of(super::card::Suite::Clubs),
        Rank::Eight.of(super::card::Suite::Clubs),
        Rank::Nine.of(super::card::Suite::Clubs),
        Rank::Ten.of(super::card::Suite::Clubs),
        Rank::Jack.of(super::card::Suite::Clubs),
        Rank::Queen.of(super::card::Suite::Clubs),
        Rank::King.of(super::card::Suite::Clubs),
        Rank::Two.of(super::card::Suite::Diamonds),
        Rank::Three.of(super::card::Suite::Diamonds),
        Rank::Four.of(super::card::Suite::Diamonds),
        Rank::Five.of(super::card::Suite::Diamonds),
        Rank::Six.of(super::card::Suite::Diamonds),
        Rank::Seven.of(super::card::Suite::Diamonds),
        Rank::Eight.of(super::card::Suite::Diamonds),
        Rank::Nine.of(super::card::Suite::Diamonds),
        Rank::Ten.of(super::card::Suite::Diamonds),
        Rank::Jack.of(super::card::Suite::Diamonds),
        Rank::Queen.of(super::card::Suite::Diamonds),
        Rank::King.of(super::card::Suite::Diamonds),
        Rank::Ace.of(super::card::Suite::Spades),
        Rank::Two.of(super::card::Suite::Spades),
        Rank::Three.of(super::card::Suite::Spades),
        Rank::Four.of(super::card::Suite::Spades),
        Rank::Five.of(super::card::Suite::Spades),
        Rank::Six.of(super::card::Suite::Spades),
        Rank::Seven.of(super::card::Suite::Spades),
        Rank::Eight.of(super::card::Suite::Spades),
        Rank::Nine.of(super::card::Suite::Spades),
        Rank::Ten.of(super::card::Suite::Spades),
        Rank::Jack.of(super::card::Suite::Spades),
        Rank::Queen.of(super::card::Suite::Spades),
        Rank::King.of(super::card::Suite::Spades),
        Rank::Two.of(super::card::Suite::Hearts),
        Rank::Three.of(super::card::Suite::Hearts),
        Rank::Four.of(super::card::Suite::Hearts),
        Rank::Five.of(super::card::Suite::Hearts),
        Rank::Six.of(super::card::Suite::Hearts),
        Rank::Seven.of(super::card::Suite::Hearts),
        Rank::Eight.of(super::card::Suite::Hearts),
        Rank::Nine.of(super::card::Suite::Hearts),
        Rank::Ten.of(super::card::Suite::Hearts),
    ];
}

const EMPTY_ROOM: [Option<Card>; 4] = [None, None, None, None];

impl Card {
    fn value(&self) -> u8 {
        match self.rank {
            Rank::Ace => 14,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
        }
    }
}

impl Game {
    fn populate_room(&mut self) {
        for slot in 0..4 {
            if self.room[slot].is_some() {
                continue;
            }
            self.room[slot] = self.dungeon.pop_top_card()
        }
    }

    pub fn start_new<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let mut dungeon = Pile::from(INITIAL_DUNGEON.clone());
        dungeon.shuffle(rng);

        let mut game = Game {
            dungeon,
            slain: vec![],
            health: 20,
            room: EMPTY_ROOM,
            discard_count: 0,
            equipped: None,
            already_avoided: false,
            already_healed: false,
        };
        game.populate_room();
        game
    }

    fn remove_room(&mut self) -> Vec<Card> {
        let room: Vec<Card> = self
            .room
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default();
        self.room = EMPTY_ROOM;
        room
    }

    fn count_occupied_room_slots(&self) -> usize {
        self.room.iter().filter(|it| it.is_some()).count()
    }

    fn room_is_full(&self) -> bool {
        self.count_occupied_room_slots() == 4
    }

    pub fn try_avoid<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        if self.already_avoided {
            return;
        }

        if !self.room_is_full() {
            return;
        }

        let mut room = self.remove_room();
        room.shuffle(rng);
        self.dungeon.add_to_bottom(room);
        self.populate_room();

        self.already_avoided = true;
    }

    fn take_card_from_slot(&mut self, slot: usize) -> Option<Card> {
        let card = self.room[slot];
        self.room[slot] = None;
        card
    }

    fn discard(&mut self) {
        self.discard_count += 1;
    }

    fn fight_bare_handed(&mut self, monster: Card) {
        self.health = self.health.saturating_sub(monster.value());
        self.discard();
    }

    fn fight_with(&mut self, monster: Card, weapon: &Card) {
        self.health = self.health.saturating_sub(monster.value());
        self.slain.push(monster);
    }

    fn can_heal(&self) -> bool {
        !self.already_healed
    }

    fn heal(&mut self, health: u8) {
        self.health = cmp::min(20, self.health + health);
        self.already_healed = true
    }

    fn reset_for_next_turn(&mut self) {
        self.populate_room();
        self.already_avoided = false;
        self.already_healed = false;
    }

    fn discard_equipped(&mut self) {
        if self.equipped.is_some() {
            self.discard();
            self.equipped = None;
        }

        self.discard_count += self.slain.len() as u8;
        self.slain.clear();
    }

    fn equip(&mut self, card: Card) {
        self.equipped = Some(card);
    }

    pub fn interact_slot(&mut self, slot: usize) {
        let Some(card) = self.take_card_from_slot(slot) else {
            return;
        };

        match card.suite {
            Suite::Hearts => {
                if self.can_heal() {
                    self.heal(card.value());
                }
                self.discard();
            }
            Suite::Clubs | Suite::Spades => {
                if let Some(weapon) = self.equipped {
                    self.fight_with(card, &weapon);
                } else {
                    self.fight_bare_handed(card);
                }
            }
            Suite::Diamonds => {
                self.discard_equipped();
                self.equip(card);
            }
        }

        let occupied_slots = self.count_occupied_room_slots();
        if occupied_slots == 1 {
            self.reset_for_next_turn();
        }
    }
}
