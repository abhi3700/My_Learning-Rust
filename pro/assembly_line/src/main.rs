#[allow(unused)]

#[path = "./assembly_line.rs"]
mod assembly_line;

fn main() {
    println!("{}", assembly_line::production_rate_per_hour(1));
}