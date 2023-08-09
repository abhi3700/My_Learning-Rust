mod type_struct;
use type_struct::*;

mod type_num;
use type_num::*;

use redis::{Client, Connection};

// Connect to database
pub(crate) fn init() -> Result<Connection, String> {
    dotenv::from_path("./.env").expect("Failed to load the env file");

    let redis_url = std::env::var("REDIS_URL").expect("Invalid REDIS_URL");

    // connect to redis
    let client = Client::open(redis_url).expect("Parse the correct Redis URL");
    let con = client
        .get_connection()
        .expect("Failed when connecting to the Redis URL");

    Ok(con)
}

fn main() {
    let con = &mut init().expect("Database initialization failed");

    let x = fetch_an_integer(con).unwrap();
    println!("{}", x);

    let p1 = fetch_person_struct(con).unwrap();
    println!("{:?}", p1);
}
