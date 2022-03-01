/* 
    Many shared smart pointer types, including Rc<T> and Arc<T>, provide containers 
    that can be cloned and shared between multiple parties.

    Here, the `shared_map` is 
*/

use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

pub fn run() {
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));

    // create a new block to limit the scope of the dynamic borrow
    {
        let mut map: RefMut<_> = shared_map.borrow_mut();
        map.insert("Abhijit", 34573);
        map.insert("Michael", 467);
        map.insert("Gary", 890);
        map.insert("Bob", 2576);
    }
    let tot: u32 = shared_map.borrow().values().sum();

    println!("{:?}", shared_map.borrow().keys());           // ["Bob", "Michael", "Abhijit", "Gary"]
    println!("{:?}", shared_map.borrow().values());         // [467, 2576, 890, 34573]
    println!("{:?}", shared_map);                           // RefCell { value: {"Bob": 2576, "Michael": 467, "Abhijit": 34573, "Gary": 890} }
    println!("{:?}", tot);                                  // 38506
}