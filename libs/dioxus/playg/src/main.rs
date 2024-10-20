#![allow(non_snake_case)]

mod blog;
mod counter;
mod home;

use blog::{Blog, Toi, Tribune};
use counter::Counter;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use home::Home;

/// One global route for the entire App.
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
	#[route("/")]
	Home {},
	#[route("/blog")]
	Blog {},
	#[route("/blog/tribune")]
	Tribune {},
	#[route("/blog/toi")]
	Toi {},
	#[route("/counter/:id")]
	Counter { id: i32 },
}

fn main() {
	// Init logger
	dioxus_logger::init(Level::INFO).expect("failed to init logger");
	info!("starting app");
	launch(App);
}

fn App() -> Element {
	rsx! {
		Router::<Route> {}
	}
}
