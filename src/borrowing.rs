pub fn borrowing() {
    let mut s1 = String::from("open");

    let len = calculate_length(&s1);
    change(&mut s1);

    println!("The length of '{}' is {}.", s1, len);
    println!("The modified phrase is {s1}");

    only_one_borrower();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(" sesame!");
}

fn only_one_borrower() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);
//     The above will throw an error.
//     This is meant for examples only.

    // The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion. It’s something that new Rustaceans struggle with because most languages let you mutate whenever you’d like. The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:
    //
    //     Two or more pointers access the same data at the same time.
    //     At least one of the pointers is being used to write to the data.
    //     There’s no mechanism being used to synchronize access to the data.


    let mut p = String::from("hello");

    {
        let p1 = &mut p;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let p2 = &mut p;

    let mut t = String::from("hello");

    let t1 = &t; // no problem
    let t2 = &t; // no problem
    // let t3 = &mut t; // BIG PROBLEM

    // println!("{}, {}, and {}", t1, t2, t3);
//     We cannot have mutable references to immutable values before their usage.

    let mut f = String::from("Formula");

    let f1 = &f;
    let f2 = &f; // Still an error when building. Only one borrower at a time.

    println!("{} and {}", f1, f2);
//     variables f1 and f2 will not be used beyond this point.

    let f3 = &mut s; // no problemo
    f3.push_str(" 1");
    println!("{}", f3);
}