//! Rust By Practice
//! ================
//! https://practice.rs/generics-traits/generics.html#functions-1

// Fill in the blanks to make it work
struct A; // Concrete type `A`.
struct S(A); // Concrete type `S`.
struct SGen<T>(T); // Generic type `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

pub fn main() {
    let a = A;
    let s = S(A);
    let sgen = SGen::<A>(A);
    let si32 = SGen::<i32>(5);
    let schar: SGen<char> = SGen::<char>('a');
    // Using the non-generic functions
    // DONE:
    reg_fn(s); // Concrete type.

    // DONE:
    gen_spec_t(sgen); // Implicitly specified type parameter `A`.

    // DONE:
    gen_spec_i32(si32); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    // DONE:
    generic::<char>(schar);

    // Implicitly specified type parameter `char` to `generic()`.
    let sgen2 = SGen::<A>(A);

    // DONE:
    generic(sgen2);

    println!("Success!");
}
