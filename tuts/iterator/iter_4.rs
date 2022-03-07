/* 
    Implement iter for struct
*/

struct Counter {
    count: usize
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;        // make the count to 0

        if self.count < 6 {
            Some(self.count)   
        } else {
            None
        }
    }
}


pub fn run() {
    let mut c = Counter::new();

    assert_eq!(c.next(), Some(1));
    assert_eq!(c.next(), Some(2));
    assert_eq!(c.next(), Some(3));
    assert_eq!(c.next(), Some(4));
    assert_eq!(c.next(), Some(5));
    assert_eq!(c.next(), None);

}