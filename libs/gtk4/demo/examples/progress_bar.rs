//! Example for using progress bar

use gtk::{
	glib::{self},
	prelude::*,
	Application, ApplicationWindow, ProgressBar,
};
use std::time::Duration;

const APP_ID: &str = "org.gtk_rs.ProgressBarExample";

fn main() -> glib::ExitCode {
	// Create a new application
	let app = Application::builder().application_id(APP_ID).build();

	// Connect to "activate" signal of `app`
	app.connect_activate(build_ui);

	// Run the application
	app.run()
}

fn build_ui(application: &Application) {
	// Create a progress bar
	let progress_bar = ProgressBar::new();

	// Create a window
	let window = ApplicationWindow::builder()
		.application(application)
		.title("My GTK App")
		.default_width(300)
		.default_height(100)
		.child(&progress_bar)
		.build();

	// Update the progress bar every second
	glib::timeout_add_local(Duration::from_secs(1), move || {
		let current_fraction = progress_bar.fraction();
		if current_fraction < 1.0 {
			// Increase the fraction
			progress_bar.set_fraction(current_fraction + 0.1);
		} else {
			// Reset the fraction
			progress_bar.set_fraction(0.0);
		}
		glib::ControlFlow::Continue
	});

	// Present the window
	window.present();
}
