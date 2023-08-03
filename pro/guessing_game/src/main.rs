//! A Target Proximity Game that demonstrates various Rust concepts

use rand::Rng;
use std::io;

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

/// Define a function to get the max number
fn get_max_num(players: &Vec<Player>) -> u32 {
    players.len() as u32 * 50
}

/// Define a function to generate a random number
fn generate_number(max_num: u32) -> u32 {
    rand::thread_rng().gen_range(1..max_num)
}

/// Define a function to compare a guess to the target number
// fn compare_guess(guess: u32, target: u32) -> GuessResult {
//     match guess.cmp(&target) {
//         std::cmp::Ordering::Less => GuessResult::TooLow,
//         std::cmp::Ordering::Greater => GuessResult::TooHigh,
//         std::cmp::Ordering::Equal => GuessResult::Correct,
//     }
// }

/// Define a function to get the players
fn get_players() -> Vec<Player> {
    let mut players = Vec::new();
    let num_players = get_input::<u32>("How many players?");
    for i in 1..=num_players {
        let name = get_input(format!("Player {} name:", i).as_str());
        players.push(Player { name, score: 0 });
    }
    players
}

/// Define a function to get the proximity from the players
/// [proximity = abs_diff(guess, target)]
fn get_guesses(players: &Vec<Player>, max_num: u32) -> Vec<(String, u32)> {
    let mut player_proximities = Vec::<(String, u32)>::new();
    let target = generate_number(get_max_num(players));
    println!("target: {}", target);
    for player in players {
        println!("{}'s turn", player.name);
        let guess = get_input::<u32>(&format!("Guess the number (1-{max_num}):"));
        player_proximities.push((player.name.clone(), guess.abs_diff(target)));
    }
    player_proximities
}

/// Define a function to get the winner
fn get_winner(player_proximities: &Vec<(String, u32)>) -> String {
    player_proximities[0].0.clone()
}

/// Define a function to update the scores
fn update_scores(players: &mut Vec<Player>, winner: &str) {
    for player in players {
        if player.name == winner {
            player.score += 1
        }
    }
}

/// Define a function to print the scores
fn print_scores(players: &Vec<Player>) {
    println!("Scores: 📊");
    for player in players {
        println!("- {}", player.to_string());
    }
}

/// Define a function to play the game
fn play_game() {
    println!("Welcome to the Target Proximity Game! 🎮");
    let mut players = get_players();
    let max_num = get_max_num(&players);
    loop {
        let mut player_proximities = get_guesses(&players, max_num);
        player_proximities.sort_by_key(|&(_, v)| v);
        let winner = get_winner(&player_proximities);

        println!("Congratulations, {}! 🎉 You are the winner!", winner);
        update_scores(&mut players, &winner);
        print_scores(&players);

        let play_again: String = get_input("Play again? (y/n) 🔄");

        // if input is anything other than "y", it breaks
        if play_again.to_ascii_lowercase() != "y" {
            break;
        }
    }
}

/// Define the main function to run the game
fn main() {
    play_game();
}

// Define unit tests for the functions
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_compare_guess() {
//         assert_eq!(compare_guess(50, 50), GuessResult::Correct);
//         assert_eq!(compare_guess(50, 40), GuessResult::TooHigh);
//         assert_eq!(compare_guess(50, 60), GuessResult::TooLow);
//     }

//     #[test]
//     fn test_player_to_string() {
//         let player = Player {
//             name: "Alice".to_string(),
//             score: 3,
//         };
//         assert_eq!(player.to_string(), "Alice (3)");
//     }
// }
