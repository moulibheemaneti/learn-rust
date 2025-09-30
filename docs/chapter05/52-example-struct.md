# Example Structs

- Using println! to print the struct throws an error.
- Avoid it by using `#[derive(Debug)]` on top of the struct.
    - This enables debug and allows us to print on the screen.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
    dbg!(&rect1); // Debug line
}
```

- {:?} → Compact debug format
- {:#?} → Pretty debug format (multiline, indented)

- Let's try to tie up the area method to the struct.
