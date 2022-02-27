struct Film {
    title: String,
    release_year: u16,
    genre: String
}

struct Book {
    title: String,
    release_year: u16,
    genre: String,
    is_filmed: bool
}

trait Description {
    fn describe(&self);
}

impl<T> Description for T where T: Film, Book {
    fn describe(&self) {
        println!("title: {}", self.title);
        println!("release year: {}", self.release_year);
        println!("title: {}", self.title);
        println!("title: {}", self.title);
    } 
}


fn run() {
    
}