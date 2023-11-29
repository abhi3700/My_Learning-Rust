/* 
    unreachable
    - used to mark the routes that the program should not enter.
*/

pub fn run() {
    let level = 22;
    let stage = match level {
        1..=5 => "beginner",
        6..=10 => "intermediate", 
        11..=20 => "advanced",
        _ => unreachable!("should not be the case with {}", level)
    };

    println!("{:?}", stage);
}