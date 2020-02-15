fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let x = 5;
    let y = x;
    println!("x = {}", x);
    println!("y = {}", y);

    let s2 = s.clone();
    println!("s = {}", s);
    println!("s2 = {}", s2);

    let s3 = gives_ownership();
    let s4 = takes_and_gives_back(s3);
    println!("s4 = {}", s4);
    
    let len = get_length(&s4);
    println!("len {}", len);

    let mut s5 = s4;
    append_world(&mut s5);
    println!("s5 = {}", s5);

    let s6 = no_dangle();
    println!("s6 = {}", s6);

    let s7 = String::from("Hello world");
    let first = first_word(&s7[..]);
    println!("first = {}", first);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn append_world(s: &mut String) {
    s.push_str(", world");
}

fn no_dangle() -> String {
    String::from("hello")
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}