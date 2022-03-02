/// `parse()` is used by default to convert any to int type & then use unwrap() to return the value inside, if exist
/// `parse::<char>()` is used for converting any to char type & then use unwrap() to return the value inside, if exist
/// `unwrap()` is to capture the value if exist inside Some() used in nth function


use std::{env::{args, Args}, result};         // `Args` is optional to use

fn main() {
    let mut inputs = args();
    // my_string.parse::<i32>().unwrap()
    let input_1 = inputs.nth(1).unwrap();       // unwrap is used to get the value inside Some used in nth() function
    // M-1
    // let oper = inputs.nth(0).unwrap();
    // NOTE: to get the char out of string. As the string is an array of char. So, apply `char()` as iter & then extract the next or 1st element & then unwrap() if exist
    // M-2 [RECOMMENDED]
    let oper = inputs.nth(0).unwrap().chars().next().unwrap();
    let input_2 = inputs.nth(0).unwrap();

    let num_1 = input_1.parse::<f64>().unwrap();
    let num_2 = input_2.parse::<f64>().unwrap();
    // println!("{:?}", input_1);
    // println!("{:?}", oper);
    // println!("{:?}", input_2);

    // ==================================================
    // print the result
    // M-1
    // println!("Result: {:?}", operate(oper.parse::<char>().unwrap(), input_1.parse::<f64>().unwrap(), input_2.parse::<f64>().unwrap()));
    
    // M-2
    let res = operate(oper, input_1.parse::<f64>().unwrap(), input_2.parse::<f64>().unwrap());
    println!("{}", output(num_1, oper, num_2, res) );
    
}

// arithmetic function
fn operate(oper: char, first_input: f64, second_input: f64) -> f64 {
    // using pattern matching
    let res = match oper {
        '+' => first_input + second_input,
        '-' => first_input - second_input,
        '*' | 'x' | 'X' => first_input * second_input,
        '/' => {
            if second_input == 0.0 {
                panic!("Error: Divide by zero")
            } else {
                first_input / second_input
            }
        },
        _ => panic!("Invalid operator used")
    };

    res
}

// format method to output string
fn output(first_input: f64, oper: char, second_input: f64, res: f64) -> String {
    format!("{} {} {} = {}", first_input, oper, second_input, res)
}
