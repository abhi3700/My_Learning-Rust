//! Counter App using Relm4

use relm4::{
	gtk::{glib::clone, prelude::*, Label},
	prelude::*,
};

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
struct AppWidgets {
	label: Label,
}

impl Component for CounterApp {
	type CommandOutput = ();

	type Input = AppInput;

	type Output = ();

	type Init = i16;

	type Root = gtk::Window;

	type Widgets = AppWidgets;

	fn init_root() -> Self::Root {
		gtk::Window::builder()
			.title("Counter App using Relm4")
			.default_height(300)
			.default_width(300)
			.build()
	}

	fn init(
		init: Self::Init,
		root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		/* 1. UI */
		let model = CounterApp { counter: init };
		let label = Label::new(Some(&format!("Counter: {}", model.counter.to_string())));

		let inc_button = gtk::Button::with_label("➕");
		let dec_button = gtk::Button::with_label("➖");

		let vbox = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(10).build();
		// NOTE: Set the child of the root widget like in React.
		root.set_child(Some(&vbox));
		vbox.append(&inc_button);
		vbox.append(&label);
		vbox.append(&dec_button);

		/* 2. event/message handler functions */
		inc_button.connect_clicked(clone!(@strong sender => move |_| {
			sender.input(AppInput::Increment);
		}));
		dec_button.connect_clicked(clone!(@strong sender => move |_| {
			sender.input(AppInput::Decrement);
		}));

		/* App widgets */
		let widgets = AppWidgets { label };

		ComponentParts { model, widgets }
	}

	// update the model based on the input/message/event
	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
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
	fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
		widgets.label.set_text(&format!("Counter: {}", self.counter.to_string()));
	}
}

fn main() {
	let app = RelmApp::new("abhi.counter.app");
	// NOTE: counter value is initialized to 0
	app.run::<CounterApp>(0)
}
