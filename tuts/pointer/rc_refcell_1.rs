//! Many shared smart pointer types, including Rc<T> and Arc<T>, provide containers
//! that can be cloned and shared between multiple parties.
//!
//! Here, the `shared_map` is being mutably borrowed by a new block. And this
//! is only possible because of [`RefCell`] which allows a container to be mutably
//! borrowed once at a time. So, basically, when `shared_map` is mutably borrowed
//! inside the new block, then it returns a [`RefMut`] type which means that `shared_map`
//! can be mutated with new (key, value) pairs.

use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    // create a shared map using an Rc and a RefCell
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));

    // create a new block to limit the scope of the dynamic borrow
    {
        // obtain a mutable reference to the map inside the RefCell
        let mut map: RefMut<_> = shared_map.borrow_mut();
        // insert some key-value pairs into the map
        map.insert("Abhijit", 34573);
        map.insert("Michael", 467);
        map.insert("Gary", 890);
        map.insert("Bob", 2576);
    }

    // calculate the sum of the values in the map
    let tot: u32 = shared_map.borrow().values().sum();

    // print the keys and values in the map
    println!("{:?}", shared_map.borrow().keys()); // ["Bob", "Michael", "Abhijit", "Gary"]
    println!("{:?}", shared_map.borrow().values()); // [467, 2576, 890, 34573]

    // print the shared map itself
    println!("{:?}", shared_map); // RefCell { value: {"Bob": 2576, "Michael": 467, "Abhijit": 34573, "Gary": 890} }

    // print the sum of the values in the map
    println!("{:?}", tot); // 38506
}
