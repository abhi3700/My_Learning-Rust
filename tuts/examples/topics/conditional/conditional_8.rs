pub fn main() {
    let mut range = 10;
    let mut optional_integers: Vec<Option<i8>> = Vec::new();
    for i in 0..(range + 1) {
        optional_integers.push(Some(i));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while let Some(integer) = optional_integers.pop() {
        assert_eq!(integer.unwrap(), range);
        range -= 1;
    }
}
