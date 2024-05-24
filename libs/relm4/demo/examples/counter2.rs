//! Counter App using Relm4

use relm4::{gtk::prelude::*, prelude::*};

// model for Counter App
// aka data type or app state.
struct CounterApp {
	counter: i16,
}

// App input i.e. based on these events, the model will be updated
#[derive(Debug)]
enum AppInput {
	Increment,
	Decrement,
}

// This includes widgets that receive the input from the user like buttons, text fields etc or show
// the output as label. In this case, we have buttons like Increment, Decrement to take the inputs
// and a label to show the counter.
// struct AppWidgets {
// 	label: Label,
// }

#[relm4::component]
impl SimpleComponent for CounterApp {
	type Init = i16;
	type Input = AppInput;
	type Output = ();

	view! {
		gtk::Window {
			set_title: Some("Counter app"),
			set_default_width: 300,
			set_default_height: 100,

			gtk::Box {
				set_orientation: gtk::Orientation::Vertical,
				set_spacing: 5,
				set_margin_all: 5,

				// NOTE: One way of setting label in a button
				gtk::Button {
					set_label: "➕",
					connect_clicked => AppInput::Increment
				},
				gtk::Label {
					#[watch]
					set_label: &format!("Counter: {}", model.counter),
					set_margin_all: 5,
				},
				// NOTE: Another way of setting label in a button
				gtk::Button::with_label("➖") {
					connect_clicked => AppInput::Decrement
				}
			}
		}
	}

	fn init(
		counter: Self::Init,
		root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		/* 1. UI */
		let model = CounterApp { counter };

		// Insert the macro code generation here
		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	// update the model based on the input/message/event
	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		match message {
			AppInput::Increment => {
				self.counter = self.counter.wrapping_add(1);
			},
			AppInput::Decrement => {
				self.counter = self.counter.wrapping_sub(1);
			},
		}
	}

	// update the view based on the model
	// fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
	// 	widgets.label.set_text(&format!("Counter: {}", self.counter.to_string()));
	// }
}

fn main() {
	let app = RelmApp::new("abhi.counter.app");
	// NOTE: counter value is initialized to 0
	app.run::<CounterApp>(0)
}
