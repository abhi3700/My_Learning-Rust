//! KeyValue Store
//! Implement a generic KeyValue struct that can store a key of type K and a value of type V.
//! Implement a method new to create a new KeyValue, and a method get_value to retrieve the value.

struct KeyValue<K, V> {
    key: K,
    value: V,
}

impl<K, V> KeyValue<K, V> {
    // Implement methods
    fn new(k: K, v: V) -> Self {
        Self { key: k, value: v }
    }

    fn get_value(&self) -> &V {
        &self.value
    }
}

fn main() {
    let kv = KeyValue::new(2, "abhi");
    assert_eq!(kv.get_value(), &"abhi");

    let kv = KeyValue::new("rock", 56);
    assert_eq!(kv.get_value(), &56);
}
