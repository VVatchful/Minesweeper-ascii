extern crate rand;
use rand::{thread_rng, Rng};
use std::fmt;
use std::io;

const ROWS: usize = 10;
const COLS: usize = 10;
const MINES: usize = 15;

#[derive(Clone, Copy)]
struct Cell {
    is_mine: bool,
    is_revealed: bool,
    is_flagged: bool,
    adjacent_mines: u8,
}

impl Cell {
    fn new() -> Self {
        Cell {
            is_mine: false,
            is_revealed: false,
            is_flagged: false,
            adjacent_mines: 0,
        }
    }
}

struct GameBoard {
    grid: Vec<Vec<Cell>>,
    game_over: bool,
}

impl GameBoard {
    fn new() -> Self {
        let mut board = GameBoard {
            grid: vec![vec![Cell::new(); COLS]; ROWS],
            game_over: false,
        };
        board.place_mines();
        board
    }

    fn place_mines(&mut self) {
        let mut rng = thread_rng();
        let mut mines_placed = 0;
        while mines_placed < MINES {
            let row = rng.gen_range(0..ROWS);
            let col = rng.gen_range(0..COLS);
            if !self.grid[row][col].is_mine {
                self.grid[row][col].is_mine = true;
                mines_placed += 1;
            }
        }
        self.calculate_adjacent_mines();
    }

    fn calculate_adjacent_mines(&mut self) {
        for row in 0..ROWS {
            for col in 0..COLS {
                if !self.grid[row][col].is_mine {
                    let mut count = 0;
                    for i in -1..=1 {
                        for j in -1..=1 {
                            if i == 0 && j == 0 { continue; }
                            let new_row = (row as isize + i) as usize;
                            let new_col = (col as isize + j) as usize;
                            if new_row < ROWS && new_col < COLS && self.grid[new_row][new_col].is_mine {
                                count += 1;
                            }
                        }
                    }
                    self.grid[row][col].adjacent_mines = count;
                }
            }
        }
    }

    fn reveal_cell(&mut self, row: usize, col: usize) {
        if row >= ROWS || col >= COLS || self.grid[row][col].is_revealed {
            return;
        }

        self.grid[row][col].is_revealed = true;

        if self.grid[row][col].is_mine {
            self.game_over = true;
        } else if self.grid[row][col].adjacent_mines == 0 {
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 { continue; }
                    let new_row = (row as isize + i) as usize;
                    let new_col = (col as isize + j) as usize;
                    if new_row < ROWS && new_col < COLS {
                        self.reveal_cell(new_row, new_col);
                    }
                }
            }
        }
    }

    fn toggle_flag(&mut self, row: usize, col: usize) {
        if row < ROWS && col < COLS && !self.grid[row][col].is_revealed {
            self.grid[row][col].is_flagged = !self.grid[row][col].is_flagged;
        }
    }
}

impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            for cell in row {
                let symbol = if cell.is_revealed {
                    if cell.is_mine {
                        '*'
                    } else {
                        char::from_digit(cell.adjacent_mines as u32, 10).unwrap_or('0')
                    }
                } else if cell.is_flagged {
                    'F'
                } else {
                    '.'
                };
                write!(f, "{} ", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut game = GameBoard::new();
    while !game.game_over {
        println!("{}", game);
        println!("Enter action (r for reveal, f for flag), row and column:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }

        let action = parts[0];
        let row: usize = parts[1].parse().unwrap_or(ROWS);
        let col: usize = parts[2].parse().unwrap_or(COLS);

        match action {
            "r" => game.reveal_cell(row, col),
            "f" => game.toggle_flag(row, col),
            _ => println!("Invalid action! Use 'r' for reveal or 'f' for flag."),
        }

        if game.game_over {
            println!("Boom! You hit a mine!");
            println!("{}", game);
            break;
        }
    }
    if !game.game_over {
        println!("Congratulations! You have cleared the minefield!");
        println!("{}", game);
    }
}

