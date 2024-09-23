use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // get current time in seconds since UNIX_EPOCH
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("{}", now);
}
