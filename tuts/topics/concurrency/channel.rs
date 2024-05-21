//! Open a channel with a tx and rx end.
//! Send a message from tx to rx.
//! Print the message received by rx.

use std::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
	let (tx, rx) = mpsc::channel();
	tokio::spawn(async move {
		println!("Sending..pls wait ⏰...");
		sleep(Duration::from_secs(5)).await;
		tx.send("Hello, World!").unwrap();
	});
	println!("Received: {}", rx.recv().unwrap());
}
