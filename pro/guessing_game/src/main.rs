// A guessing game that demonstrates various Rust concepts

use rand::Rng;
use std::{collections::HashMap, io};

/// Define a struct to represent a player
struct Player {
    name: String,
    score: u32,
}

/// Define an enum to represent the result of a guess
#[derive(PartialEq, Debug)]
enum GuessResult {
    Correct,
    TooHigh,
    TooLow,
}

/// Define a trait to represent printable objects
trait Printable {
    fn to_string(&self) -> String;
}

// Implement the Printable trait for the Player struct
impl Printable for Player {
    fn to_string(&self) -> String {
        format!("{} ({})", self.name, self.score)
    }
}

/// Define a generic function to get user input
fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => continue,
        }
    }
}

/// Define a function to generate a random number
fn generate_number(max_num: u32) -> u32 {
    rand::thread_rng().gen_range(1..max_num)
}

/// Define a function to compare a guess to the target number
fn compare_guess(guess: u32, target: u32) -> GuessResult {
    match guess.cmp(&target) {
        std::cmp::Ordering::Less => GuessResult::TooLow,
        std::cmp::Ordering::Greater => GuessResult::TooHigh,
        std::cmp::Ordering::Equal => GuessResult::Correct,
    }
}

/// Define a function to play a round of the game
fn play_round(player: &mut Player, target: u32, max_num: u32) {
    println!("{}'s turn", player.name);
    // let guesses = HashMap::<>::new()
    let guess = get_input::<u32>(&format!("Guess the number (1-{max_num}):"));
    match compare_guess(guess, target) {
        GuessResult::Correct => {
            println!("Congratulations, {}! 🎉 You are the winner!", player.name);
            player.score += 1;
        }
        GuessResult::TooHigh => println!("Too high!"),
        GuessResult::TooLow => println!("Too low!"),
    }
}

// Define a function to play the game
fn play_game() {
    println!("Welcome to the Target Proximity Game! 🎮");
    let mut players = Vec::new();
    let num_players = get_input::<u32>("How many players?");
    for i in 1..=num_players {
        let name = get_input(format!("Player {} name:", i).as_str());
        players.push(Player { name, score: 0 });
    }
    let max_num = players.len() as u32 * 50;
    loop {
        let mut players_guess_diffs = Vec::<(String, u32)>::new();
        let target = generate_number(max_num);
        // collect the guesses
        for player in &players {
            //// play_round(player, target, max_num);
            println!("{}'s turn", player.name);
            let guess = get_input::<u32>(&format!("Guess the number (1-{max_num}):"));
            players_guess_diffs.push((player.name.clone(), guess.abs_diff(target)));
        }

        // sort by value
        players_guess_diffs.sort_by_key(|&(_, v)| v);
        println!("Debug: {:?}", players_guess_diffs);
        let winner = &players_guess_diffs[0].0;
        println!("Congratulations, {}! 🎉 You are the winner!", winner);
        let _ = players.iter_mut().map(|x| {
            if x.name == *winner {
                x.score += 1
            }
        });
        println!("Scores: 📊");
        for player in &players {
            println!("- {}", player.to_string());
        }
        let play_again: String = get_input("Play again? (y/n) 🔄");

        // if input is anything other than "y", it breaks
        if play_again.to_ascii_lowercase() != "y" {
            break;
        }
    }
}

// Define unit tests for the functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_guess() {
        assert_eq!(compare_guess(50, 50), GuessResult::Correct);
        assert_eq!(compare_guess(50, 40), GuessResult::TooHigh);
        assert_eq!(compare_guess(50, 60), GuessResult::TooLow);
    }

    #[test]
    fn test_player_to_string() {
        let player = Player {
            name: "Alice".to_string(),
            score: 3,
        };
        assert_eq!(player.to_string(), "Alice (3)");
    }
}

// Define the main function to run the game
fn main() {
    play_game();
}
