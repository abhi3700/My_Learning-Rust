//! Example for using a tiny circular progress bar with GTK4
//!
//! Circle filled with blue color initially is left with white color as progress reduces from 100%
//! to 0%.

use gtk::{glib, prelude::*, Application, ApplicationWindow, DrawingArea};
use std::{cell::RefCell, f64::consts::PI, rc::Rc};

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

	let drawing_area = DrawingArea::builder()
		.content_width(280)
		.content_height(280)
		.margin_top(10)
		.margin_bottom(10)
		.margin_start(10)
		.margin_end(10)
		.halign(gtk::Align::Center)
		.valign(gtk::Align::Center)
		.build();

	window.set_child(Some(&drawing_area));

	// Customize diameter to be very small
	let diameter = 20.0; // Small diameter for a tiny progress circle

	// Create a shared state for the progress
	let progress = Rc::new(RefCell::new(1.0)); // Start fully "unwiped"

	drawing_area.set_draw_func({
		let progress = progress.clone();
		move |_, cr, width, height| {
			let percentage = *progress.borrow();

			// Center coordinates
			let center_x = width as f64 / 2.0;
			let center_y = height as f64 / 2.0;

			// Draw the full circle in blue
			cr.set_source_rgb(0.1, 0.4, 0.8); // Deep blue for full circle
			cr.arc(center_x, center_y, diameter / 2.0, 0.0, 2.0 * PI);
			let _ = cr.fill();

			// Draw the sweeping arc in white
			cr.set_source_rgb(1.0, 1.0, 1.0); // White for sweeping arc
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

	// Update the progress every second
	glib::timeout_add_seconds_local(1, {
		let progress = progress.clone();
		move || {
			let mut percentage = progress.borrow_mut();
			if *percentage <= 0.0 {
				*percentage = 1.0; // Reset to full when it reaches 0
			} else {
				*percentage -= 0.1; // Decrease by 10%
			}
			drawing_area.queue_draw();
			glib::ControlFlow::Continue
		}
	});

	window.present();
}
