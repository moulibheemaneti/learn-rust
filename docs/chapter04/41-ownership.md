# Ownership

## New Terms

### 1. **Garbage collector**  

**Definition:**  
A garbage collector is a system that automatically detects and reclaims memory that a program no longer uses, eliminating the need for the programmer to manually free memory.

**Example:**  
In Java, memory is automatically freed by the garbage collector when objects are no longer referenced, so you don’t have to explicitly release memory.

**Explanation:**  
Think of a garbage collector like a cleaning robot in your house. As you use things and leave them lying around, the robot comes by and picks up anything you’re done with, so you don’t have to worry about cleaning up after yourself. In programming, this means you can focus on writing your code without having to remember to free up memory, because the garbage collector takes care of it for you.

### 2. **Systems programming language**

**Definition:**  
A systems programming language is a category of programming languages designed to provide direct access to hardware and low-level system resources, enabling the development of operating systems, device drivers, and other system software.

**Example:**  
C and C++ are systems programming languages that allow manual memory management and direct hardware interaction.

**Explanation:**  
Think of a systems programming language like a set of specialized tools for building and maintaining the foundation of a house (the computer). These languages let you work directly with the building materials (hardware and memory), giving you more control and responsibility. This is why they are used for tasks like writing operating systems and device drivers, where efficiency and direct access are crucial.

### 3. **Stack**

**Definition:**  
A stack is a memory storage system that stores data in a fixed-size, last-in-first-out (LIFO) order, allowing for fast access and retrieval. Adding data is called `pushing` onto the stack, and removing data is called `popping` off the stack.

**Example:**  
In Rust, local variables, function parameters, and return addresses are stored on the stack. For example, when you declare a variable inside a function, it is automatically stored on the stack.

**Explanation:**  
Think of the stack like a stack of plates: you add (push) new plates on top and remove (pop) them from the top. The stack is fast because the CPU always knows where the top is, but it only works for data with a known, fixed size. This is why things like local variables and function calls use the stack.

### 4. **Heap**

**Definition:**  
A heap is a memory storage system used for dynamic memory allocation, where data can be stored and accessed in any order, not just last-in-first-out (LIFO).

**Example:**  
In C or C++, when you use `malloc` or `new` to allocate memory for things like dynamic arrays or linked lists, that memory comes from the heap. To remove or free this memory when you're done, you use `free` (for `malloc`) or `delete` (for `new`).

**Explanation:**  
Think of the heap like a big, messy storage room where you can put things wherever there’s space. When you need memory, you ask the allocator, which finds a spot for you and gives you the address (a pointer). Accessing data in the heap is generally slower than the stack because the memory can be scattered and less organized. The heap is used for data whose size or lifetime can’t be known at compile time, such as objects that need to live beyond the current function call.

### 5. **Contemporary processor**

**Definition:**  
A contemporary processor is a modern CPU designed to execute instructions efficiently, with optimizations that favor accessing memory in a predictable, sequential manner.

**Example:**  
When a program accesses elements in a large array one after another (sequentially), a contemporary processor can fetch data faster due to its memory caching strategies.

**Explanation:**  
Think of a contemporary processor like a librarian who works much faster when books are arranged in order on a shelf. If you ask for books one after another from the same shelf, the librarian can hand them to you quickly. But if you keep asking for books from random places all over the library, it takes longer. Similarly, modern CPUs are much faster when your code accesses memory in a straight line, rather than jumping around to random locations.

## Ownership Rules

Core ownership rules:
1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

## Variable Scope

- All data stored on the stack must have a known, fixed size.
- Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

- Heap allocation:
  - You request a certain amount of memory.
  - The memory allocator finds a free spot big enough and marks it as used.
  - It gives you a pointer (address) to that memory.
  - This is called "allocating on the heap" (unlike stack, which does not require allocation).

<table>
  <thead>
    <tr>
      <th>Language</th>
      <th>Memory Allocation</th>
      <th>Memory Freeing</th>
      <th>When Freed?</th>
      <th>Pros</th>
      <th>Cons</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Python / JavaScript</td>
      <td>Heap (via GC)</td>
      <td>Garbage Collector scans & frees</td>
      <td>Eventually (not immediate)</td>
      <td>Easy to use, no manual free</td>
      <td>Unpredictable timing, possible GC pauses</td>
    </tr>
    <tr>
      <td>C / C++ (manual)</td>
      <td>Heap (malloc/new)</td>
      <td>Explicit free/delete</td>
      <td>When programmer calls free()</td>
      <td>Full control</td>
      <td>Error-prone (leaks, double free, dangling)</td>
    </tr>
    <tr>
      <td>Rust</td>
      <td>Heap (String::from, Vec, etc.)</td>
      <td>Automatic via <code>drop</code></td>
      <td>Immediately at scope end</td>
      <td>Safe, predictable, no GC overhead</td>
      <td>Stricter rules (ownership/borrowing)</td>
    </tr>
  </tbody>
</table>

### Example: Representation in heap

```rust
let s1 = String::from("hello");
```

Listing 4-1: Representation in heap
![Listing 4-1: Representation in heap](https://doc.rust-lang.org/stable/book/img/trpl04-01.svg){ width=400 }

> - **Pointer**: Stores the address of the string data in the heap (i.e., the starting address of the string's contents in memory).
> - **Length**: bytes currently used by the String.
> - **Capacity**: total bytes allocated for the String.
> - The difference between length and capacity matters, but not in this context, so for now, it’s fine to ignore the capacity.


```rust
let s1 = String::from("hello");
let s2 = s1;
```

Listing 4-2: Representation in memory of the variable s2 that has a copy of the pointer, length, and capacity of s1
![Listing 4-2: Representation in memory of the variable s2 that has a copy of the pointer, length, and capacity of s1](https://doc.rust-lang.org/stable/book/img/trpl04-02.svg){ width=400 }

- Copying pointer, length, and capacity (not the data) is like a shallow copy in other languages.
- In Rust, this is called a "move" because the original variable becomes invalid after the copy.
- As per the [first rule of ownership](#ownership-rules), only one variable can own a value at a time. So, when we assign `s1` to `s2`, the ownership of the value is moved from `s1` to `s2`.
- Rust never automatically makes deep copies; automatic copies are always shallow and cheap.
-  Assigning a new value to a String drops (frees) the old heap allocation automatically.

```rust
let mut s = String::from("hello");
s = String::from("ahoy"); // drops the previous value in the heap.
```

## Variables and Data Interacting with Clone

- If we *`do want to deeply copy`* the heap data of the String, not just the stack data, use *`clone`* method.
- Types with known, fixed size (like integers) are:
  - Stored entirely on the stack
  - Fast and cheap to copy (just copy the value)
- No ownership move needed — both variables remain valid
- No difference between deep and shallow copy for these types
- `clone` is unnecessary; assignment is enough

## Copy Trait
- The `Copy` trait in Rust:
  - Can be applied to types stored on the stack (e.g., integers).
  - If a type implements `Copy`:
    - Assigning it to another variable copies the value, rather than moving it.
    - Both variables remain valid after assignment.

- Restrictions on `Copy`:
  - A type cannot implement `Copy` if it or any of its parts implements the `Drop` trait.
    - If a type needs special cleanup when it goes out of scope (`Drop`), it cannot also be `Copy`.
    - Attempting to do so results in a compile-time error.

- As a general rule, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy. Here are some of the types that implement Copy:
    - All the integer types, such as u32.
    - The Boolean type, bool, with values true and false.
    - All the floating-point types, such as f64.
    - The character type, char.
    - Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

## Concept: References

### Problem
How can we continue to use a variable after passing it to a function, given that its scope would otherwise end?

### Solution

1. **Using tuple:** We can pass and extract the same back from it. Like the below code.

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```
> NOTE: Since s1 gets expired, you can rename the `s2` variable to `s1`.

### Again problem!

What if there are multiple parameters? Will we be returning each and every parameter? That would be tedious.

### Solution

Answer is `References`. You will take a deep dive in the [next chapter](./42-references-and-borrowing.md).

## Questions

<details>
<summary>1. Is allocation to heap or stack under our direct control?</summary>

- Direct control: No, you cannot explicitly tell Rust to put something on the heap or stack.
- Indirect control: Yes, by choosing certain data structures.
    - Fixed-size arrays and simple structs are usually stored on the stack.
    - Smart pointers and collections (`Box`, `Vec`, `String`, `Rc`, `Arc`) allocate memory on the heap.
</details>

<details>
<summary>2. How does the allocation decision happen?</summary>

- At compile time, Rust checks if the size of a value is known and fixed.
- If the size is known and fixed, the value is stored on the stack.
- If the size is not known or is dynamic, Rust uses the heap.
- At runtime, heap allocations are handled by the system allocator (e.g., `malloc`, `jemalloc`).
- Rule of thumb:
    - Default: stack.
    - Using smart pointers or collections: heap.
</details>

<details>
<summary>3. In Rust, after <code>let s2 = s1;</code>, why is <code>s1</code> invalidated and not <code>s2</code>?</summary>

- Ownership of the heap data moves from `s1` to `s2`.
- Only one variable can own a value at a time.
- After the move, `s1` is invalid and cannot be used.
- `s2` becomes the new and only valid owner of the data.
- The whole point of assignment (let s2 = s1;) is to transfer ownership. So invalidating `s2` doesn't make sense.
</details>

<details>
<summary>4. Why doesn’t ownership move for primitives like <code>i32</code>, <code>char</code>, <code>bool</code>?</summary>

- Primitives have a fixed size known at compile time.
- They are stored entirely on the stack.
- When assigned or passed, they are simply copied (not moved).
- Copying is fast and inexpensive, so move semantics are not needed.
- There is no distinction between deep and shallow copy for these types.
</details>

<details>
<summary>5. Does Rust use a garbage collector?</summary>
- No, Rust does not have a garbage collector.
- Instead, Rust uses an ownership and borrowing system, checked at compile time, to ensure memory is freed exactly once.
- This approach avoids the runtime cost and unpredictability of garbage collection.
</details>

<details>
<summary>6. Is a garbage collector mandatory in programming languages?</summary>
- No, a garbage collector is not mandatory.
- Some languages (like Java, Go, and C#) require a garbage collector to manage memory automatically.
- Other languages (like C and C++) require the programmer to manually allocate and free memory.
- Rust provides a third approach: its ownership system automatically cleans up memory at the end of a value's scope, without a garbage collector or manual freeing.
</details>
