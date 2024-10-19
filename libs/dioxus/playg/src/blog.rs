//! Blog page
use crate::Route;
use dioxus::prelude::*;

// TODO: Add Tribune, TOI
// enum Blog {
// 	Tribune
//	Toi
// }

#[component]
pub(crate) fn Blog(id: i32) -> Element {
	rsx! {
		div { style: "display: flex; gap: 5px; padding: 2px; flex-direction: column",
			Link { to: Route::Home {},
				button { "🔙" }
			}
			nav { style: "display: flex; gap: 20px; padding: 10px; background-color: #EEC995;",
				Link { to: "/blog/tribune", "Tribune" }
				Link { to: "/blog/toi", "Times of India" }
			}
		}
		h2 { "Blog post {id}" }
	}
}
