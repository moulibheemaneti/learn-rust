# Hello World

## Rust file
Ends with `.rs` extension.

## Binary Exectuatble file
To generate a corresponding executable binary file:
```bash
rustc main.rs
```

> **NOTE:**  
> - **On Unix:** the executable has no extension (`main`)  
> - **On Windows:** the executable ends with `.exe` (`main.exe`)

## Assembly file
To generate a corresponding assembly file:
```bash
rustc --emit asm main.rs
```

> **NOTE:**  
> - **Generated file:** `main.s`

## Key concepts

1. Rust files always end with the .rs extension
2. Use underscores to separate words in filenames.

| ✅ **Valid**                | ❌ **Invalid**             |
|-------------------------|-------------------------|
| `hello_world.rs`        | `helloworld.rs`         |
| `my_first_program.rs`   | `HelloWorld.rs`         |
|                         | `my first program.rs`   |

3. **rustc**: The Rust compiler.  
    a. It compiles Rust code into a binary executable file (a machine code program you can run directly).  
    
    b. No matter your OS, you should see `Hello, world!` in the terminal.

4. main function is the entry point for a executable rust program.

5. If you see an exclamation mark `!` after a name (like `println!`), it means you are calling a macro, not a regular function. Macros work differently from functions in Rust.
