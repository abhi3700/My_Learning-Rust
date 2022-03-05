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

// There is a system trait to add support for default values.
// This is especially important when implementing generic types.
impl Default for TicTacToe {
    fn default() -> Self {
        TicTacToe::Empty
    }
}

pub fn run() {
    println!("{:?}", TicTacToe::default());

}