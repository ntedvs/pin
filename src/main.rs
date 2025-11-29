#[derive(Clone, Copy)]
struct Game {
    board: [u8; 9],
}

impl Game {
    fn new() -> Self {
        Game { board: [0; 9] }
    }

    fn print(&self) {
        for row in 0..3 {
            for col in 0..3 {
                let output = match self.board[row * 3 + col] {
                    1 => 'X',
                    2 => 'O',
                    _ => '*',
                };

                print!("{}", output);
            }

            println!();
        }
    }

    fn empty(&self) -> Vec<usize> {
        self.board
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == 0)
            .map(|(i, _)| i)
            .collect()
    }

    fn winner(&self) -> Option<u8> {
        const LINES: [[usize; 3]; 8] = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for line in LINES {
            let [a, b, c] = line;
            let spot = self.board[a];

            if spot != 0 && spot == self.board[b] && spot == self.board[c] {
                return Some(spot);
            }
        }

        None
    }

    fn place(&self, index: usize, player: u8) -> Game {
        let mut new = *self;

        new.board[index] = player;
        new
    }
}

fn main() {
    let game = Game::new();
    game.print();
}
