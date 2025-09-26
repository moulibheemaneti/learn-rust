# Installation

## Keywords

1. **rustup**: The Rust toolchain installer and version manager. It allows you to easily install, update, and manage different versions of Rust and associated tools.

2. **linker**: Joins compiled code into a single file. If you see linker errors, install a C compiler (it includes a linker), as some Rust packages need it.

## Steps to install
1. Run the following:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. (MacOS Only) Install linker

For MacOS, C Compiler (which indirectly has linker that is required for rust) is available in XCode.

```bash
xcode-select --install
```

3. Check for successful installation
```bash
rustc --version
```

## Steps to update

```bash
rustup update
```

## Steps to update

```bash
rustup self uninstall
```


## Questions

<details>
<summary>1. Why do we need a linker?</summary>
A linker is used to join compiled code (object files) into a single executable file. It resolves references between different parts of the program.
</details>

<details>
<summary>2. Are linkers the same for both <code>C</code> and <code>Rust</code>?</summary>
No, the linker can be different for C and Rust. While both languages use a linker to combine compiled code, the specific linker used may vary depending on the toolchain and platform.
</details>

<details>
<summary>3. Why do we need a C compiler?</summary>
Some common Rust packages depend on C code. A C compiler is needed to build these dependencies, as it provides the necessary tools (including a linker) to compile and link C code with your Rust project.
</details>
