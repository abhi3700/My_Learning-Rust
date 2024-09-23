
trait Computer {
    fn get_answer(&self) -> u8;
}
struct DeepThought;

impl Computer for DeepThought {
    fn get_answer(&self) -> u8 {
        48
    }
}

pub fn run() {
    let d = DeepThought{};
    println!("{}", d.get_answer());

}