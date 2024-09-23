/* 
    While loop
*/

fn main() {
    let mut x = 0;
    while x <= 100 {
        if x % 15 == 0 {
            println!("YES-15, {}", x);
        } else if x % 7 == 0{
            println!("YES-7, {}", x);
        }
        x += 1;

    }
}