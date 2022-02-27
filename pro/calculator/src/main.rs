/// `parse()` is used by default to convert any to int type & then use unwrap() to return the value inside, if exist
/// `parse::<char>()` is used for converting any to char type & then use unwrap() to return the value inside, if exist
/// `unwrap()` is to capture the value if exist inside Some() used in nth function


use std::env::{args, Args};         // `Args` is optional to use

fn main() {
    let mut inputs = args();
    // my_string.parse::<i32>().unwrap()
    let input_1 = inputs.nth(1).unwrap();       // unwrap is used to get the value inside Some used in nth() function
    // M-1
    // let oper = inputs.nth(0).unwrap();
    // NOTE: to get the char out of string. As the string is an array of char. So, apply `char()` as iter & then extract the next or 1st element & then unwrap() if exist
    // M-2
    let oper = inputs.nth(0).unwrap().chars().next().unwrap();
    let input_2 = inputs.nth(0).unwrap();
    // println!("{:?}", input_1);
    // println!("{:?}", oper);
    // println!("{:?}", input_2);

    // print the result
    // M-1
    // println!("Result: {:?}", operate(oper.parse::<char>().unwrap(), input_1.parse::<f64>().unwrap(), input_2.parse::<f64>().unwrap()));
    // M-2
    println!("Result: {:?}", operate(oper, input_1.parse::<f64>().unwrap(), input_2.parse::<f64>().unwrap()));
    
}

// arithmetic function
fn operate(oper: char, first_input: f64, second_input: f64) -> f64 {
    // using `match` conditional
    // let res = match oper {
    //     '+' => input_1.parse::<u32>().unwrap() + input_2.parse::<u32>().unwrap(),
    //     _ => 0
    // };
    
    let mut res: f64 = 0.0;
    if oper == '+' {
        res = first_input + second_input; 
    }
    else if oper == '-' {
        res = first_input - second_input; 
    }
    else if oper == '*' {
        res = first_input * second_input; 
    }
    else if oper == '/' {
        res = first_input / second_input; 
    } else {
        res = 0.0;
    }

    res
}
