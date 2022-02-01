mod input;

fn main() {
    input::run();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_guess_num() {
        let num = 3;
        assert_eq!(num, 3);
    }
}