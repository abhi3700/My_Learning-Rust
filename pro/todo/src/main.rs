use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
struct Todo {
    items: HashMap<String, String>,
}

impl Todo {
    fn new() -> Todo {
        Todo {
            items: HashMap::new(),
        }
    }

    fn add(&mut self, key: String, value: String) {
        self.items.entry(key).or_insert(value);
    }

    fn update(&mut self, key: String, value: String) {
        self.items.insert(key, value);
    }

    fn remove(&mut self, key: String) {
        self.items.remove(&key);
    }

    fn list(&self) {
        println!("Todo List:");
        for (key, value) in &self.items {
            println!("{}: {}", key, value);
        }
    }

    fn get(&self, key: String) -> Option<&String> {
        self.items.get(&key)
    }
}

fn main() {
    let mut todo: Todo = Todo::new();

    todo.add("1343".to_string(), "Alice".to_string());
    todo.add("2434".to_string(), "Bob".to_string());
    todo.add("35435".to_string(), "Charlie".to_string());
    todo.remove("2434".to_string());
    todo.add("7979".to_string(), "Den".to_string());
    todo.update("7979".to_string(), "Denis".to_string());

    println!("Total todo list count: {}\n---", todo.items.len());
    todo.list();
    println!(
        "--\nGet value of a key: {}",
        todo.get("35435".to_string()).unwrap()
    );
}
