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
}

fn main() {
    let game = Game::new();
    game.print();
}
