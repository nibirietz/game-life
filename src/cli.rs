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
    while ch != '\n' as i32 {
        let ch = getch();
        if ch == 'w' as i32 {
            x -= 1;
        }
        if ch == 'a' as i32 {
            y -= 1;
        }
        if ch == 's' as i32 {
            x += 1;
        }
        if ch == 'd' as i32 {
            y += 1;
        }
        if ch == ' ' as i32 {
            board.toggle_cell(x, y);
            clear();
            show_board(board);
        }
        if ch == 10 as i32 {
            break;
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
    clear();
    show_board(&board);

    loop {
        let ch = getch();
        if ch == 27 {
            break;
        }
        clear();
        board.simulate_board();
        show_board(&board);
    }

    refresh();

    getch();

    endwin();
}
