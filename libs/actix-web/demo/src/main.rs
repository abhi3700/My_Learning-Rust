// TODO: We need to spin up multiple servers with individual app instance.

#[path = "./l4_app_state_outside.rs"]
mod l4_app_state_outside;

use std::net::TcpListener;

pub(crate) fn is_port_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn main() {
    l4_app_state_outside::main();
}
