# C++ vs Rust

## 1. Get size of array in bytes

```cpp
//C++ code
#include <iostream>

using namespace std;

//Looks can be deceiving: arr is not a pointer
//to an array of 5 integers. It has decayed to
//a pointer to an integer.
void print_array_size(int (*arr)[5]) {
    //prints 8 (the size of a pointer)
    cout << "Array size in print_array_size function: " << sizeof(arr) << endl;
}

int main()
{
    int arr[5] = {1, 2, 3, 4, 5};
    //prints 20 (size of 5 4-byte integers)
    cout << "Array size in main function: " << sizeof(arr) << endl;
    print_array_size(&arr);
    return 0;
}
```

Same code in Rust:

```rust
use std::mem::size_of_val;

fn print_array_size(arr: [i32; 5]) {
    //prints 20
    println!("Array size in print_array_size function: {}", size_of_val(&arr));
}

fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    //print 20
    println!("Array size in main function: {}", size_of_val(&arr));
    print_array_size(arr);
}
```

> Rust's strictness also prevents problems like array to pointer decay in C/C++. In C++, the **size of the array** is the **size of the pointer**, not the **size of the array**. This is because the array decays to a pointer when passed to a function. In Rust, the **size of the array** is always the **size of the array**.
>
> Hence, It throws different values in C++. In Rust, the **size of the array** is always the **size of the array**.

## 2. Get element at index of array

```cpp
#include <iostream>

using namespace std;

int main()
{
    int arr[3] = {1, 2, 3};
    const auto index = 5;
    //arr[index] is undefined behaviour
    cout << "Integer at index " << index << ": " << arr[index] << endl;
    return 0;
}
```

Same code in Rust:

```rust
fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    let index = 5;
    //arr[index] panics with the following message:
    //index out of bounds: the len is 3 but the index is 5
    println!("Integer at index {}: {}", index, arr[index]);
}
```

It throws an error in Rust. In C++, it is undefined behaviour. So, hence Rust is safer than C++.

## Conclusion

- Rust is a **safer** language than C++.
