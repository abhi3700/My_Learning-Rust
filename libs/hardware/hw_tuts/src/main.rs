fn main() {
    println!(
        "The no. of available cpus in this hardware: {}",
        num_cpus::get()
    );

    println!("Number of physical cores in this hardware: {}", num_cpus::get_physical());
}
