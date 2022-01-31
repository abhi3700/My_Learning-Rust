use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input the number:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_guess_num() {
        let num = 3;
        assert_eq!(num, 3);
    }
}