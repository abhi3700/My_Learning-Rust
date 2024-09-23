/*
	match control conditional statement with functions
*/


fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}

fn add_fancy_hat() {
    println!("add fancy hat");
}
fn remove_fancy_hat() {
    println!("remove fancy hat");
}
fn reroll() {
    println!("reroll");
}