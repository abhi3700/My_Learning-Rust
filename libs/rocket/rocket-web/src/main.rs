#![feature(decl_macro)]
use rocket::*;

use rocket::response::content::Json;

#[get("/hello")]
fn hello() -> Json<&'static str> {
    Json(
        "{
            'status': 'success',
            'message': 'Hello API!'
        }",
    )
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to Rocket Web framework!"
}

fn main() {
    rocket::ignite().mount("/", routes![index, hello]).launch();
}
