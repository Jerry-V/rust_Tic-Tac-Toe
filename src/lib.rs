use std::io::{self, stdout, Write};
use termion::{clear, cursor, raw::IntoRawMode};
pub struct Board {
    grid_cells: Vec<u8>,
    current_player: u8,
}
impl Default for Board {
    fn default() -> Self {
        Board {
            grid_cells: vec![0; 9],
            current_player: 1,
        }
    }
}

impl Board {
    pub fn change_element(&mut self, cell_number: u8) {
        let index = cell_number as usize - 1;
        if self.grid_cells[index] == 0 {
            self.grid_cells[index] = self.current_player;
        }
    }

    pub fn get_current_player(&mut self) -> u8 {
        self.current_player
    }

    pub fn change_player(&mut self) {
        if self.current_player == 2 {
            self.current_player = 1;
        } else {
            self.current_player = 2;
        }
    }

    pub fn check_element_is_empty(&mut self, cell_number: u8) -> bool {
        let index = cell_number as usize - 1;
        self.grid_cells[index] == 0
    }

    fn refresh_terminal() {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}", clear::All).unwrap();
        write!(stdout, "{}", cursor::Goto(1, 3)).unwrap();
        write!(stdout, "{}", cursor::Goto(1, 2)).unwrap();
        stdout.flush().unwrap();
    }

    pub fn print_board(&mut self) {
        // https://coding.tools/ascii-table
        // Used ASCII code (Decimal): 179, 196, 197 and maybe some others

        Board::refresh_terminal();
        println!(
            "Key:          Game Board:\n\n  1 │ 2 │ 3     {} │ {} │ {} \n ───┼───┼───   ───┼───┼───\n  4 │ 5 │ 6     {} │ {} │ {} \n ───┼───┼───   ───┼───┼───\n  7 │ 8 │ 9     {} │ {} │ {} ",
            self.player_symbol(0),
            self.player_symbol(1),
            self.player_symbol(2),
            self.player_symbol(3),
            self.player_symbol(4),
            self.player_symbol(5),
            self.player_symbol(6),
            self.player_symbol(7),
            self.player_symbol(8),

        );
        println!("\n");
    }

    pub fn player_symbol(&self, index: usize) -> &str {
        // Returns the symbol corresponding to which player's mark is read
        if self.grid_cells[index] == 1 {
            "X"
        } else if self.grid_cells[index] == 2 {
            "O"
        } else {
            " "
        }
    }
}

pub fn get_input() -> u8 {
    // Continually asks for a valid value until given one
    println!("q: quit");
    loop {
        print!("Input move (1-9): ");
        // flush ensures the prompt is immediately visible before waiting for user input
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input value");

        let trimmed_input = input.trim();

        match trimmed_input {
            "q" | "quit" => {
                // Exit the program
                std::process::exit(1);
            }
            _ => {
                match trimmed_input.parse::<u8>() {
                    // If parse is successful and within the valid range of values
                    Ok(parsed_value @ 1..=9) => {
                        return parsed_value;
                    }
                    // If parse is successful but outside the valid range of values
                    Ok(_) => {
                        println!("Only values from 1 to 9 are valid");
                        continue;
                    }
                    // If parse is unsuccessful
                    Err(_) => {
                        println!("Only values from 1 to 9 are valid");
                        continue;
                    }
                }
            }
        }
    }
}

pub fn check_win(input_board: &Board) -> bool {
    let win_rows = [
        // Horizontal wins
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        // Vertical wins
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        // Diagonal wins
        [0, 4, 8],
        [6, 4, 2],
    ];

    let symbol_positions: Vec<usize> = input_board
        .grid_cells
        .iter()
        .enumerate()
        .filter(|(_, &x)| x == input_board.current_player)
        .map(|(idx, _)| idx)
        .collect();

    let win_present: bool = win_rows
        .iter()
        .any(|row| row.iter().all(|element| symbol_positions.contains(element)));

    win_present
}

pub fn check_draw(input_board: &Board) -> bool {
    !input_board.grid_cells.iter().any(|x| *x == 0)
}

pub fn get_move(new_board: &mut Board) {
    // Get current player's move
    loop {
        let game_move = get_input();
        if new_board.check_element_is_empty(game_move) {
            new_board.change_element(game_move);
            break;
        } else {
            println!("Position already filled");
        }
    }
}
