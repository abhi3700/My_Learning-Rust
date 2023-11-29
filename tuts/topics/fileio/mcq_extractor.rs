//! Extract MCQs from a text file

use std::{fs::File, io::BufRead};

pub(crate) fn main() {
    // give the relative path of the file w.r.t. the `./src/main.rs` file
    let file = File::open("./tuts/fileio/mcq.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    // create empty vectors to store the questions, options, answers & explanations
    let mut questions: Vec<String> = Vec::new();
    let mut options_a: Vec<String> = Vec::new();
    let mut options_b: Vec<String> = Vec::new();
    let mut options_c: Vec<String> = Vec::new();
    let mut options_d: Vec<String> = Vec::new();
    let mut answers: Vec<String> = Vec::new();
    let mut explanations: Vec<String> = Vec::new();

    // Each line may contain a question starting with 'Q:' or
    // options starting with 'A:', 'B:', 'C:', 'D:' or
    // answer starting with 'Solution:' or
    // if other, then trim the contents of the line until 'Q:' found or
    // explanation starting with 'Explanation:'
    // So, we need to extract the question, options, answer & explanation from each line based on the what
    // the line starts with & store them in the respective vectors:
    // questions, options_a, options_b, options_c, options_d, answers & explanations
    for line in reader.lines() {
        let line = line.unwrap();

        // trim the initial whitespace if any
        let line = line.trim_start();
        // println!("{}", line);

        if line.contains("Q:") {
            // extract the question from the line
            let (_, question) = line.split_at(6);
            questions.push(question.to_string());
        } else if line.starts_with("A:") {
            // extract the option from the line
            let option = line.trim_start_matches("A: ").to_string();
            options_a.push(option);
        } else if line.starts_with("B:") {
            // extract the option from the line
            let option = line.trim_start_matches("B: ").to_string();
            options_b.push(option);
        } else if line.starts_with("C:") {
            // extract the option from the line
            let option = line.trim_start_matches("C: ").to_string();
            options_c.push(option);
        } else if line.starts_with("D:") {
            // extract the option from the line
            let option = line.trim_start_matches("D: ").to_string();
            options_d.push(option);
        } else if line.starts_with("Solution:") {
            // extract the answer from the line
            let answer = line.trim_start_matches("Solution: ").to_string();
            answers.push(answer);
        } else if line.starts_with("Explanation: ") {
            // extract the explanation from the line
            let explanation = line.trim_start_matches("Explanation: ").to_string();
            explanations.push(explanation);
        } else {
            ()
        }
    }

    // iterate over the questions, options, answers & explanations vectors
    // & print the contents line by line
    questions
        .iter()
        .for_each(|question| println!("{}", question));

    options_a.iter().for_each(|option| println!("{}", option));
    options_b.iter().for_each(|option| println!("{}", option));
    options_c.iter().for_each(|option| println!("{}", option));
    options_d.iter().for_each(|option| println!("{}", option));
    answers.iter().for_each(|answer| println!("{}", answer));
    explanations
        .iter()
        .for_each(|explanation| println!("{}", explanation));

    // TODO: insert each of the vector element into gsheet cell as per columns
}
