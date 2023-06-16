//! Associated types in Rust are a powerful feature that allow you to express more complex relationships between types in your trait definitions. They are a type of "type-level function" that can express a one-to-one mapping between types.
//!
//! In Rust, traits define a set of methods that a type should implement. But sometimes, the methods you want to define in a trait might need to work with or return types that are related to the implementing type in some way. This is where associated types come in.
//!
//! Here's an example. Let's say you have a `Vehicle` trait that represents different vehicles which might have different kinds of parts. For instance, a `Car` might have a `CarEngine` and a `Bike` might have a `BikeEngine`.
//!
//! Without associated types, you might try to define your trait like this:
//!
//! ```rust
//! trait Vehicle {
//!     fn get_engine(&self) -> ???;
//! }
//! ```
//!
//! But what type should go in place of `???`? It should be a type that's somehow related to the implementing type. With associated types, you can express this relationship like so:
//!
//! ```rust
//! trait Vehicle {
//!     type Engine;
//!
//!     fn get_engine(&self) -> Self::Engine;
//! }
//! ```
//!
//! Now, when you implement the `Vehicle` trait for `Car` and `Bike`, you can specify the associated `Engine` type:
//!
//! ```rust
//! struct CarEngine;
//! struct BikeEngine;
//!
//! struct Car;
//! struct Bike;
//!
//! impl Vehicle for Car {
//!     type Engine = CarEngine;
//!
//!     fn get_engine(&self) -> Self::Engine {
//!         // returns a CarEngine
//!     }
//! }
//!
//! impl Vehicle for Bike {
//!     type Engine = BikeEngine;
//!
//!     fn get_engine(&self) -> Self::Engine {
//!         // returns a BikeEngine
//!     }
//! }
//! ```
//!
//! As you can see, associated types allow you to express complex relationships between types and make your trait definitions more flexible and expressive. They are a key part of Rust's powerful and flexible type system.

/// Vehicle trait for all types of vehicle
trait Vehicle {
    /// associated type for `get_engine()`
    type Engine;

    /// without self
    fn get_engine() -> Self::Engine;

    /// with self
    fn get_name(&self) -> &'static str;
}

#[derive(Debug)]
struct Cylinder4;

#[derive(Debug)]
struct Cylinder1;

struct Car;
struct Bike;

impl Vehicle for Car {
    type Engine = Cylinder4;
    fn get_engine() -> Self::Engine {
        todo!()
    }

    fn get_name(&self) -> &'static str {
        todo!()
    }
}

impl Vehicle for Bike {
    type Engine = Cylinder1;

    fn get_engine() -> Self::Engine {
        todo!()
    }

    fn get_name(&self) -> &'static str {
        todo!()
    }
}

fn main() {
    let c = Car;
    println!("the car engine: {:?}", Car::get_engine());
    println!("M-1 of getting the car name: {:?}", Car::get_name(&c));
    println!("M-2 of getting the car name: {:?}", c.get_name());

    todo!("similar implementation for bike")
}
