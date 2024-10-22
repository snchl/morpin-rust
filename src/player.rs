use crate::gameboard::{Game, GameInput};
use std::{fmt, io};

pub enum Player {
    One,
    Two,
}

impl Player {
    pub fn turn(&self, game: &mut Game) {
        println!("{} it's your turn!", self);

        loop {
            let input_string = self.input();

            match input_string.parse::<u16>() {
                Ok(cell) => match game.update((cell - 1) as usize, self) {
                    GameInput::Ok => {
                        break;
                    }
                    GameInput::Filled => {
                        println!("This cell is already filled, please retry");
                        continue;
                    }
                    GameInput::Wrong => {
                        println!("This cell doesn't exist, pleasy retry");
                        continue;
                    }
                },
                Err(_) => {
                    println!("You input invalid cell number, please retry");
                    continue;
                }
            };
        }
    }

    pub fn switch(self) -> Player {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }

    fn input(&self) -> String {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read your input");

        // Remove newline char
        input.pop();

        input
    }

    pub fn to_char(&self) -> char {
        match self {
            Player::One => PlayerInput::O.to_char(),
            Player::Two => PlayerInput::X.to_char(),
        }
    }

    fn to_number_char(&self) -> char {
        match self {
            Player::One => '1',
            Player::Two => '2',
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player {}", self.to_number_char())
    }
}

enum PlayerInput {
    O,
    X,
}

impl PlayerInput {
    pub fn to_char(&self) -> char {
        match self {
            PlayerInput::O => 'O',
            PlayerInput::X => 'X',
        }
    }
}
