#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use freya::prelude::*;

fn main() {
	launch(app); // Be aware that this will block the thread it is running on
}

fn app() -> Element {
	// `use_signal` takes a callback that initializes the state
	// No matter how many times the component re runs,
	// the initialization callback will only run once at the first component run
	let mut state = use_signal(|| 0);

	// Because signals are copy, we can move them into closures
	let onclick = move |_| {
		// Signals provide some mutation shortcuts for certain types
		state += 1;
		// But we could do as well
		*state.write() += 1;
	};

	// You subscribe to a signal by calling it (`signal()`),
	// calling the `read()` method, or just embedding it into the RSX.
	// Everytime the signal is mutated the component function will rerun
	// because it has been subscribed, this will end up producing a
	// new `Element` with the updated counter.
	println!("{}", state());
	println!("{}", state.read());

	rsx!(
		label { onclick, "State is {state}" }
	)
}
