mod ownership_string;
mod ownership_functions;

use crate::ownership_string::ownership_string;

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
}                      // this scope is now over, and s is no longer valid