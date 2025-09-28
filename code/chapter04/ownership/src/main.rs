/// Prints the type name of the given value to the console.
///
/// # Type Parameters
/// - `T`: The type of the value whose type name will be printed.
///
/// # Arguments
/// - `_: &T` - A reference to a value of any type.
///
/// # Example
/// ```
/// let x = 5;
/// print_type_of(&x); // prints "i32"
/// ```
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

/// Prints the type name of the value pointed to by the given reference at runtime.
/// The only difference from the above function is that this one prints the type name of the value at runtime,
/// allowing it to work with trait objects and unsized types, whereas the previous function prints the static type name at compile time.
///
/// # Arguments
/// - `x`: A reference to a value of any type (including unsized types).
///
/// # Example
/// ```
/// let s = String::from("hello");
/// print_type_of_variable(&s); // prints "alloc::string::String"
/// ```
fn print_type_of_variable(x: &impl ?Sized) {
    println!("{}", std::any::type_name_of_val(x));
}

fn main() {
    let a = "hello";
    print!("type of a: ");
    print_type_of(&a); // prints "&str"

    let b = String::from("hello");
    print!("type of b: ");
    print_type_of_variable(&b); // prints "alloc::string::String"

    {
        // c is not valid here, since it's not yet declared
        let c = "hello"; // c is valid from this point forward

        // do stuff with c
        println!("c: {}", c);
    } // this scope is now over, and s is no longer valid

    {
        let mut d = String::from("hello"); // d is valid from this point forward

        // do stuff with d
        d.push_str(", world!"); // push_str() appends a literal to a String

        println!("d: {d}"); // this will print `hello, world!`
    } // this scope is now over, and d is no longer valid

    // Concept: Copy
    let e = 5;
    let f = e;
    println!("e: {e}");
    println!("f: {f}");

    let g = String::from("hello");
    let h = g;

    // println!("g: {g}");
    println!("\n-------------ERROR-------------------");
    println!("let h = g;");
    println!("g: Here error will occur if g is fetched, as it is invalid in this scope.");
    println!("-------------END OF ERROR-------------------\n");
    println!("h: {h}");

    let mut i = String::from("hello");

    println!("i: old: {i}");

    i = String::from("ahoy");

    println!("i: new: {i}");

    println!("\n-------------UPDATE-------------------");
    println!("`i` dropped previous value and took new value");
    println!("-----------------------------------------------\n");

    let j = String::from("hello");
    let k = j.clone();

    println!("j: {j}");

    println!("\n-------------INFO-------------------");
    println!("k: {k}");
    println!("`k` is cloned from `j`");
    println!("-----------------------------------------------\n");

    let l = String::from("hello"); // l comes into scope

    takes_ownership(l); // l's value moves into the function...
                        // ... and so is no longer valid here

    println!("\n-------------ERROR-------------------");
    println!("l = String::from('hello')");
    println!("Since `l` is moved to `takes_ownership` function, it is no longer valid here.");
    println!("-----------------------------------------------\n");

    let m = 5; // m comes into scope

    makes_copy(m); // Because i32 implements the Copy trait,
                   // m does NOT move into the function,
                   // so it's okay to use m afterward.

    println!("\n-------------INFO-------------------");
    println!("let m = 5");
    println!(
        "Since `m` is just copied and not moved to `makes_copy` function, it is still valid here."
    );
    println!("-----------------------------------------------\n");

    println!("m: {m}");

    let n = gives_ownership(); // gives_ownership moves its return
                               // value into n

    println!("\n-------------UPDATE-------------------");
    println!("`n` got ownership to the returned value from `gives_ownership`");
    println!("-----------------------------------------------\n");

    println!("n: {n}");

    let o = String::from("hello"); // o comes into scope

    let p = takes_and_gives_back_ownership(o); // o is moved into
                                               // takes_and_gives_back_ownership, which also
                                               // moves its return value into p

    println!("\n-------------UPDATE-------------------");
    println!("`o` is moved to `takes_and_gives_back_ownership` function, which also moves its return value into `p`");
    println!("-----------------------------------------------\n");

    println!("p: {p}");
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("fn takes_ownership: {some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("fn makes_copy: {some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

// This function takes a String and returns a String.
fn takes_and_gives_back_ownership(a_string: String) -> String {
    // a_string comes into scope

    a_string // a_string is returned and moves out to the calling function
}
