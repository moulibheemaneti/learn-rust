# References and Borrowing

## Key points
- References represented by `ampersand (&)`.
- The opposite of referencing by using `&` is dereferencing, which is accomplished with the dereference operator, `*`.
- Creating a reference is called `borrowing`. You use it temporarily, but you don’t own it.

![references illustration](https://doc.rust-lang.org/book/img/trpl04-06.svg)

- Mutable reference:
```rust
let mut s = String::from("hello");
change(&mut s);
```


### Mutable reference restriction
- You can't have more than one mutable reference to a value at a time. For example, this will fail:

This code does not compile!
```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

// Here r1 is not accessible.
println!("{r1}, {r2}");
```

- At a given point of time only one owner for a value should exist, so when r2 is being referenced to &mut s, then r1 transferred it's ownership.

- We also cannot have a mutable reference while we have an immutable one to the same value.

The below code also doesn't work
```rust
let mut s = String::from("hello");

let r1 = &s; // immutable reference
let r2 = &s; // immutable reference
let r3 = &mut s; // mutable reference

// Cannot access r1 and r1 here anymore.
```

- The benefit of having this restriction is that Rust can prevent data races at compile time.
- A data race is similar to a race condition and happens when these three behaviors occur:
    - Two or more pointers access the same data at the same time.
    - At least one of the pointers is being used to write to the data.
    - There’s no mechanism being used to synchronize access to the data.
- Data races cause undefined behavior; Rust prevents them by refusing to compile such code.
- Solution to avoid race conditions
```rust
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
```


1. Differences between pointer and references
<table>
  <thead>
    <tr>
      <th>Aspect</th>
      <th>Reference (<code>&amp;T</code>, <code>&amp;mut T</code>)</th>
      <th>Raw Pointer (<code>*const T</code>, <code>*mut T</code>)</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><b>Safety</b></td>
      <td>Checked by borrow checker, always safe</td>
      <td>Must be used in <code>unsafe</code>, risky</td>
    </tr>
    <tr>
      <td><b>Nullability</b></td>
      <td>Never null</td>
      <td>Can be null</td>
    </tr>
    <tr>
      <td><b>Lifetimes</b></td>
      <td>Enforced by compiler</td>
      <td>Not tracked</td>
    </tr>
    <tr>
      <td><b>Dereferencing</b></td>
      <td>Safe, automatic in many contexts</td>
      <td>Requires <code>unsafe</code> block</td>
    </tr>
    <tr>
      <td><b>Guarantees</b></td>
      <td>No dangling, aliasing rules enforced</td>
      <td>No guarantees (UB possible)</td>
    </tr>
    <tr>
      <td><b>Use case</b></td>
      <td>Day-to-day borrowing &amp; references</td>
      <td>FFI, low-level memory, unsafe code</td>
    </tr>
  </tbody>
</table>

- At any given time, you can have either one mutable reference or any number of immutable references.
References must always be valid.

## Dangling References

- A dangling pointer is a pointer that references memory that has already been freed or is otherwise invalid. Accessing it leads to undefined behavior in languages like C/C++.
- Rust’s ownership system prevents dangling pointers at compile time by enforcing strict borrowing and lifetime rules.
- This avoids accessing invalid memory, ensuring memory safety without garbage collection.

- In Rust (compiler prevents it):
```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // ❌ ERROR: `s` does not live long enough
}      // `s` is dropped here → no dangling reference allowed
```
- Correct way in Rust (return ownership):
```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // ownership moved out, safe
}
```


1. Why do we need to use dereferencing?
