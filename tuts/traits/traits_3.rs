/*
    Similarly, implement human for Human & Speaks traits types.

    Output:
        The Rakesh speaks Oye!
        The Sunita speaks Haa ji!    

*/

trait Speaks {
    fn speak(&self);
}

trait Human {
    fn name(&self) -> &str;
    fn sentence(&self) -> &str;
}

impl<T> Speaks for T where T: Human {
    fn speak(&self) {
        println!("The {} speaks {}", self.name(), self.sentence());
    }
}

struct Man {}
struct Woman {}

impl Human for Man {
    fn name(&self) -> &str {
        "Rakesh"
    }

    fn sentence(&self) -> &str {
        "Oye!"
    }
}

impl Human for Woman {
    fn name(&self) -> &str {
        "Sunita"
    }

    fn sentence(&self) -> &str {
        "Haa ji!"
    }
}


fn main() {
    let man = Man {};
    man.speak();

    let woman = Woman {};
    woman.speak();
}