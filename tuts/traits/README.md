# Traits in Rust

## About

- <u>Definition</u>: A trait in Rust is a collection of methods that can be implemented by structs or other traits. Traits are similar to interfaces in other languages, but they can also provide `default` method implementations.
- In Rust, there is no concept of "inheriting" the properties of a struct. Instead, when you are designing the relationship between objects do it in a way that one's functionality is defined by an interface (a trait in Rust). This promotes **composition over inheritance**, which is considered more useful and easier to extend to larger projects.
- For defining attributes, use `struct`, `enum`. But, for behavior, define everything in `traits` like an `interface` i.e. w/o definition (in Solidity, Java).
- `trait` name is supposed to be written in CamelCase.
- Define `trait` like this:

```rs
<visibility: pub, priv> trait <TraitName> {
    ///
    /// method declaration
    ///
}
```

- Implementing `trait` for a `type` like this:

```rs
impl <TraitName> for <TypeName> {
    ///
    /// method definition
    ///
}
```

- Ideally, `struct` in C/C++ is same as here in Rust. But, in order to define methods we need to use `impl`s for them.
- `impl`s are used to define methods for Rust `struct`s and `enum`s.
- Traits are kind of similar to interfaces in **OOP** languages. They are used to define the functionality a type must provide. Multiple traits can be implemented for a single type.
- But traits can also include default implementations of methods. Default methods can be overridden when implementing types."
- A `trait` tells the Rust compiler about functionality a particular `type` has and can share with other `type`s. Traits are an abstract definition of shared behavior amongst different types. So, we can say that `trait`s are to **Rust** what `interface`s are to **Java** or abstract classes are to C++. A trait method is able to access other methods within that trait.
- **Analogy**

| Rust  | Java      | C++            | Solidity  |
| ----- | --------- | -------------- | --------- |
| trait | interface | abstract class | interface |
