//! If you are a `tokio` user, then u can see that there is no `#[tokio::main]` attribute on `main`
//! function.
//!
//! `smol` uses a `block_on` function that acts as executor.
//!
//! So, no `async`/`await` used here unlike in `tokio`, the `main` fn would be defined as async &
//! the `get_number` would be awaited to return the value.

async fn get_number() -> u8 {
	42
}

fn main() {
	let num = get_number();
	let result = smol::block_on(num);
	assert_eq!(result, 42);
}
