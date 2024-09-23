/* 
    assert_eq
    - check the left & right side matches, otherwise it panics
    - Add a custom message
*/

pub fn run() {
    let a = 5;
    let b = 6;
    let c = 40;
    // assert_eq!(a+b, c, "{} is not equal with value: {} + {}", c, a, b);
    assert_eq!(a+b, c, "{c} is not equal with value: {a} + {b}");           // [RECOMMENDED]
}