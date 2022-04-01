/* 
Use of 'From' trait
The Into trait is simply the reciprocal of the From trait. That is, if you have implemented the From trait for your type, Into will call it when necessary.
*/

use std::convert::From;

#[derive(Debug)]
enum SquareContent {
    Empty,
    X,
    O
}

impl From<u8> for SquareContent {
    fn from(value: u8) -> Self {
        match value {
            0 => SquareContent::Empty,
            1 => SquareContent::X,
            2 => SquareContent::O,
            v => panic!("Cannot convert this: {}", v)
        }
    }
}

impl From<SquareContent> for u8 {
    fn from(c: Self) -> u8 {
        match c {
            SquareContent::Empty => 0,
            SquareContent::X => 1,
            SquareContent::O => 2,
            v => panic!("Cannot convert this: {}", v)
        }
    }
}


pub fn run() {
    println!("{:?}", SquareContent::from(0));
    println!("{:?}", SquareContent::from(1));
    println!("{:?}", SquareContent::from(2));

    // let x = SquareContent::Empty;
    // let num: u8 = x.into();
    // println!("{:?}", x.into());
    
}