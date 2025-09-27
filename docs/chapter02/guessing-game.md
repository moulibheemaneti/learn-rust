# Programming a Guessing Game

## Breaking down 1

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

1. **use**:
    - `use` is a keyword in Rust that brings items (such as modules, structs, functions, or traits) from a crate or module into scope, so you can refer to them without their full path. 
    - In this example, `use std::io;` allows us to use the `io` module from Rust's standard library directly.

2. **std:io**:
    -  Rust has default a standard library which has too many libraries in it.
    - Since we need only input output related library, so using `std::io`

3. **println!**:
    - A macro that prints text to the console.

4. **let**:
    - Using `let` we create new variables.
    - By default variables in rust are `immutable`. That means, once a value is assigned the value won't change.

5. **String::new**:
    - The `::` syntax in `String::new` indicates that `new` is an *associated function* of the `String` type.
    - An *associated function* is a function that’s implemented on a type (like `String`), rather than on an instance of the type.
    - Many types in Rust provide a `new` function as a convention for creating a new value of that type.
    - In this example, `let mut guess = String::new();` creates a mutable, empty string to store the user's input.

6. **expect**:
    - `read_line` returns a `Result` enum, which can be in one of two variants:
        - `Ok`: the operation was successful and contains the value.
        - `Err`: the operation failed and contains error information.
    - The `.expect()` method is used to handle the `Result`. If it's `Err`, the program will stop and display the provided error message.

## New terms
1. **Library Crate**  
**Definition**: A library crate is a package of Rust code that provides functionality for other programs to use, but does not have a `main` function and cannot be run directly.  

**Example**: The `rand` crate is a library crate that provides random number generation.  

**Explanation**: Think of a library crate as a toolbox full of useful tools that you can use in your own projects.

2. **Binary Crate**  
**Definition**: A binary crate is a package of Rust code that has a `main` function and compiles to an executable program you can run.  

**Example**: The guessing game program you write in this chapter is a binary crate.  

**Explanation**: A binary crate is like a finished machine built from tools in your toolbox—it does something when you run it.

## Key Concepts

1. `rand::rng` function that gives us the particular random number generator we’re going to use: one that is local to the current thread of execution and is seeded by the operating system.

**What does "local to the current thread" mean?**  
It means that each thread (a separate path of execution in your program) gets its own random number generator, so random numbers generated in one thread won’t interfere with those in another.

**What does "seeded by the operating system" mean?**  
A "seed" is an initial value used to start the random number generator. When it’s seeded by the operating system, it uses unpredictable data from your computer (like the current time or other system events) to make sure the sequence of random numbers is different each time you run the program.

2. Running the `cargo doc --open` command will build documentation provided by all your dependencies locally and open it in your browser.

3. **& (Reference)**
   - The `&` symbol means "reference" in Rust.
   - References let multiple parts of your code access the same data without copying it.

4. **match Expression**
   - A `match` expression is made up of "arms".
     - Each arm has a pattern and code to run if the pattern matches.
   - Rust checks each arm in order and runs the code for the first matching pattern.
   - Patterns and `match` help you handle different situations safely and clearly.

5. **Variable Shadowing**
   - You can reuse a variable name (like `guess`) to "shadow" its previous value.
   - This avoids creating extra variable names (e.g., `guess_str` and `guess`).

## References

1. [Crates.io](https://www.crates.io): Similar to [npm](https://www.npmjs.com/)
