use crate::player::Player;
use std::fmt;

const WIN_ARRAYS: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

pub struct Game {
    pub winner: Option<Player>,
    pub full: bool,
    board: [char; 9],
}

pub enum GameInput {
    Ok,
    Filled,
    Wrong,
}

impl Game {
    pub fn new() -> Game {
        Game {
            winner: None,
            full: false,
            board: ['1', '2', '3', '4', '5', '6', '7', '8', '9'],
        }
    }

    pub fn end(&self) -> bool {
        match &self.winner {
            Some(player) => {
                println!("The winner is {}", player);
                return true;
            }
            None => {}
        }

        if self.full {
            println!("It's an equality, no winner");
        }

        return self.full;
    }

    pub fn update(&mut self, cell: usize, player: &Player) -> GameInput {
        if cell > 8 {
            return GameInput::Wrong;
        }

        match self.board.get(cell) {
            Some(value) => {
                if *value == 'X' || *value == 'O' {
                    return GameInput::Filled;
                }

                self.board[cell] = player.to_char();
            }
            None => {
                return GameInput::Wrong;
            }
        }

        GameInput::Ok
    }

    pub fn check_winner(&mut self) {
        let mut winner: Option<Player> = None;

        for array in WIN_ARRAYS {
                let first = self.board[array[0]];
                let second = self.board[array[1]];
                let third = self.board[array[2]];

                if first == second && second == third {
                    if first == 'O' {
                        winner = Some(Player::One);
                    } else if first == 'X' {
                        winner = Some(Player::Two);
                    }

                    break;
                }

            match winner {
                Some(_) => {
                    break;
                }
                None => {}
            }
        }

        self.winner = winner;
    }

    pub fn check_full_filled(&mut self) {
        let mut full_filled = true;

        for cell in self.board {
            if cell != 'X' && cell != 'O' {
                full_filled = false;
                break;
            }
        }

        self.full = full_filled;
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} | {} | {}\n",
            self.board[0], self.board[1], self.board[2]
        )?;
        write!(
            f,
            "{} | {} | {}\n",
            self.board[3], self.board[4], self.board[5]
        )?;
        write!(
            f,
            "{} | {} | {}",
            self.board[6], self.board[7], self.board[8]
        )
    }
}
