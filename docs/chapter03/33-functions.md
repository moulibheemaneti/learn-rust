# Functions

Functions are fundamental building blocks in Rust. They allow you to organize code into reusable, named blocks that perform specific tasks.

## Key Concepts

• **Function Definition**: Use `fn` keyword followed by function name, parameters, and return type
• **Return Values**: Functions can return values using expressions (without semicolon) or explicit `return` statements
• **Expression vs Statement**: Expressions evaluate to a value, statements don't return values
• **Function Parameters**: Specify parameter types in the function signature
• **Function Bodies**: Contain statements and expressions

## Examples

### Basic Function with Return Value

```rust
fn five() -> i32 {
    5  // Expression - no semicolon means this is returned
}

fn main() {
    let x = five();
    println!("The value of x is: {x}");
}
```

### Block Expression Example

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1  // This expression becomes the value of y
    };

    println!("The value of y is: {y}");
}
```

### Important Notes

- Return expressions should **not** end with a semicolon

- If you add a semicolon to a return expression, it becomes a statement and returns `()`

- Function parameters must have explicit type annotations

- Functions without a return type implicitly return `()`


