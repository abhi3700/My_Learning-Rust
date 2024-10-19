//! Home page.

use crate::Route;
use dioxus::prelude::*;

#[component]
pub(crate) fn Home() -> Element {
	let count = use_signal(|| 0);

	rsx! {
		nav { style: "display: flex; gap: 20px; padding: 10px; background-color: #EEC995;",
			Link { to: Route::Blog { id: count() }, "Blog" }
			Link { to: Route::Counter { id: count() }, "Counter" }
		}
		h2 { "Welcome to my Dixous playground! 🏃" }
	}
}
