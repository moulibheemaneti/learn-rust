fn main() {
    let mut a = String::from("hello world");

    let b = first_word(&a); // word will get the value 5

    println!("b: {b}, {}", &b);

    let c = &a[1..6];

    println!("c: {c} (1 to 6), {}", &c);

    a.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!

    // println!("s: {}, word: {}", s, word);

    let d = "hello world";

    let e = &d[2..8];

    println!("e: {e} (2 to 8), {}", &e);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // println!("bytes: {:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        // println!("i:{}, &item: {}, item: {}", i, &item, item);
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
