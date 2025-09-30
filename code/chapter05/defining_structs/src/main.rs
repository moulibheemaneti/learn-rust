// The #[derive(Debug)] attribute automatically implements the Debug trait for the struct below.
// This allows instances of the struct to be formatted using {:?} in println! and similar macros.
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("---------- USER 1 ----------");
    println!("User 1: {:?}", user1);
    println!("Username: {}", user1.username);

    user1.email = String::from("another@example.com");
    println!("After Email Update: {:?}", user1);

    println!("--------------------------------\n");

    let user2 = build_user(
        String::from("anotherusername567"),
        String::from("anotherusername567"),
    );
    println!("---------- USER 2 ----------");
    println!("User 2: {:?}", user2);

    println!("--------------------------------\n");

    let user3 = User {
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("---------- USER 3 ----------");
    println!("User 3: {:?}", user3);

    println!("--------------------------------\n");

    let black = Color(0, 0, 0);
    println!("Black: {:?}", black);

    let origin = Point(0, 0, 0);
    println!("Origin: {:?}", origin);

    let subject = AlwaysEqual;
    println!("Subject: {:?}", subject);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // shorthand for username: username,
        email,    // shorthand for email: email,
        sign_in_count: 1,
    }
}
