use std::io;

pub fn run(){
    println!("Guess the number!");
    println!("Please input the number:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}