//! Blog page

use crate::Route;
use dioxus::prelude::*;

#[component]
pub(crate) fn Blog() -> Element {
	rsx! {
		div { style: "display: flex; gap: 5px; padding: 2px; flex-direction: column",
			Link { to: Route::Home {},
				button { class: "bg-gray-200 hover:bg-gray-300 py-1.5 px-5 rounded-md",
					"🔙 🏠"
				}
			}
			nav { style: "display: flex; gap: 20px; padding: 10px; background-color: #EEC995;",
				Link { class: "text-blue-800 hover:underline", to: Route::Tribune {}, "Tribune" }
				Link { class: "text-blue-800 hover:underline", to: Route::Toi {}, "Times of India" }
			}
		}
		h2 { "Blog" }
	}
}

#[component]
pub(crate) fn Tribune() -> Element {
	rsx! {
		h1 { "📰 The Tribune" }
	}
}

#[component]
pub(crate) fn Toi() -> Element {
	rsx! {
		h1 { "📰 The Times of India" }
	}
}
