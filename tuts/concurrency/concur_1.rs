use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

pub(crate) fn main() {
    let future = hello_world(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
}
