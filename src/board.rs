use rand::Rng;
use std::sync::Arc;

#[derive(Clone)]
pub struct Board {
    board: Vec<Vec<bool>>,
    rows: usize,
    columns: usize,
    born: Arc<Vec<usize>>,
    survive: Arc<Vec<usize>>,
}

impl Board {
    pub fn new(rows: usize, columns: usize) -> Board {
        let born: Vec<usize> = vec![3];
        let survive: Vec<usize> = vec![2, 3];
        Board::new_with_rules(rows, columns, born, survive)
    }

    pub fn new_with_rules(
        rows: usize,
        columns: usize,
        born: Vec<usize>,
        surive: Vec<usize>,
    ) -> Board {
        let new_board: Vec<Vec<bool>> = vec![vec![false; rows]; columns];
        Board {
            board: new_board,
            rows: rows,
            columns: columns,
            born: Arc::new(born),
            survive: Arc::new(surive),
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> bool {
        return self.board[x][y];
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn cell_is_live(&self, x: usize, y: usize) -> bool {
        !(x >= self.rows || y >= self.columns) && self.board[x][y]
    }

    fn count_live_neighbours(self, x: usize, y: usize) -> usize {
        let x_1 = x.wrapping_sub(1);
        let y_1 = y.wrapping_sub(1);
        let mut count_neighbours: usize = 0;
        #[rustfmt::skip]
        let neighbours = [
            self.cell_is_live(x_1, y_1),     self.cell_is_live(x, y_1),      self.cell_is_live(x + 1, y_1),
            self.cell_is_live(x_1, y),                                       self.cell_is_live(x + 1, y),
            self.cell_is_live(x_1, y + 1), self.cell_is_live(x, y + 1), self.cell_is_live(x + 1, y + 1),
        ];
        for x in neighbours {
            count_neighbours += x as usize;
        }
        count_neighbours
    }

    fn next_board(&mut self, board: Vec<Vec<bool>>) {
        if board.len() == self.board.len() && board[0].len() == self.board[0].len() {
            self.board = board;
        }
    }

    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        if x < self.rows && y < self.columns {
            self.board[x][y] = !self.board[x][y];
        }
    }

    pub fn simulate_board(&mut self) {
        let mut new_board: Vec<Vec<bool>> = vec![vec![false; self.rows]; self.columns];
        for x in 0..new_board.len() {
            for y in 0..new_board.len() {
                if self.cell_is_live(x, y)
                    && self
                        .survive
                        .contains(&self.clone().count_live_neighbours(x, y))
                {
                    new_board[x][y] = true;
                }
                if !self.cell_is_live(x, y)
                    && self
                        .born
                        .contains(&self.clone().count_live_neighbours(x, y))
                {
                    new_board[x][y] = true;
                }
            }
        }
        self.next_board(new_board);
    }

    pub fn rand_board(&mut self, coefficient: f64) {
        let mut new_board = vec![vec![false; self.rows]; self.columns];
        let mut rng = rand::rng();

        for _ in 0..(((self.rows * self.rows) as f64 * coefficient) as usize) {
            let x = rng.random_range(0..self.rows);
            let y = rng.random_range(0..self.columns);
            new_board[x][y] = true;
        }
        self.next_board(new_board);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn init() {
        let mut board = Board::new(10, 10);
        board.toggle_cell(1, 2);
        board.toggle_cell(2, 2);
        board.toggle_cell(3, 2);
        board.simulate_board();
        println!("meow");
    }
}
