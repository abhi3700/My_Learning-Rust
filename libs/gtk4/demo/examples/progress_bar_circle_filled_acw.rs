//! Example for using a circular progress bar with GTK4
//!
//! Also learned about setting dark theme in cross-platform way.
//!
//! Circle filled with black color initially is sweeped with arc filled white color as progress
//! reduces from 100% to 0% in anti-clockwise direction.
//!
//! ## Params
//!
//! - diameter: diameter of the circle
//! - progress: 1.0 = 100%, 0.0 = 0%
//! - color: 100% color = black. sweeping color = white
//! - content_sizes (height, width) should be at least > the set diameter
//! - margin_top, margin_bottom, margin_start, margin_end
//! - halign, valign
//! - sweep interval (in seconds) i.e. how often the progress is updated?
//!
//! ## Images
//!
//! - img/gtk_progress_bar_circle_filled_cw_1.png
//! - img/gtk_progress_bar_circle_filled_cw_2.png
//! - img/gtk_progress_bar_circle_filled_cw_3.png
#[path = "./widgets/progress_bar.rs"]
pub mod progress_bar;

use gtk::{glib, prelude::*, Application, ApplicationWindow};
use progress_bar::create_circular_progress_bar_acw;

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

	let progress_bar = create_circular_progress_bar_acw(20.0, 10, 10, 10, 10, "Test...");
	window.set_child(Some(&progress_bar));
	window.present();
}
