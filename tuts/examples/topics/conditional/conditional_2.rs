/*

        Conditional: match control
        - assign output to a variable

*/

fn main() {

    let grade = "A";

    let result = match grade {
        "A" => "Excellent!",
        "B" => "Great!",
        "C" => "Good",
        "D" => "You passed",
        "F" => "Sorry, you failed",
        _ => "Unknown Grade"
    };

    println!("Grade: {} - {}", grade, result);
}