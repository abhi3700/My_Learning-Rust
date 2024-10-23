//! Home page.

use crate::Route;
use dioxus::prelude::*;

#[component]
pub(crate) fn Home() -> Element {
	let count = use_signal(|| 0);

	rsx! {
		div { class: "flex flex-col gap-2 p-2",
			h1 { class: "font-mono font-bold", "🏠 HOME" }
			nav { style: "display: flex; gap: 20px; padding: 10px; background-color: #C9C9E9;",
				Link { class: "text-blue-800 hover:underline", to: Route::Blog {}, "Blog" }
				Link {
					class: "text-blue-800 hover:underline",
					to: Route::Counter { id: count() },
					"Counter"
				}
			}
			h2 { "Welcome to my Dixous playground! ⚽🏃" }
		}
	}
}
