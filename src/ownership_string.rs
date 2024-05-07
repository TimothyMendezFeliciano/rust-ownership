pub fn ownership_string() {

    // This string can be mutated. Since it is not a string literal. And its references are stored in heap.
    let mut s = String::from("hello");

    s.push_str(", world!");
    println!("{}", s);

//     This s can be mutated. But string literals cannot.


}