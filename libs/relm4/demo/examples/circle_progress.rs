//! This example is inspired from "./progress.rs" that showed a progress bar/line.
//! But, this shows the progress in CW/ACW direction. To change the direction, you just need to
//! change the angle2. Follow the NOTE please.
//!
//! Workflow
//! ========
//! When the button is pressed, a `Compute` input is triggered that leads to rotation operation of
//! the progress bar. After each progress/arc making, another command `Progress(f64)` is emitted.
//! And that updates the Rc mutable value via `update_cmd` function, which in turn updates the
//! `update_view` function where both button & progress bar does its own operation irrespective of
//! any of the Command output in this case. Hence, the `.queue_draw()` function actually updates the
//! latest progress value.
//! And once, the arc progress is over i.e. moves out of while loop, then `Finished` command is
//! sent. That activates back the button.
//!
//! Now, again you can click the button & that would lead to progress bar moving in CW/ACW
//! direction.
//!
//! NOTE: We could also have dedicated operation based on each Command output emitted.

use futures::FutureExt;
use relm4::{
	gtk::{self, prelude::*, DrawingArea},
	Component, ComponentParts, ComponentSender, RelmApp,
};
use std::{cell::RefCell, f64::consts::PI, rc::Rc};

/// Create a circular progress bar
pub fn create_circular_progress_bar(
	diameter: f64,
	margin_top: i32,
	margin_bottom: i32,
	margin_start: i32,
	margin_end: i32,
	set_visible: bool,
	tooltip_text: &str,
	progress: Rc<RefCell<f64>>,
) -> DrawingArea {
	let drawing_area = DrawingArea::builder()
		.content_width((diameter + 1.0) as i32)
		.content_height((diameter + 1.0) as i32)
		.margin_top(margin_top)
		.margin_bottom(margin_bottom)
		.margin_start(margin_start)
		.margin_end(margin_end)
		.tooltip_text(tooltip_text)
		.visible(set_visible)
		.build();

	drawing_area.set_draw_func(move |_, cr, width, height| {
		let percentage = *progress.borrow();

		// Center coordinates
		let center_x = width as f64 / 2.0;
		let center_y = height as f64 / 2.0;

		// Draw the background circle with respective color in dark/light themes
		if matches!(dark_light::detect(), dark_light::Mode::Dark) {
			// Grey for dark theme
			cr.set_source_rgb(0.2078431373, 0.2078431373, 0.2078431373);
		} else {
			// White for light theme
			cr.set_source_rgb(1.0, 1.0, 1.0);
		}
		cr.arc(center_x, center_y, diameter / 2.0, 0.0, 2.0 * PI);
		// let _ = cr.fill();       // NOTE: Fill w/o border color
		let _ = cr.fill_preserve(); // Preserve the path for stroking
		cr.set_source_rgb(0.0, 0.0, 0.0); // Black border color for both the themes.
		cr.set_line_width(0.5); // Set the border width
		let _ = cr.stroke(); // Draw the circle border

		// Draw the sweeping with respective color in dark/light themes
		if matches!(dark_light::detect(), dark_light::Mode::Dark) {
			// White for dark theme
			cr.set_source_rgb(1.0, 1.0, 1.0);
		} else {
			// Grey for light theme
			cr.set_source_rgb(0.2078431373, 0.2078431373, 0.2078431373);
		}
		cr.arc_negative(
			center_x,
			center_y,
			diameter / 2.0,
			-PI / 2.0,
			// NOTE: CW
			// -PI / 2.0 - 2.0 * PI * percentage,
			// NOTE: ACW
			-PI / 2.0 + 2.0 * PI * percentage,
		);
		cr.line_to(center_x, center_y);
		let _ = cr.fill();
	});

	drawing_area
}

#[derive(Default)]
struct App {
	/// Tracks progress status
	computing: bool,
	/// Contains output of a completed task.
	task: Option<CmdOut>,
	/// Progress value
	progress: Rc<RefCell<f64>>,

	// progress bar widget
	progress_bar: DrawingArea,
}

#[derive(Debug)]
pub enum Input {
	Compute,
}

#[derive(Debug)]
pub enum CmdOut {
	/// Progress update from a command.
	Progress(f64),
	/// The final output of the command.
	Finished(Result<String, ()>),
}

#[relm4::component(pub)]
impl Component for App {
	type Init = String;
	type Input = Input;
	type Output = ();
	type CommandOutput = CmdOut;

	view! {
		#[root]
		gtk::Window{
			gtk::Box {
				set_halign: gtk::Align::Center,
				set_valign: gtk::Align::Center,
				set_width_request: 300,
				set_spacing: 12,
				set_margin_top: 4,
				set_margin_bottom: 4,
				set_margin_start: 12,
				set_margin_end: 12,
				set_orientation: gtk::Orientation::Horizontal,

				gtk::Box {
					set_spacing: 4,
					set_hexpand: true,
					set_valign: gtk::Align::Center,
					set_orientation: gtk::Orientation::Vertical,

					append: &model.progress_bar
				},

				#[name="button"]
				gtk::Button {
					set_label: "Compute",
					connect_clicked => Input::Compute,
				}
			}
		}
	}

	fn init(
		_args: Self::Init,
		root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let progress = Rc::new(RefCell::new(1.0));

		let model = Self {
			progress: progress.clone(),
			progress_bar: create_circular_progress_bar(
				20.0,
				10,
				10,
				10,
				10,
				true,
				"ETA for next reward payment",
				progress,
			),
			..App::default()
		};
		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
		match message {
			Input::Compute => {
				self.computing = true;
				sender.command(|out, shutdown| {
					shutdown
						// Performs this operation until a shutdown is triggered
						.register(async move {
							let mut percentage = 0.0;

							while percentage > 0.0 {
								out.send(CmdOut::Progress(percentage)).unwrap();
								percentage -= 0.1; // Decrease by 10%
								tokio::time::sleep(std::time::Duration::from_secs(1)).await;
							}

							out.send(CmdOut::Finished(Ok("42".into()))).unwrap();
						})
						// Perform task until a shutdown interrupts it
						.drop_on_shutdown()
						// Wrap into a `Pin<Box<Future>>` for return
						.boxed()
				});
			},
		}
	}

	fn update_cmd_with_view(
		&mut self,
		widgets: &mut Self::Widgets,
		message: Self::CommandOutput,
		_sender: ComponentSender<Self>,
		_root: &Self::Root,
	) {
		match message {
			CmdOut::Progress(p) => {
				*self.progress.borrow_mut() = p;
			},
			CmdOut::Finished(_) => {
				self.computing = false;
			},
		}

		self.task = Some(message);

		// NOTE: Previously it was inside `update_view` fn.
		self.progress_bar.queue_draw();
		widgets.button.set_sensitive(!self.computing);
	}
}

fn main() {
	RelmApp::new("abhi.circle.progress.bar").run::<App>("Demo".to_string());
}
