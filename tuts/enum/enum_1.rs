/*
	Enum:
	- match control with checking enum values.

*/

#[allow(dead_code)]
#[derive(Debug)]		// this use is recommended, otherwise there is error.
enum UsState {
	California,
	Mexico,
	Alaska,
}

#[allow(dead_code)]
enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter,
	Custom(UsState),
}

fn main() {
	// let state: UsState;
	let coin = Coin::Custom(UsState::Alaska);
	match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		Coin::Dime => 8,
		Coin::Quarter => 10,
		Coin::Custom(s) => {
			println!("custom coin is of: {:?}", s);
			25
		},
	};
}