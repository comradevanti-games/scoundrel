use std::cmp;

use lazy_static::lazy_static;
use rand::{Rng, seq::SliceRandom};

use super::{
    card::{Card, Rank, Suite},
    pile::Pile,
    room::Room,
};

pub enum GameOverState {
    Win,
    Death,
}

#[derive(PartialEq, Debug)]
pub struct Game {
    pub dungeon: Pile,
    pub slain: Vec<Card>,
    pub room: Room,
    pub health: u8,
    pub discard_count: u8,
    pub equipped: Option<Card>,
    pub already_avoided: bool,
    already_healed: bool,
}

lazy_static! {
    static ref INITIAL_DUNGEON: Vec<Card> = vec![
        "2D", "3D", "4D", "5D", "6D", "7D", "8D", "9D", "10D", "JD", "QD", "KD", "AD", "2H", "3H",
        "4H", "5H", "6H", "7H", "8H", "9H", "10H", "2C", "3C", "4C", "5C", "6C", "7C", "8C", "9C",
        "10C", "JC", "QC", "KC", "AC", "2S", "3S", "4S", "5S", "6S", "7S", "8S", "9S", "10S", "JS",
        "QS", "KS", "AS",
    ]
    .iter()
    .map(|s| s.parse().unwrap())
    .collect();
}

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

const INITIAL_HEALTH: u8 = 20;

impl Game {
    pub fn start_new<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let mut dungeon = Pile::from(INITIAL_DUNGEON.clone());
        dungeon.shuffle(rng);

        let mut game = Game {
            dungeon,
            slain: vec![],
            health: INITIAL_HEALTH,
            room: Room::empty(),
            discard_count: 0,
            equipped: None,
            already_avoided: false,
            already_healed: false,
        };
        game.room.populate_from(&mut game.dungeon);
        game
    }

    pub fn try_avoid<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        if self.already_avoided {
            return;
        }

        if !&self.room.is_full() {
            return;
        }

        let mut room = self.room.clear();
        room.shuffle(rng);
        self.dungeon.add_to_bottom(room);
        self.room.populate_from(&mut self.dungeon);

        self.already_avoided = true;
    }

    fn discard(&mut self) {
        self.discard_count += 1;
    }

    fn take_damage(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
    }

    fn fight_bare_handed(&mut self, monster: Card) {
        self.take_damage(monster.value());
        self.discard();
    }

    fn last_slain_monster(&self) -> Option<&Card> {
        self.slain.last()
    }

    fn can_fight(&self, monster: &Card) -> bool {
        match (self.equipped, self.last_slain_monster()) {
            (Some(_), None) => true,
            (Some(_), Some(slain)) => monster.value() <= slain.value(),
            _ => false,
        }
    }

    fn can_heal(&self) -> bool {
        !self.already_healed
    }

    fn heal(&mut self, health: u8) {
        self.health = cmp::min(20, self.health + health);
        self.already_healed = true
    }

    fn reset_for_next_turn(&mut self) {
        self.room.populate_from(&mut self.dungeon);
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

    fn weapon_damage(&self) -> Option<u8> {
        self.equipped.map(|weapon| weapon.value())
    }

    pub fn check_game_over(&self) -> Option<GameOverState> {
        if self.health == 0 {
            Some(GameOverState::Death)
        } else if self.dungeon.is_empty() && self.room.count_occupied_slots() == 0 {
            Some(GameOverState::Win)
        } else {
            None
        }
    }

    pub fn interact_slot(&mut self, slot: usize) {
        let Some(card) = self.room.take_card(slot) else {
            return;
        };

        match card.suite {
            Suite::Hearts => {
                if self.can_heal() {
                    self.heal(card.value());
                }
                self.discard();
            }
            Suite::Clubs | Suite::Spades => match self.equipped {
                Some(_) if self.can_fight(&card) => {
                    let damage = self.weapon_damage().unwrap();
                    self.take_damage(card.value().saturating_sub(damage));
                    self.slain.push(card);
                }
                _ => self.fight_bare_handed(card),
            },
            Suite::Diamonds => {
                self.discard_equipped();
                self.equip(card);
            }
        }

        let occupied_slots = self.room.count_occupied_slots();
        if occupied_slots == 1 {
            self.reset_for_next_turn();
        }
    }
}
