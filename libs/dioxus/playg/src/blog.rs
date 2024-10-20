//! Blog page
use crate::Route;
use dioxus::prelude::*;

#[component]
pub(crate) fn Blog() -> Element {
	rsx! {
		div { style: "display: flex; gap: 5px; padding: 2px; flex-direction: column",
			Link { to: Route::Home {},
				button { "🔙 🏠" }
			}
			nav { style: "display: flex; gap: 20px; padding: 10px; background-color: #EEC995;",
				Link { to: Route::Tribune {}, "Tribune" }
				Link { to: Route::Toi {}, "Times of India" }
			}
		}
		h2 { "Blog" }
	}
}

#[component]
pub(crate) fn Tribune() -> Element {
	rsx! {
		h1 { "The Tribune" }
	}
}

#[component]
pub(crate) fn Toi() -> Element {
	rsx! {
		h1 { "The Times of India" }
	}
}
