#![allow(non_snake_case)]

mod blog;
mod counter;
mod home;

use blog::Blog;
use counter::Counter;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use home::Home;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
	#[route("/")]
	Home {},
	#[route("/blog/:id")]
	Blog { id: i32 },
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
