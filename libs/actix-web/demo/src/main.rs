// TODO: We need to spin up multiple servers with individual app instance.

#[path = "./l5_app_state_shared_globally.rs"]
mod l5_app_state_shared_globally;

use std::net::TcpListener;

pub(crate) fn is_port_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn main() {
    l5_app_state_shared_globally::main();
}
