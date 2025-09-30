# Method Syntax

- We declare methods with `fn` keyword and a name
    - They can have `parameters` and a `return value`
- Unlike functions, methods are defined within the context of a struct (or an enum or a trait object)

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

- `&self`, here `&` is being used because we are borrowing the instance.

- Methods that simply return a field's value are called *getters*.
- Rust does not auto-generate getters for struct fields.
- Getters let you keep fields private but expose read-only access via public methods.

- Implementing the same struct is valid. Like the following:
```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

