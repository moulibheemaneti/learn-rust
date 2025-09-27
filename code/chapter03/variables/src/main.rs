/*
 * Variables are by default immutable.
 * To make a variable mutable, we need to use the `mut` keyword.
 * Immutable variables are different from constant variables.
 * Not allowed to use mut with constant.
 * Constants are always immutable.
 * Constants are declared using the `const` keyword.
 * Constants can be declared in any scope (including global).
 * Their values must be constant expressions (evaluated at compile time), not runtime results.
 */

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    let x = 5;

    // Shadowing a variable
    let x = x + 1;

    // Cannot mutate. It will throw an error.
    // This is to avoid accidental mutation.
    // x = x + 1;

    {
        // Shadowing a variable, but in a different scope.
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    // The value of x is 6.
    println!("The value of x is: {x}");
}
