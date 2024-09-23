pub fn say_hello() -> String {
    "Hello, World!".to_string()
}

// use crate::hello::say_hello;
use crate::hello::say_hello;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(say_hello().to_string(), "Hello, World!");
    }
}
