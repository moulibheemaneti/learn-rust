// Functions in Rust - Comprehensive Examples
// Based on Chapter 3.3 documentation

fn main() {
    println!("=== Functions in Rust Examples ===\n");

    // Example 1: Basic Function with Return Value
    println!("1. Basic Function with Return Value:");
    let x = five();
    println!("The value of x is: {x}\n");

    // Example 2: Block Expression Example
    println!("2. Block Expression Example:");
    let y = {
        let x = 3;
        x + 1 // This expression becomes the value of y
    };
    println!("The value of y is: {y}\n");

    // Example 3: Function with Parameters
    println!("3. Function with Parameters:");
    let result = add(5, 3);
    println!("5 + 3 = {result}\n");

    // Example 4: Function without Return Type (implicitly returns ())
    println!("4. Function without Return Type:");
    greet("Alice");
    println!("Function call completed\n");

    // Example 5: Expression vs Statement Demo
    println!("5. Expression vs Statement Demo:");
    let expr_value = expression_example();
    println!("Expression returns: {expr_value}");

    let stmt_value = statement_example();
    println!("Statement returns: {:?}", stmt_value);
    println!();

    // Example 6: Multiple Parameters
    println!("6. Function with Multiple Parameters:");
    let area = calculate_rectangle_area(10, 5);
    println!("Rectangle area (10 x 5): {area}\n");

    // Example 7: String Parameters
    println!("7. Function with String Parameters:");
    let full_name = concatenate_names("John", "Doe");
    println!("Full name: {full_name}\n");

    // Example 8: Early Return
    println!("8. Early Return Example:");
    let number = 15;
    let is_even = check_even(number);
    println!("Is {number} even? {is_even}\n");

    // Example 9: Nested Function Calls
    println!("9. Nested Function Calls:");
    let complex_result = multiply_and_add(2, 3, 4);
    println!("(2 * 3) + 4 = {complex_result}\n");

    // Example 10: Function with Explicit Return Statement
    println!("10. Explicit Return Statement:");
    let explicit_result = explicit_return_example(10);
    println!("Explicit return result: {explicit_result}");
}

// Function that returns the value 5
fn five() -> i32 {
    5 // Expression - no semicolon means this is returned
}

// Function that adds two numbers
fn add(a: i32, b: i32) -> i32 {
    a + b // Expression without semicolon
}

// Function that greets a person (no return type, implicitly returns ())
fn greet(name: &str) {
    println!("Hello, {name}!");
}

// Example of expression (returns a value)
fn expression_example() -> i32 {
    42 // This is an expression that returns 42
}

// Example of statement (returns ())
fn statement_example() -> () {
    42; // This is a statement that returns ()
}

// Function with multiple parameters
fn calculate_rectangle_area(width: i32, height: i32) -> i32 {
    width * height
}

// Function that concatenates two strings
fn concatenate_names(first: &str, last: &str) -> String {
    format!("{} {}", first, last)
}

// Function with early return
fn check_even(number: i32) -> bool {
    if number % 2 == 0 {
        return true; // Early return
    }
    false // This would be reached if the condition is false
}

// Function that calls other functions
fn multiply_and_add(x: i32, y: i32, z: i32) -> i32 {
    let product = multiply(x, y);
    add(product, z)
}

// Helper function for multiplication
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// Function with explicit return statement
fn explicit_return_example(value: i32) -> i32 {
    return value * 2; // Explicit return (though not necessary)
}
