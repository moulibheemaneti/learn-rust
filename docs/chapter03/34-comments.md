# Comments

## Single-Line Comments

In Rust, single-line comments start with two forward slashes `//`. Everything after `//` on that line is ignored by the compiler.

```rust
fn main() {
    // This is a single-line comment
    let x = 5; // Comments can also be at the end of a line
    let y = 10; // This line will be executed
    // let z = 15; // This line is commented out and won't run
    println!("x = {}, y = {}", x, y);
}
```

## Multi-Line Comments

Rust also supports multi-line comments, which start with `/*` and end with `*/`. Everything between these markers is ignored by the compiler, even if it spans multiple lines. Multi-line comments can be useful for temporarily disabling blocks of code or writing longer explanations.

```rust
fn main() {
    /* This is a multi-line comment
       that spans multiple lines
       and can contain detailed explanations */
    
    let x = 5;
    
    /* 
    This is another way to write
    multi-line comments with
    better formatting
    */
    
    /* Temporarily disable this code block:
    let y = 10;
    let z = x + y;
    println!("z = {}", z);
    */
    
    println!("x = {}", x);
}
```

## Documentation Comments

Rust has a special kind of comment called a **documentation comment** (or *doc comment*), which is used to generate HTML documentation for your code. Doc comments use triple slashes `///` for documenting items like functions, structs, enums, and modules.

```rust
/// This function calculates the area of a rectangle
/// 
/// # Arguments
/// 
/// * `width` - The width of the rectangle
/// * `height` - The height of the rectangle
/// 
/// # Returns
/// 
/// The area of the rectangle as a floating-point number
/// 
/// # Examples
/// 
/// ```
/// let area = calculate_area(5.0, 3.0);
/// assert_eq!(area, 15.0);
/// ```
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}

/// A simple struct representing a point in 2D space
struct Point {
    /// The x-coordinate of the point
    x: f64,
    /// The y-coordinate of the point
    y: f64,
}

fn main() {
    let area = calculate_area(4.0, 6.0);
    println!("Area: {}", area);
    
    let point = Point { x: 1.0, y: 2.0 };
    println!("Point: ({}, {})", point.x, point.y);
}
```

