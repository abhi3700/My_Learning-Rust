/* 
    Default trait

    In this example, during the game the default value would always be empty,
    rather than 'X' or 'O'
*/

#[derive(Debug)]
enum TicTacToe {
    Empty,
    X,
    O,
}

impl Default for TicTacToe {
    fn default() -> Self {
        TicTacToe::Empty
    }
}

pub fn run() {
    println!("{:?}", TicTacToe::default());

}