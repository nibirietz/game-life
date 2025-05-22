use crate::board::*;
use std::io;
extern crate ncurses;

use ncurses::*;
fn to_chtype(x: bool) -> chtype {
    if x == false {
        return '*' as chtype;
    } else {
        return '@' as chtype;
    }
}

pub struct Game {
    board: Board,
    generation: usize,
}

impl Game {
    fn show_board(&self) {
        for i in 0..self.board.rows() {
            for j in 0..self.board.columns() {
                addch(to_chtype(self.board.get_cell(i, j)));
            }
            addch('\n' as chtype);
        }
    }

    fn init_board(&mut self) {
        let mut x = 0;
        let mut y = 0;
        let ch = getch();
        while ch != '\n' as i32 {
            let ch = getch();
            // println!("{} {}", x, y);
            if ch == 'w' as i32 {
                if x > 0 {
                    x -= 1;
                }
            }
            if ch == 'a' as i32 {
                if y > 0 {
                    y -= 1;
                }
            }
            if ch == 's' as i32 {
                if x + 1 < self.board.rows() {
                    x += 1;
                }
            }
            if ch == 'd' as i32 {
                if y + 1 < self.board.columns() {
                    y += 1;
                }
            }
            if ch == ' ' as i32 {
                self.board.toggle_cell(x, y);
                clear();
                self.show_board();
            }
            if ch == 10 as i32 {
                break;
            }
            // println!("{} {}", x, y);
        }
    }

    pub fn run_game(&mut self) {
        println!("With random? (y/n)");
        let mut flag_random = false;
        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        if input_line == "y\n" {
            flag_random = true;
        }

        if flag_random {
            let mut input_line = String::new();
            io::stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");
            let coefficient: f64 = input_line.trim().parse().expect("Invalid input.");

            self.board.rand_board(coefficient);
        }
        initscr();
        noecho();

        if !flag_random {
            addstr("WASD for control, Space to toggle, Enter to Run.\n").unwrap();
            self.show_board();
            self.init_board();
            refresh();
        }
        self.show_board();
        refresh();

        clear();
        addstr("Escape for exit, any key to next generation.\n").unwrap();
        self.show_board();

        loop {
            self.generation += 1;
            let ch = getch();
            if ch == 27 {
                break;
            }
            clear();
            addstr("Escape for exit, any key to next generation.\n").unwrap();
            self.board.simulate_board();
            self.show_board();
            addstr(format!("Generation: {}", self.generation).as_ref()).unwrap();
        }

        refresh();

        getch();

        endwin();
    }

    pub fn create() -> Game {
        println!("Input width:");
        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        let rows = input_line.trim().parse().expect("Invalid input.");

        println!("Input height:");
        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        let columns = input_line.trim().parse().expect("Invalid input.");

        let board = Board::new(rows, columns);
        Self {
            board: board,
            generation: 0,
        }
    }
}
