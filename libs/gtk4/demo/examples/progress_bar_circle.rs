//! Example for using a circular progress bar with GTK4

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

	// Create a shared state for the progress
	let progress = Rc::new(RefCell::new(0.0));

	drawing_area.set_draw_func({
		let progress = progress.clone();
		move |_, cr, width, height| {
			let percentage = *progress.borrow();
			cr.set_line_width(10.0);
			cr.set_source_rgb(0.5, 0.5, 1.0);
			cr.arc(width as f64 / 2.0, height as f64 / 2.0, 100.0, 0.0, 2.0 * PI * percentage);
			let _ = cr.stroke();

			cr.set_font_size(24.0);
			cr.move_to(width as f64 / 2.0 - 18.0, height as f64 / 2.0 + 8.0);
			let _ = cr.show_text(&format!("{:.0}%", percentage * 100.0));
		}
	});

	// Update the progress every second
	glib::timeout_add_seconds_local(1, {
		let progress = progress.clone();
		move || {
			let mut percentage = progress.borrow_mut();
			if (*percentage + 0.1) >= 1.0 {
				*percentage = 0.0;
			} else {
				*percentage = (*percentage + 0.1) % 1.0;
			}
			println!("curr percent: {:.1}%", *percentage * 100.0);
			drawing_area.queue_draw();
			glib::ControlFlow::Continue
		}
	});

	window.present();
}
