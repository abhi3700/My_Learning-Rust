//! Example for using a circular progress bar with GTK4

#[path = "./widgets/progress_bar.rs"]
pub mod progress_bar;

use gtk::{glib, prelude::*, Application, ApplicationWindow};
use progress_bar::create_circular_progress_bar_cw;

const APP_ID: &str = "org.gtk_rs.CircularProgressBar";

fn main() -> glib::ExitCode {
	let app = Application::builder().application_id(APP_ID).build();
	app.connect_activate(build_ui);
	app.run()
}

fn build_ui(application: &Application) {
	let window = ApplicationWindow::builder()
		.application(application)
		.title("My GTK App")
		.default_width(300)
		.default_height(300)
		.build();

	let progress_bar = create_circular_progress_bar_cw(20.0, 10, 10, 10, 10, "Test...");
	window.set_child(Some(&progress_bar));
	window.present();
}
