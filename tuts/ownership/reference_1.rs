/*
    Learning
    - References
let x = 10;
let r = &x;

                   +–––––––+
                   │       │
            +–––+––V–+–––+–│–+–––+
stack frame │   │ 10 │   │ • │   │ 
            +–––+––––+–––+–––+–––+
                [––––]   [–––]
                  x        r

2. In Case-2, we find the problem of y not available for lifetime

3. Case-3: the a slice of `my_name` is referenced in `last_name`
                my_name       last_name
            [––––––––––––]    [–––––––]
            +–––+––––+––––+–––+–––+–––+
stack frame │ • │ 16 │ 13 │   │ • │ 6 │ 
            +–│–+––––+––––+–––+–│–+–––+
              │                 │
              │                 +–––––––––+
              │                           │
              │                           │
              │                         [–│––––––– str –––––––––]
            +–V–+–––+–––+–––+–––+–––+–––+–V–+–––+–––+–––+–––+–––+–––+–––+–––+
       heap │ P │ a │ s │ c │ a │ l │   │ P │ r │ e │ c │ h │ t │   │   │   │
            +–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+

SOURCE: https://blog.thoughtram.io/references-in-rust/
*/

pub fn run() {
    // Case-1
    let x = 10;
    let r = &x;

    // Case-2
    let k;
    {
        // let y = Box::new(5);            // Using Box pointer for storing into heap
        let y = 5;              // stored in stack for primitive variable
        // k = &y;         // y dropped here as it is not available for lifetime. Moreover the block is getting over after this
        k = y;          // this implies that the ownership of 5 is transferred to `k` from `y`
    }
    print!("{}", k);

    // Case-3
    let my_name = "Abhijit Roy".to_string();
    let last_name = &my_name[7..];

    println!("{}", last_name);

}