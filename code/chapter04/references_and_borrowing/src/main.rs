fn main() {
    let a = String::from("hello");

    let length = calculate_length(&a);

    println!("a: len: {length}");

    let mut b = String::from("hello");

    println!("b old: {b}");

    change(&mut b);

    println!("b new: {b}");

    let _c = &mut b;
    let d = &mut b;

    println!("\n-------------ERROR-------------------");
    println!("let c = &mut b;");
    println!("c: Cannot access c as it was transferred to d.");
    println!("-------------END OF ERROR-------------------\n");

    println!("d: {d}");

    {
        let e = &mut b;

        println!("\n-------------INFO-------------------");
        println!("e: {e}");
        println!("Since e is in different scope from f, it is still valid.");
        println!("-------------END OF ERROR-------------------\n");
    } // e goes out of scope here, so we can make a new reference with no problems.

    let f = &mut b;
    println!("f: {f}");

    let g = dangle();

    println!("g: {g}; from `dangle`");
}

fn calculate_length(s: &String) -> usize {
    println!("calculate_length: s is immutable, changing it would throw an error.");

    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.

fn change(s: &mut String) {
    println!("change: some_string is mutable, changing it would not throw an error.");
    s.push_str(", world");
}

fn dangle() -> String {
    // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    // &s // we return a reference to the String, s
    println!("\n-------------ERROR-------------------");
    println!("returning `&s` results in error as the s's scope terminates at the end.");
    println!("-------------END OF ERROR-------------------\n");

    s
} // Here, s goes out of scope and is dropped, so its memory goes away.
