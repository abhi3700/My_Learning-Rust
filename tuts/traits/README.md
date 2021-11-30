# Traits in Rust

## About
* In Rust, there is no concept of "inheriting" the properties of a struct. Instead, when you are designing the relationship between objects do it in a way that one's functionality is defined by an interface (a trait in Rust). This promotes __composition over inheritance__, which is considered more useful and easier to extend to larger projects.