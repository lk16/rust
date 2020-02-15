#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let email = String::from("hello@world.com");
    let username = String::from("user");

    let mut user = build_user(email, username);

    user.username = String::from("other_user");

    let user2 = User {
        sign_in_count: 42,
        ..user
    };

    println!("user2 = {:#?}", user2);

    let rect = Rectangle {
        width: 6,
        height: 7,
    };
    println!("rect = {:#?}", rect);

    let rect_area = rect.area();
    println!("area = {}", rect_area);

    let smaller_rect = Rectangle {
        width: 3,
        height: 4,
    };

    println!("small can hold big? {}", rect.can_hold(&smaller_rect));
    println!("big can hold small? {}", smaller_rect.can_hold(&rect));
    println!("rect can hold itself? {}", rect.can_hold(&rect));

    let square = Rectangle::square(5);
    println!("square = {:#?}", square);

}
