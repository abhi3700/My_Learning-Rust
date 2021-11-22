/*
	Borrow Checker
	- This means a var can be borrowed in terms of ownership right and its use. But, it is returned back before the main function is fully executed.
	- When a function calls a var or the var is copied via equal_to sign, then the data is moved from one to the other.
	- But, in order to avoid this or make a copy, one can use either of these two methods:
		1. Use `v.clone()`
		2. Use `&v`

	> NOTE: Using of '&' in Rust is for copying/borrow a var, but in C++ it's using the original var & not making a copy. 
		Rather in C++, var without '&' when called, then it is copied into stack.

	So, here we apply both the garbage collector feature (i.e. automatic memory handling feature) as the `hold_my_vec` function 
		is going to give back the borrowed vector after its use. Although here the function is making no use as it is empty.
		
		And we also have a control on manual memory management. A programmer can use the '&' sign whenever needed.

	REFERENCE: https://blog.logrocket.com/introducing-the-rust-borrow-checker/
*/


// fn hold_my_vec<T>(_: Vec<T>) {}				// M-1
fn hold_my_vec<T>(_: &Vec<T>) {}			// M-2 (RECOMMENDED)


fn main() {
    let v = vec![2, 3, 5, 7, 11, 13, 17];

    // hold_my_vec(v.clone());					// M-1
    hold_my_vec(&v);						// M-2 (RECOMMENDED) 

    let element = v.get(3);

    println!("I got this element from the vector: {:?}", element);
}