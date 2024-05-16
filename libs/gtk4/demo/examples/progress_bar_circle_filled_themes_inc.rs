//! Example for using a tiny circular progress bar with GTK4
//!
//! Also learned about setting dark theme in cross-platform way.
//!
//! Circle filled with black color initially is sweeped with arc filled white color as progress
//! reduces from 100% to 0% in clockwise direction.
//!
//! ## Params
//! - diameter: diameter of the circle
//! - progress: 1.0 = 100%, 0.0 = 0%
//! - color: 100% color = black. sweeping color = white
//! - content_sizes (height, width) should be at least > the set diameter
//! - margin_top, margin_bottom, margin_start, margin_end
//! - halign, valign
//! - sweep interval (in seconds) i.e. how often the progress is updated?
//!
//! ## Images
//! - img/gtk_progress_bar_circle_filled_1.png
//! - img/gtk_progress_bar_circle_filled_2.png
//! - img/gtk_progress_bar_circle_filled_3.png

use gtk::{glib, prelude::*, Application, ApplicationWindow, DrawingArea};
use std::{cell::RefCell, f64::consts::PI, rc::Rc};

const APP_ID: &str = "org.gtk_rs.CircularProgressBar";

fn main() -> glib::ExitCode {
	let app = Application::builder().application_id(APP_ID).build();
	app.connect_activate(build_ui);

	// Prefer dark theme in cross-platform way if environment is configured that way
	// else by default, app uses light theme.
	let _ = gtk::init(); // has to be initialized first.
	if let Some(settings) = gtk::Settings::default() {
		if matches!(dark_light::detect(), dark_light::Mode::Dark) {
			settings.set_gtk_application_prefer_dark_theme(true);
		}
	}

	app.run()
}

fn build_ui(application: &Application) {
	// Create a shared state for the progress
	let progress = Rc::new(RefCell::new(1.0)); // Start fully "unwiped"

	let drawing_area = create_circular_progress_bar(20.0);
	let window = ApplicationWindow::builder()
		.application(application)
		.title("My GTK App")
		// OPTIONAL (both height & width). It's automatically going to take the size of set circular
		// diameter & some more space.
		.default_width(300)
		.default_height(300)
		.child(&drawing_area)
		.build();

	// Update the progress i.e. sweep arc every `interval` second
	glib::timeout_add_seconds_local(1, {
		let progress = progress.clone();
		move || {
			let mut percentage = progress.borrow_mut();
			if *percentage <= 0.0 {
				*percentage = 1.0; // Reset to full when it reaches 0
			} else {
				*percentage -= 0.1; // Decrease by 10%
				println!("Percentage = {}", *percentage);
			}
			drawing_area.queue_draw();
			glib::ControlFlow::Continue
		}
	});

	window.present();
}

/// @diameter diameter for a progress circle
pub fn create_circular_progress_bar(diameter: f64) -> DrawingArea {
	let drawing_area = Rc::new(RefCell::new(
		DrawingArea::builder()
			.content_width((diameter + 1.0) as i32)
			.content_height((diameter + 1.0) as i32)
			.margin_top(10)
			.margin_bottom(10)
			.margin_start(10)
			.margin_end(10)
			.tooltip_text("ETA for next reward payment")
			.build(),
	));

	// Create a shared state for the progress
	let progress = Rc::new(RefCell::new(1.0)); // Start fully "unwiped"
	drawing_area.borrow().set_draw_func({
		let progress = progress.clone();
		move |_, cr, width, height| {
			let percentage = *progress.borrow();

			// Center coordinates
			let center_x = width as f64 / 2.0;
			let center_y = height as f64 / 2.0;

			// Draw the full circle with respective color in dark/light themes
			if matches!(dark_light::detect(), dark_light::Mode::Dark) {
				// Grey for dark theme
				cr.set_source_rgb(0.2078431373, 0.2078431373, 0.2078431373);
			} else {
				// White for light theme
				cr.set_source_rgb(1.0, 1.0, 1.0); // White for full circle
			}
			cr.arc(center_x, center_y, diameter / 2.0, 0.0, 2.0 * PI);
			/* M-1: w/o border color */
			// let _ = cr.fill();

			/* M-2: with border color */
			let _ = cr.fill_preserve(); // Preserve the path for stroking
			cr.set_source_rgb(0.0, 0.0, 0.0); // Black for the border
			cr.set_line_width(0.5); // Set the border width
			let _ = cr.stroke(); // Draw the border

			/* ====================================== */

			// Draw the sweeping with respective color in dark/light themes
			if matches!(dark_light::detect(), dark_light::Mode::Dark) {
				// White for dark theme
				cr.set_source_rgb(1.0, 1.0, 1.0);
			} else {
				// Grey for light theme
				cr.set_source_rgb(0.2078431373, 0.2078431373, 0.2078431373);
			}
			cr.arc(
				center_x,
				center_y,
				diameter / 2.0,
				-PI / 2.0,
				-PI / 2.0 + 2.0 * PI * (1.0 - percentage),
			);
			cr.line_to(center_x, center_y);
			let _ = cr.fill();
		}
	});

	// Update the progress i.e. sweep arc every `interval` second
	glib::timeout_add_seconds_local(1, {
		let progress = progress.clone();
		let drawing_area = drawing_area.clone();
		move || {
			let mut percentage = progress.borrow_mut();
			if *percentage <= 0.0 {
				*percentage = 1.0; // Reset to full when it reaches 0
			} else {
				*percentage -= 0.1; // Decrease by 10%
			}
			drawing_area.borrow().queue_draw();
			glib::ControlFlow::Continue
		}
	});

	let progress_bar = drawing_area.borrow().clone();
	progress_bar
}
