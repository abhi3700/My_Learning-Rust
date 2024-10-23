#![allow(non_snake_case)]

mod blog;
mod counter;
mod home;

use self::manganis;
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

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("./public/tailwind.css"));

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
