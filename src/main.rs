use crate::gameboard::Game;
use crate::player::Player;

pub mod gameboard;
pub mod player;

// Game
fn game() {
    let mut game = Game::new();
    let mut player = Player::One;

    println!("Welcome to the morpion game!\n");

    loop {
        println!("{}", game);

        player.turn(&mut game);

        game.check_winner();
        game.check_full_filled();

        if game.end() {
            break;
        }

        player = player.switch();
    }
}

fn main() {
    game();
}
