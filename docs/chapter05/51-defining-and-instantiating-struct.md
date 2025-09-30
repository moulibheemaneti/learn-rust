# Defining and Instantiating Structs

## Struct Anatomy

Defining the struct.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

1. `struct` is a keyword stating that the following is a struct.
2. `User` is the name of the struct.
3. `active`, `username`, `email`, `sign_in_count` are the *`fields`* of the struct.
4. `bool`, `String`, `u64` are the types of the fields respectively.

## Instantiating struct

Creating an instance of the struct that is defined above.
```rust
let user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};
```

- Fetch a particular field using `Dot Notation`.
```rust
println!("Username: {}", user1.username);
```

- Update a field of the struct if it is mutable.
```rust
user1.email = String::from("another@example.com");
```

- **NOTE:** Rust doesn’t allow us to mark only certain fields as mutable

## Field Init Shorthand Syntax

This shorthand syntax is called: `field init shorthand`

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // shorthand for username: username,
        email, // shorthand for email: email,
        sign_in_count: 1,
    }
}
```

## Struct Update Syntax

This syntax is called: `struct update syntax`

```rust
let user2 = User {
    username: String::from("anotherusername567"),
    ..user1
};
```

- The `struct update syntax` must come at the end.

### Other observation
- Since the username and email fields are `String` type, they do not have `Copy` trait.
- So even if one of them is unchanged then the user1 object will move the values and makes user1 unusable.

## Tuple Structs

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

- Though `black` and `origin` seems to have same values internally, they are not same.
- They are not same because they both have different structs.

## Unit-Like Structs Without Any Fields

- Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```
