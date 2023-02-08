/*

    1. immutably borrow a struct object
    2. make changes to the object in a borrowed state via borrower using dereferencing (*)

    Changelog: Compared to the previous example, scope is defined using a block {}

    NOTE: Don't use the original object unless lifetime of the borrower is over.

    Lifetime is over by 2 ways:
        - usage
        - block
*/

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

pub fn main() {
    let mut p = Person {
        name: "Abhijit".to_string(),
        age: 29,
    };
    println!("{:?}", p);

    let p2 = &mut p;
    {
        // M-1
        // *p2 = Person {
        //     name: "Sumit".to_string(),
        //     age: 30,
        // };

        // M-2: allows me to independently change the struct variable.
        (*p2).name = "Sumit".to_string();
        (*p2).age = 30;

        // Printing the value of `p2` using the `Debug` trait.
        // println!("{:?}", p); // Err: the value of which is already in a borrowed state unless the lifetime of 'p2' is finished.
        println!("{:?}", p2); // should give the changed value unless the original variable called in between (borrowed state).
    }

    p = Person {
        name: "Alice".to_string(),
        age: 60,
    };
    println!("{:?}", p);
}
