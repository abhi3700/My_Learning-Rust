//! Demonstrate spawn blocking for a sync task
//! in main thread

#[tokio::main(flavor = "current_thread")]
async fn main() -> eyre::Result<()> {
	tokio::spawn(async {
		// Async task 1
		tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
		println!("Async task 1 done");
	})
	.await?;

	tokio::spawn(async {
		// Async task 2
		tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
		println!("Async task 2 done");
	})
	.await?;

	let sync_task = || {
		// Some synchronous, CPU-bound work
		std::thread::sleep(std::time::Duration::from_secs(1)); // Simulating work
		println!("Synchronous task done");
	};

	// Execute sync_task in a way that doesn't block the async runtime
	tokio::task::spawn_blocking(sync_task).await?;

	println!("Main task done");

	Ok(())
}
