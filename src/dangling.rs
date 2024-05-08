pub fn dangling() {

    // let reference_to_nothing = dangle();

    let actual_value_of_something = no_dangle();
}

// This is an error.
// fn dangle() -> &String {
//     let s = String::from("hello");

    // &s // We return a reference to the String, s.
// } // Here, s goes out of scope, and is dropped. Its memory goes away.

// solution
fn no_dangle() -> String {
    let s  = String::from("Hello");

    s
} //Ownership is moved out, and nothing is deallocated.

// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.