# Struct

- There are different types of struct

  - normal struct: with parameters
  - unit struct: without parameters

- The purpose of unit struct is to implement traits on it and use that struct as value for the other traits' associated type during implementation. [Code example](./struct_11.rs).
- Structs can be inherited from other structs via declaring the 1 struct type as type of another struct's parameter.

  ```rust
  struct Purchase {
    bill: f64,
    tax: f64,
    shop_name: String,
    gst_no: String,
  }

  struct House {
    address: String,
    area: f64,
    price: f64,
    purchase: Purchase,
  }
  ```

- Inside a `impl` block, if there is a function without `self` (ref, mut ref, value) argument, then it is supposed to be called like this `Struct::function_name()`. Code example:

  ```rust
  struct Person {
    name: String,
    age: u8,
  }

  impl Person {
    fn foo() {
      // ==snip==
    }
  }

  fn main() {
    Person::foo();
  }
  ```

- Suppose a function name is repeated inside multiple traits implementation of a struct type, then the function can be called like this `Struct::trait_name::function_name()`. [Code example](./struct_10.rs).
