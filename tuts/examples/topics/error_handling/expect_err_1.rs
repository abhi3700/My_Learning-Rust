// The opposite case of expect() 

pub fn run() {
    let o: Result<i8, &str> = Ok(10);
    o.expect_err("error message");
}