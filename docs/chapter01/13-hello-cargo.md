# Hello Cargo

## New terms
1. **Build system**

**Definition**: A build system is a tool that automates the process of turning your source code into a finished program (an executable or library). It handles steps like compiling code, linking files, and managing dependencies.

**Example**: In Rust, Cargo is the build system. In C, you might use Make.

**Explanation**: Imagine you’re baking a cake. The build system is like a recipe and a kitchen assistant—it tells you what steps to follow and does the mixing, baking, and decorating for you, so you get a finished cake (your program) every time.


## Keywords

1. **Cargo**: It is Rust's build system and package manager.

2. **TOML**
    - TOML stands for "Tom’s Obvious, Minimal Language." It is a simple, easy-to-read file format used for configuration files.
    - Rust projects use `Cargo.toml` (in TOML format) to list project info and dependencies, like `package.json` in JavaScript.
    - Think of TOML as a recipe card for your project. It lists the ingredients (dependencies) and instructions (settings) in a clear, organized way, so Cargo knows how to build and manage your project.

3. **Crates**:
    - In Rust, packages of code are referred to as `crates`.

## Key concepts

1. By default, `cargo` comes with rustup. To crosscheck whether you have it or not:
```bash
cargo --version
```

2. Cargo skips creating a .gitignore if git is already set up in the project. To override it and create a new .gitignore, use:
```bash
cargo new --vcs=git
```

3. **Best practice:** Keep code in the `src` folder; put configs and docs in the project root.

## Creating `Cargo` project

If you want to create a new cargo project, run the following command:
```bash
# cargo new <project_name>
cargo new hello_cargo
```

If you want to add cargo to an existing rust files project, run the following command:

```bash
cargo init
```

> **Note:**  
> - When you create a new Cargo project, your Rust source files should be placed inside the `src` directory (for example, `src/main.rs`).  
> 
> - If you have existing Rust files in the top-level project directory, move them into the `src` folder so that Cargo can find and build them correctly.

## Building and Running a Cargo project

To build the cargo project
```bash
cargo build
```

To run the built project
```bash
# ./target/debug/<project_name>
./target/debug/hello_cargo
#./target/debug/hello_cargo.exe # in Windows
```

To build and run in one step, use:
```bash
cargo run
```

Check if code compiles without building an executable:
```bash
cargo check
```

## Build for release
```bash
cargo build --release
```
> **Notes:**
> 1. Creates `target/release` folder instead of `target/debug`
> 2. When you build in release mode, Cargo automatically applies optimizations to make your program run faster and more efficiently.


## Questions

<details>
<summary>
Why would you want to check if code compiles without building an executable?
</summary>
Sometimes, you just want to quickly check for errors in your code (like syntax or type errors) without spending time generating the final executable file. This is especially useful for large projects where building can take a while. The <code>cargo check</code> command helps you catch mistakes early and speeds up your workflow.
</details>
