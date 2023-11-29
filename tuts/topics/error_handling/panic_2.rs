/* 
  panic macro supports println!() i.e. able to print the panic message in console.

  The next lines then won't be reachable. In this case add allow macro
*/

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8
}

#[allow(unreachable_code)]
pub fn run() {
    let some_color = Color {
        r: 28,
        g: 78,
        b: 90
    };

    panic!("Color {:?}", some_color);

    println!("{:?}", some_color);
}