/*
    In this eg, home directory is displayed based on the system.

    So, wrap the home_path value as `Option`.

    NOTE: `dirs` dependency has been used in the dependencies.
    std::env::home_dir() is DEPRECATED.
*/

extern crate dirs;

pub fn run() {
    let home_path = dirs::home_dir();

    match home_path {
        Some(p) => println!("{:?}", p.display()),
        None => println!("Cannot find the home directory"),
    }
}
