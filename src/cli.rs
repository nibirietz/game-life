use crate::board::*;
use std::io;
extern crate ncurses;

use ncurses::*;
fn to_chtype(x: bool) -> chtype {
    if x == false {
        return '.' as chtype;
    } else {
        return '*' as chtype;
    }
}

fn show_board(board: &Board) {
    for i in 0..board.rows() {
        for j in 0..board.columns() {
            addch(to_chtype(board.board[i][j]));
        }
        addch('\n' as chtype);
    }
}

fn init_board(board: &mut Board) {
    let mut x = 0;
    let mut y = 0;
    let ch = getch();
    while ch != KEY_ENTER {
        let ch = getch();
        if ch == KEY_RIGHT {
            x += 1;
        }
        if ch == KEY_LEFT {
            x -= 1;
        }
        if ch == KEY_UP {
            y -= 1;
        }
        if ch == KEY_DOWN {
            y += 1;
        }
        if ch == ' ' as i32 {
            board.toggle_cell(x, y);
        }
    }
}

pub fn run_game() {
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

    let mut board = Board::new(rows, columns);

    initscr();
    clear();
    noecho();

    show_board(&board);

    refresh();

    init_board(&mut board);
    refresh();
    show_board(&board);
    refresh();

    getch();

    endwin();
}
