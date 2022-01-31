fn do_something() {
    println!("Please do something!");
}

fn main() {
    do_something();
}


// Unit tests
#[cfg(test)]
mod tests {
    use crate::do_something;
    #[test]
    fn it_tests_do_something() {
        do_something();
    }
}
