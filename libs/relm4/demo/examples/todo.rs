use relm4::{
	gtk::{self, glib::clone, prelude::*, Label},
	Component, ComponentParts, RelmApp,
};

#[derive(Debug)]
enum TodoInput {
	Add(String),
	Remove(u32),
	Toggle(u32),
}

struct AppWidgets {
	name: Label,
	done: Label,
}

// App model
struct TodoItem {
	id: u32,
	name: String,
	done: bool,
}

struct TodoApp {
	items: Vec<TodoItem>,
}

impl Default for TodoApp {
	fn default() -> Self {
		Self { items: vec![] }
	}
}

impl Component for TodoApp {
	type CommandOutput = ();

	type Input = TodoInput;

	type Output = ();

	type Init = Vec<TodoItem>;

	type Root = gtk::Window;

	type Widgets = AppWidgets;

	fn init_root() -> Self::Root {
		gtk::Window::builder()
			.title("Todo App using Relm4")
			.default_height(300)
			.default_width(300)
			.build()
	}

	fn init(
		init: Self::Init,
		root: Self::Root,
		sender: relm4::prelude::ComponentSender<Self>,
	) -> relm4::prelude::ComponentParts<Self> {
		let model = TodoApp { items: init };

		let add_button = gtk::Button::with_label("🆕"); // add the item to the list.
		let remove_button = gtk::Button::with_label("❌"); // remove the item from the list.
		let toggle_button = gtk::Button::with_label("🖲️"); // done to undone.

		add_button.connect_clicked(clone!(@strong sender => move |_| {
			sender.input(TodoInput::Add("New Item".to_string()));
		}));
		remove_button.connect_clicked(clone!(@strong sender => move |_| {
			// TODO: add the id explicitly.
			sender.input(TodoInput::Remove(0));
		}));
		toggle_button.connect_clicked(clone!(@strong sender => move |_| {
			// TODO: add the id explicitly.
			sender.input(TodoInput::Toggle(0));
		}));

		let item_name = gtk::Label::new(None);
		let item_done = gtk::Label::new(None);

		let vbox = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(10).build();
		root.set_child(Some(&vbox));

		let buttons = gtk::Box::builder()
			.orientation(gtk::Orientation::Horizontal)
			.spacing(10)
			.build();
		buttons.append(&add_button);
		buttons.append(&remove_button);
		buttons.append(&toggle_button);

		vbox.append(&buttons);
		vbox.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
		let item_hbox = gtk::Box::builder()
			.orientation(gtk::Orientation::Horizontal)
			.spacing(10)
			.build();
		item_hbox.append(&item_name);
		item_hbox.append(&item_done);
		vbox.append(&item_hbox);

		let widgets = AppWidgets { name: item_name, done: item_done };

		ComponentParts { model, widgets }
	}

	fn update(
		&mut self,
		message: Self::Input,
		_sender: relm4::prelude::ComponentSender<Self>,
		_root: &Self::Root,
	) {
		match message {
			TodoInput::Add(name) => {
				let id = self.items.len() as u32;
				self.items.push(TodoItem { id, name, done: false });
			},
			TodoInput::Remove(id) => {
				// remove the last one always
				self.items.retain(|item| item.id != id);
			},
			TodoInput::Toggle(id) => {
				if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
					item.done = !item.done;
				}
			},
		}
	}

	fn update_view(
		&self,
		widgets: &mut Self::Widgets,
		_sender: relm4::prelude::ComponentSender<Self>,
	) {
		let items = self
			.items
			.iter()
			.map(|item| {
				let done = if item.done { "Done" } else { "Not Done" };
				format!("{}: {}", item.name, done)
			})
			.collect::<Vec<_>>()
			.join("\n");

		widgets.name.set_text(&items);
		widgets.done.set_text("Done");

		// TODO: populate all the todo items where each todo item has "Remove" & toggle buttons...
		// inside a vertical scrollbar.
		// TODO: on remove, update the view.
		// TODO: on toggle, toggle the last item
	}
}

fn main() {
	let app = RelmApp::new("abhi.todo.app");
	app.run::<TodoApp>(vec![])
}
