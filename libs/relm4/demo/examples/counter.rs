//! Counter App using Relm4

use relm4::{
	gtk::{glib::clone, prelude::*, Label},
	prelude::*,
};

// App input i.e. based on these events, the model will be updated
#[derive(Debug)]
enum AppInput {
	Increment,
	Decrement,
}

// Widgets for receiving the input from the user like buttons, text fields etc or showing the
// output. In this case, we have buttons like Increment, Decrement and a label to show the counter.
struct AppWidgets {
	label: Label,
}

// model for Counter App
// aka data type or app state.
struct App {
	counter: u8,
}

impl Component for App {
	type CommandOutput = ();

	type Input = AppInput;

	type Output = ();

	type Init = u8;

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
		let model = App { counter: init };
		let label = Label::new(Some(&format!("Counter: {}", model.counter.to_string())));

		let inc_button = gtk::Button::with_label("Increment");
		let dec_button = gtk::Button::with_label("Decrement");

		inc_button.connect_clicked(clone!(@strong sender => move |_| {
			sender.input(AppInput::Increment);
		}));

		dec_button.connect_clicked(clone!(@strong sender => move |_| {
			sender.input(AppInput::Decrement);
		}));

		let vbox = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(10).build();
		root.set_child(Some(&vbox));
		vbox.append(&inc_button);
		vbox.append(&label);
		vbox.append(&dec_button);

		let widgets = AppWidgets { label };

		ComponentParts { model, widgets }
	}

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

	fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
		widgets.label.set_text(&format!("Counter: {}", self.counter.to_string()));
	}
}

fn main() {
	let app = RelmApp::new("abhi.counter.app");
	app.run::<App>(0)
}
