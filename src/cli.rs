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
}

impl Game {
    fn show_board(&self) {
        for i in 0..self.board.rows() {
            for j in 0..self.board.columns() {
                addch(to_chtype(self.board.board[i][j]));
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
            println!("{} {}", x, y);
        }
    }

    pub fn run_game(&mut self) {
        initscr();
        clear();
        noecho();

        self.show_board();

        refresh();

        self.init_board();
        refresh();
        clear();
        self.show_board();

        loop {
            let ch = getch();
            if ch == 27 {
                break;
            }
            clear();
            self.board.simulate_board();
            self.show_board();
        }

        refresh();

        getch();

        endwin();
    }

    pub fn create() -> Game {
        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        let rows = input_line.trim().parse().expect("Invalid input.");

        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        let columns = input_line.trim().parse().expect("Invalid input.");

        let board = Board::new(rows, columns);
        Self { board: board }
    }
}
