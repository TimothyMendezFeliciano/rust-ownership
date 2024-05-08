pub fn slices() {
    let mut s = String::from("hello world");
    let word = first_word(&s); //word will get the value 5.

    s.clear(); //l this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that we could meaningfully use the value 5 with. word is no totally invalid!

    // solution to the disconnected values? SLICES.

    let slice = String::from("Hello World");

    let s1 = first_word(&slice);
    let s2 = second_word(&slice);

    println!("The first word is {}", s1);
    println!("The second word is {}", s2);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }


    &s[..]
}