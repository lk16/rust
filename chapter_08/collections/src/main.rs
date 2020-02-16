use std::collections::HashMap;

fn main() {
    
    let _v = vec![1, 2, 3];
    let _v2: Vec<i32> = Vec::new();
    
    let mut v3 = Vec::new();
    v3.push(42);

    match v3.get(0) {
        Some(first) => println!("first item = {}", first),
        None => println!("there is no first item"),
    }

    match v3.get(1) {
        Some(second) => println!("second item = {}", second),
        None => println!("there is no second item"),
    }

    v3.push(43);
    v3.push(44);

    for i in &v3 {
        println!("v3 has item {}", i);
    }

    for i in &mut v3 {
        *i += 3
    }

    println!("---");

    for i in &v3 {
        println!("v3 has item {}", i);
    }

    println!("---");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    println!("---");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{} => {}", key, value);
    }

    println!("---");

    let team_name = String::from("Blue");
    println!("Blue => {:#?}", scores.get(&team_name));

    scores.insert(String::from("Blue"), 30);
    println!("Blue => {:#?}", scores.get(&team_name));

    println!("---");

    scores.entry(String::from("Blue")).or_insert(90);
    scores.entry(String::from("Red")).or_insert(90);

    for (key, value) in &scores {
        println!("{} => {}", key, value);
    }

    println!("---");

    let text = "foo bar bar foo foo foo baz quux";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count= map.entry(word).or_insert(0);
        *count += 1
    }

    for (key, value) in &map {
        println!("{} => {}", key, value);
    }
}
