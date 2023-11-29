/*
	Here, dead_code is used only because Coin::Custom has been used as variable assignment 
*/

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Custom(i32),
}

fn main() {
	let coin = Coin::Custom(30);

	let a = match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		Coin::Dime => 8,
		Coin::Custom(e) => e,
	};

	assert_eq!(a, 30);

}