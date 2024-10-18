/* use std::{
	future::{self, Future},
	io::Write,
	net::TcpStream,
	str::FromStr,
};

fn main() {
	// === M-1
	let x = TcpStream::connect("127.0.0.1");
	let y = TcpStream::connect("127.0.0.1");
	x.write("foobar");
	y.write("foobaz");

	x.read();
	y.read();

	// === M-2
	let fut_x = TcpStream::connect("127.0.0.1")
		.and_then(|c| c.write_all("foobar"))
		.and_then(|c| c.read())
		.and_then(|c, b| b == "barfoo");
	println!("{:?}", fut_x);

	let fut_y = TcpStream::connect("127.0.0.1")
		.and_then(|c| c.write_all("foobaz"))
		.and_then(|c| c.read())
		.and_then(|c, b| b == "bazfoo");
	println!("{:?}", fut_y);

	let a: Executor;
	let x = a.run(fut_x);
	let y = a.run(fut_y);

	a.spawn(fut_x.and_then(|eq| assert!(eq)));
	a.spawn(fut_y.and_then(|eq| assert!(eq)));
	a.run_all(vec![x, y]);
}

struct Executor;

impl Executor {
	fn run_all(
		&mut self,
		futures: Vec<Future>,
	) -> Vec<(usize, Result<Future::Item, Future::Error>)> {
		let mut done = 0;
		let mut results = Vec::with_capacity(futures.len());

		while done != futures.len() {
			for (i, f) in futures.iter_mut().enumerate() {
				match f.poll() {
					Ok(Async::Ready(t)) => {
						results[i] = Ok(t);
						done += 1;
					}
					Err(e) => {
						results[i] = Err(e);
						done += 1;
					}
					Ok(Async::NotReady) => {
						// continue polling until we get the results
						continue;
					}
				}
			}
		}
	}
}

enum Async<T: FromStr> {
	Ready(T),
	NotReady,
}

 */

use std::{future::Future, task::Poll};
struct MyFuture;

impl Future for MyFuture {
	type Output = i32;

	fn poll(
		self: std::pin::Pin<&mut Self>,
		cx: &mut std::task::Context<'_>,
	) -> std::task::Poll<Self::Output> {
		Poll::Ready(42)
	}
}

#[tokio::main]
async fn main() {
	let my_future = MyFuture;

	let x = my_future.await;
	println!("{}", x);
}
