// The opposite case of unwrap() 

pub fn run() {
    let o: Result<i8, &str> = Ok(10);
    o.unwrap_err();
}