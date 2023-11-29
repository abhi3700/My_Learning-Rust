/*
   Another use of `impl Trait` as return type in a function.

   Here, the function returns anything that returns iterator with associated type `u32`.

   Here, the good part is we don't need to know the output type. Hence, whenever the function signature changes,
   we don't need to do anything here.

   Then, we use the function & show as output in looping & printing the element.

   However, there are some limitations to using `impl Trait`. As I mentioned earlier,
   you can only use it as a return type for a single concrete function, and
   you can't use it as a generic type parameter.
   This is because the compiler needs to know the concrete type of the returned value in order to generate code that works with that value.

   These code snippets won't work ❌
   ```rust
   fn foo<T: impl Boo>(item: T) {

   }
   ```

   OR

   ```rust
   fn foo<T: impl Boo>(item: String) -> T {

   }
   ```
   ==================================================
   error: expected trait bound, found `impl Trait` type
   label: not a trait
   fix: use the trait bounds directly
   ==================================================

    Additionally, using `impl Trait` as a return type can sometimes make your code less readable or harder to understand.
    This is especially true if the constraint on the returned type is complex or not immediately obvious from the context.

    Overall, `impl Trait` is a powerful feature in Rust that can make your code more flexible and easier to maintain,
    but it should be used judiciously and with careful consideration of the trade-offs involved.
*/

fn foo() -> impl IntoIterator<Item = u32> {
    vec![1, 5, 7].into_iter()
}

pub fn main() {
    let v = foo();
    for i in v {
        println!("{}", i);
    }
}
