//! This Rust code demonstrates how to define a single type (in this case `SDE`) and
//! implement different methods with the same name (`code`), each provided by a different trait.
//! This allows a single type to exhibit different behaviors depending on the context of use.
//!
//! Specifically, this code includes the following:
//! - A standalone implementation of the `code` method for the `SDE` type.
//! - An implementation of the `code` method from the `FrontendDeveloper` trait for the `SDE` type.
//! - An implementation of the `code` method from the `BackendDeveloper` trait for the `SDE` type.
//! - An implementation of the `code` method from the `BlockchainDeveloper` trait for the `SDE` type.

// Define a struct `SDE` representing a Software Development Engineer.
struct SDE;

// Define a trait `FrontendDeveloper` with a method `code`.
trait FrontendDeveloper {
    fn code(&self);
}

// Define a trait `BackendDeveloper` with a method `code`.
trait BackendDeveloper {
    fn code(&self);
}

// Define a trait `BlockchainDeveloper` with a method `code`.
trait BlockchainDeveloper {
    fn code(&self);
}

/// Implement a `code` method for the `SDE` type.
impl SDE {
    fn code(&self) {
        println!("I am a SDE");
    }
}

/// Implement the `code` method from the `FrontendDeveloper` trait for the `SDE` type.
impl FrontendDeveloper for SDE {
    fn code(&self) {
        println!("SDE can code in the context of a Frontend Developer");
    }
}

/// Implement the `code` method from the `BackendDeveloper` trait for the `SDE` type.
impl BackendDeveloper for SDE {
    fn code(&self) {
        println!("SDE can code in the context of a Backend Developer");
    }
}

/// Implement the `code` method from the `BlockchainDeveloper` trait for the `SDE` type.
impl BlockchainDeveloper for SDE {
    fn code(&self) {
        println!("SDE can code in the context of a Blockchain Developer");
    }
}

pub(crate) fn main() {
    let sde = SDE;

    // Invoke the `code` method directly from the `SDE` instance.
    sde.code(); // --> I am a SDE

    println!(
        "====\n## Method 1: Invoke the `code` method from trait implementations for the `SDE` type using the trait's name:\n----"
    );
    // Invoke the `code` method from the `FrontendDeveloper` trait for the `SDE` type.
    FrontendDeveloper::code(&sde); // --> SDE can code in the context of a Frontend Developer

    // Invoke the `code` method from the `BackendDeveloper` trait for the `SDE` type.
    BackendDeveloper::code(&sde); // --> SDE can code in the context of a Backend Developer

    // Invoke the `code` method from the `BlockchainDeveloper` trait for the `SDE` type.
    BlockchainDeveloper::code(&sde); // --> SDE can code in the context of a Blockchain Developer

    println!(
        "====\n## Method 2: Invoke the `code` method from trait implementations for the `SDE` type using the struct's name:\n----"
    );

    // Invoke the `code` method from the `FrontendDeveloper` trait for the `SDE` type.
    <SDE as FrontendDeveloper>::code(&sde); // --> SDE can code in the context of a Frontend Developer

    // Invoke the `code` method from the `BackendDeveloper` trait for the `SDE` type.
    <SDE as BackendDeveloper>::code(&sde); // --> SDE can code in the context of a Backend Developer

    // Invoke the `code` method from the `BlockchainDeveloper` trait for the `SDE` type.
    <SDE as BlockchainDeveloper>::code(&sde); // --> SDE can code in the context of a Blockchain Developer
}
