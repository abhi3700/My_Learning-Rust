//! Counter page
use crate::Route;
use dioxus::prelude::*;

#[component]
pub(crate) fn Counter(id: i32) -> Element {
	let mut count = use_signal(|| id);

	rsx! {
		div { style: "display: flex; flex-direction: column; gap: 3px;",
			Link { to: Route::Home {},
				button { "🔙 🏠" }
			}
			h2 { "Hi-fi 🙌 Counter = {count}" }
			div { style: "display: flex; flex-direction: row; gap: 2px;",
				button { onclick: move |_| count += 1, "Like 👍" }
				button { onclick: move |_| count -= 1, "Dislike 👎" }
			}
		}
	}
}
