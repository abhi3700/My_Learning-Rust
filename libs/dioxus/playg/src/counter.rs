//! Counter page

use crate::Route;
use dioxus::prelude::*;

#[component]
pub(crate) fn Counter(id: i32) -> Element {
	let mut count = use_signal(|| id);

	// NOTE: Unnecessary when using tailwind CSS.
	let mut hovered = use_signal(|| false); // Track hover state

	rsx! {
		div { class: "flex flex-col gap-2 p-2",
			Link { to: Route::Home {},
				button { class: "bg-gray-200 hover:bg-gray-300 py-1.5 px-5 rounded-md",
					"🔙 🏠"
				}
			}
			h2 { class: "font-bold", "Hi-fi 🙌 Counter = {count}" }
			div { class: "flex gap-3",
				button {
					class: "bg-green-300 hover:bg-green-500 py-2 px-4 rounded-md font-bold",
					onclick: move |_| count += 1,
					"Like 👍"
				}
				// Button {
				// 	button_scheme: Some(daisy_rsx::ButtonScheme::Primary),
				// 	button_size: Some(daisy_rsx::ButtonSize::Medium),
				// 	button_type: Some(daisy_rsx::ButtonType::Submit),
				// 	children: rsx! { "Like 👍" }
				// }
				// Using tailwind CSS
				button {
					class: "bg-red-200 hover:bg-red-400 py-2 px-4 rounded-md font-bold",
					onclick: move |_| count -= 1,
					"Dislike 👎"
				}

				// Using Custom CSS
				button {
					// FIXME: Need to make this code work via custom class in `main.css` file.
					// class: "custom-button",
					// NOTE: It's CSS equivalent is working. Not sure how to add hover, so added "onmouseenter", "onmouseleave" fields.
					style: format!(
						"background-color: {}; color: #ffffff; font-weight: bold; padding: 0.5rem 1rem; border-radius: 0.25rem; border: none;",
						if hovered() { "#2563eb" } else { "#3B82F6" },
					),
					onmouseenter: move |_| hovered.set(true),
					onmouseleave: move |_| hovered.set(false),
					onclick: move |_| count.set(0),
					"Reset 🖲️"
				}
			}
		}
	}
}
