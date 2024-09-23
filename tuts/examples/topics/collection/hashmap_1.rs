use std::collections::HashMap;

pub fn run() {
    let mut h = HashMap::new();

    // insert k, v
    h.insert(
        "name".to_string(),
        "Abhijit Roy".to_string()
    );
    
    h.insert("age".to_string(), "18".to_string());

    println!("{:?}", h);

    for i in h {
        println!("{}, {}", i.0, i.1);
    }

}