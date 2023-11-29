// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!

// Define an enum type named "Message" with four variants.
#[derive(Debug)]
enum Message {
    Move { x: u32, y: u32 },
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

impl Message {
    // Define a method named "call" for the "Message" type.
    fn call(&self) {
        // Print the debug representation of this "Message".
        println!("{:?}", &self);
    }
}

fn main() {
    // Create an array of "Message" instances.
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    // Loop over each "Message" instance in the "messages" array.
    for message in &messages {
        // Call the "call" method on each "Message" instance.
        message.call();
    }
}
