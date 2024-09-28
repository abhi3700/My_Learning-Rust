//! Job scheduler
//!
//! It is absolutely possible to run multiple Cron jobs on the same server using the
//! `tokio-cron-scheduler` crate. You can define and schedule multiple jobs with different Cron
//! expressions and tasks within the same JobScheduler instance. This setup allows you to manage
//! multiple scheduled tasks independently, all running concurrently within the same server context.
//!
//! Here’s an example of how you can configure two Cron jobs running on the same server using
//! `tokio-cron-scheduler` along with the Axum web server.
//!
//! Cron expressions:
//! - "1/10 * * * * *" - Runs every 10 seconds
//! - "0 0 */3 * * ": Runs every 3 days at midnight (sometimes in Unix, you omit the second field).
//! - "0 0 * Jan-Apr Mon,Wed,Fri": Runs every day at midnight from January to April on Monday,
//!   Wednesday, and Friday.

/*
* * * * * *
| | | | | |
| | | | | └─── day of week (0 - 7) (Sunday to Saturday; 7 is also Sunday)
| | | | └───── month (1 - 12)
| | | └─────── day of month (1 - 31)
| | └───────── hour (0 - 23)
| └─────────── minute (0 - 59)
└───────────── second (optional, 0 - 59)
*/

use std::sync::Arc;

use axum::Router;
use tokio_cron_scheduler::{Job, JobScheduler};

async fn root() -> &'static str {
	"Hello, world!"
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> eyre::Result<()> {
	tracing_subscriber::fmt::init();

	dotenv::dotenv().ok();

	let port = std::env::var("PORT").expect("PORT is not set");
	let app = Router::new().route("/", axum::routing::get(root));

	// Create a shared JobScheduler instance
	let sched = JobScheduler::new().await?;
	let sched = Arc::new(sched);

	sched
		.add(Job::new("1/10 * * * * *", |_uuid, _l| {
			println!("I run every 10 seconds");
		})?)
		.await?;

	sched
		.add(Job::new_async("every 1 minutes", move |_uuid, _l| {
			Box::pin(async move {
				println!("I run every 1 minutes");
				let _ = perform_task_1().await;
			})
		})?)
		.await?;

	tokio::spawn(async move {
		tracing::info!("Starting scheduler");
		let _ = sched.start().await;
	});

	let socket_addr = &format!("0.0.0.0:{}", port);
	let listener = tokio::net::TcpListener::bind(socket_addr).await?;
	tracing::info!("Listening on {}", socket_addr);

	/* Start the server */
	axum::serve(listener, app).await?;

	Ok(())
}

async fn perform_task_1() -> eyre::Result<()> {
	tracing::info!("Performing task 1");
	tokio::time::sleep(std::time::Duration::from_secs(30)).await;
	tracing::info!("Task 1 completed");

	Ok(())
}

async fn perform_task_2() -> eyre::Result<()> {
	tracing::info!("Performing task 2");
	tokio::time::sleep(std::time::Duration::from_secs(10)).await;
	tracing::info!("Task 2 completed");

	Ok(())
}
