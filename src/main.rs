use crate::borrowing::borrowing;
use crate::dangling::dangling;
use crate::ownership_functions::{multiple_returns, returning_ownership};
use crate::ownership_string::ownership_string;

mod ownership_string;
mod ownership_functions;
mod borrowing;
mod dangling;

fn main() {
    println!("Hello, world!");
//     Ownership:
//     1. Each value has an owner
//     2. There can only be one owner at a time.
//     3. When the owner goes out of scope, the value will be dropped.

    // s is not valid here, it’s not yet declared
    let _s = "hello";   // s is valid from this point forward

    // do stuff with s
    ownership_string();

    returning_ownership();

    multiple_returns();

    borrowing();
    dangling();
}                      // this scope is now over, and s is no longer valid