use game_life::board::{self, *};
use macroquad::prelude::*;

fn draw_board(board: &mut Board) {
    let game_size = screen_width().min(screen_height());
    let offset_x = (screen_width() - game_size) / 2. + 10.;
    let offset_y = (screen_height() - game_size) / 2. + 10.;
    for i in 0..100 {
        for j in 0..100 {
            let sq_size = (screen_height() - offset_y * 2.) / 100 as f32;
            if board.get_cell(i, j) {
                draw_rectangle(
                    offset_x + i as f32 * sq_size,
                    offset_y + j as f32 * sq_size,
                    sq_size,
                    sq_size,
                    LIME,
                );
            }
        }
    }
}

#[macroquad::main("GameLife")]
async fn main() {
    let mut game_over = false;
    let mut board = Board::new(100, 100);
    board.rand_board(0.9);

    loop {
        if !game_over {
            clear_background(BLACK);
            let game_size = screen_width().min(screen_height());
            let offset_x = (screen_width() - game_size) / 2. + 10.;
            let offset_y = (screen_height() - game_size) / 2. + 10.;
            let sq_size = (screen_height() - offset_y * 2.) / 100 as f32;
            draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);
            for i in 1..100 {
                draw_line(
                    offset_x,
                    offset_y + sq_size * i as f32,
                    screen_width() - offset_x,
                    offset_y + sq_size * i as f32,
                    2.,
                    LIGHTGRAY,
                );
            }
            for i in 1..100 {
                draw_line(
                    offset_x,
                    offset_y + sq_size * i as f32,
                    screen_width() - offset_x,
                    offset_y + sq_size * i as f32,
                    2.,
                    LIGHTGRAY,
                );
            }
            for i in 1..100 {
                draw_line(
                    offset_x + sq_size * i as f32,
                    offset_y,
                    offset_x + sq_size * i as f32,
                    screen_height() - offset_y,
                    2.,
                    LIGHTGRAY,
                );
            }
            draw_board(&mut board);
            board.simulate_board();
        }
        next_frame().await;
    }
}
