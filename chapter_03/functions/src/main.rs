fn main() {

    let y = {
        let z = 3;
        z + 1
    };

    let x = plus_one(five()) - 1;

    another_function(x, y);

    let foo = if true {
        42
    } else {
        0
    };
    
    println!("Foo = {}", foo);

    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10 {
            break;
        }
    }

    println!("Counter = {}", counter);

    while counter < 20 {
        counter += 1;
    }

    println!("Counter = {}", counter);

    let a = [1, 1, 2, 3, 5, 8, 13, 21];

    for item in a.iter() {
        println!("Iter = {}", item);
    }

    for n in (1..4).rev() {
        println!("{}!", n);
    }
    println!("LIFTOFF!!!")

}


fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}