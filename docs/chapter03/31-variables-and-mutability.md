# Variables and Mutability

## New terms

### 1. (Non-technical term) **Nudges**  
**Definition**: A "nudge" is a gentle push or encouragement to do something.

**Example**: If a program gives you a hint about what to do next, that’s a nudge.

**Explanation**: Imagine a friend softly reminding you to finish your homework—that’s a nudge. In programming, nudges help guide you in the right direction without forcing you.

### 2. **Concurrency**  
**Definition**: Concurrency means running multiple parts of a program at the same time, so they can make progress independently.

**Example**: If your program is downloading a file and processing user input at the same time, that's concurrency.

**Explanation**: Imagine you’re cooking dinner and talking on the phone at the same time. You’re doing two things at once, switching between them as needed. In programming, concurrency lets your program handle more than one task at a time, making it more efficient and responsive.

## Key concepts

### Immutability by Default

- Variables are **immutable** by default in Rust.
- Once you bind a value to a name, you cannot change it (reassign) unless it’s declared `mut`.
- Attempting to reassign an immutable variable yields a compile-time error.
- This immutability helps prevent bugs: you can rely on values not changing unexpectedly.

### Using `mut` for Mutability

- To allow modification, prefix the binding with `mut`:
  ```rust
  let mut x = 5;
  x = 6;
  ```
- `mut` signals intent to future readers that the value may change.

### Constants (`const`)

- Constants are **always immutable**; you can’t use `mut` with `const`.
- Use the `const` keyword, and you must **annotate the type**.
- Constants can be declared in any scope (including global).
- Their values must be *constant expressions* (evaluated at compile time), not runtime results.

### Shadowing (`let` reuse)

- Rust allows you to **shadow** a variable by re-declaring it with `let` using the same name.
- Shadowing “overshadows” the previous binding — the old value is no longer accessible in that scope.
- **Differences vs. **``**:**
  - Shadowing requires `let`, so accidental reassignments (without `let`) are prevented.
  - You can change the **type** of the value when shadowing (e.g. from `&str` to `usize`).
  - Useful for transformation pipelines using the same name.
- Example of nested shadowing and scopes:
  ```rust
  let x = 5;
  let x = x + 1;         // x is now 6

  {
    let x = x * 2;       // in inner scope, x is now 12
    println!("inner x: {x}");
  }

  println!("outer x: {x}");  // prints 6 again
  ```

---

### Why this design?

- Encourages safer code by default.
- Enables transformation chains with the same name (without `mut`).
- Prevents accidental reuse of old values.
- Offers flexibility via shadowing (including type changes).

