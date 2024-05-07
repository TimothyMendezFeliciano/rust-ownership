pub fn ownership_string() {

    // This string can be mutated. Since it is not a string literal. And its references are stored in heap.
    let mut _s = String::from("hello");

    _s.push_str(", world!");
    println!("{}", _s);

//     This s can be mutated. But string literals cannot.


    let x1 = String::from("Example 1");
    let x2 = x1.clone();

    let _ = x2.replace("1", "2");

    println!("Was x1 affected? {}", x1);
}