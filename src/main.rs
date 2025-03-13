use domain::game::Game;

mod domain {
    pub mod card;
    pub mod game;
    pub mod pile;
}

fn main() {
    let mut rng = rand::rng();
    let game = Game::start_new(&mut rng);

    println!("{:#?}", game.dungeon);
}
