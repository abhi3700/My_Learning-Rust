/*
   In Rust, you can use the Box<dyn Trait> syntax to create trait objects that can be used to
   store values of any type that implements a particular trait, without knowing the concrete type at compile time.

   Here's an example of how to use Box<dyn Trait> to create a trait object.

   Here, `MyStruct1` has implemented `Foo` trait. hence we can create boxed heap trait object
   (with `Foo` trait) for the structs which has implemented that trait

   Disadvantages:
   --------------
   Note that using Box<dyn Trait> to create trait objects comes with some overhead, as it requires dynamic dispatch to call methods on the underlying value.
   This can have performance implications, so you should use trait objects judiciously and consider other options if performance is a concern.
   Additionally, Box<dyn Trait> can only be used for single traits,
   so if you need to store values that implement multiple traits, you'll need to use other approaches such as tuple structs or enums.
*/

trait Foo {
    fn foo1(&self, item: String);
}

#[derive(Debug)]
struct MyStruct1 {}

impl Foo for MyStruct1 {
    fn foo1(&self, item: String) {
        println!("Hello {item} from {:?}", self);
    }
}

#[derive(Debug)]
struct MyStruct2 {}

impl Foo for MyStruct2 {
    fn foo1(&self, item: String) {
        println!("Hello {item} from {:?}", self);
    }
}

pub fn main() {
    let ms1 = MyStruct1 {};
    let mytrait: Box<dyn Foo> = Box::new(ms1);
    mytrait.foo1("Abhijit".to_string());

    let ms2 = MyStruct2 {};
    let mytrait2: Box<dyn Foo> = Box::new(ms2);
    mytrait2.foo1("Sajan".to_string());
}
