#[tokio::main]
pub async fn main() {
	let f = my_function();
	println!("Let's get Rusty");
	f.await;
}

async fn my_function() {
	println!("I'm a async function");
	let s1 = read_from_database().await;
	println!("{}", s1);
	let s2 = read_from_database().await;
	println!("{}", s2);
}

async fn read_from_database() -> String {
	"DB read".to_string()
}
