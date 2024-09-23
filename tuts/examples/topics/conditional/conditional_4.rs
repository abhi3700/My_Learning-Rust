/*

        Conditional: match control
        - print the statement
        - assign output to a variable

*/

fn main() {

    let grade = "A";

    let result = match grade {
        "A" => { 
            println!("Excellent!");
            "95-100"
        },
        "B" => { 
            println!("Great!");
            "80-95"
        },
        "C" => { 
            println!("Good"); 
            "70-80"
        },
        "D" => { 
            println!("You passed"); 
            "60-70"
        },
        "F" => { 
            println!("Sorry, you failed i.e. <= 60");
            "<= 60"
        },
        _ => { 
            println!("Unknown Grade");
            ""
        },
    };

    println!("Grade: {} - {}", grade, result);

}