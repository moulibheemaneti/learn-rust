# Data Types

## Overview
- Rust is **statically typed** - all variable types must be known at compile time
- Two main data type subsets: **scalar** and **compound**

## Scalar Types
Represent single values. Four primary types:
- **Integers**
- **Floating-point numbers** 
- **Booleans**
- **Characters**

### Integer Types

<table><thead><tr><th>Length</th><th>Signed</th><th>Unsigned</th></tr></thead><tbody>
<tr><td>8-bit</td><td><code class="hljs">i8</code></td><td><code class="hljs">u8</code></td></tr>
<tr><td>16-bit</td><td><code class="hljs">i16</code></td><td><code class="hljs">u16</code></td></tr>
<tr><td>32-bit</td><td><code class="hljs">i32</code></td><td><code class="hljs">u32</code></td></tr>
<tr><td>64-bit</td><td><code class="hljs">i64</code></td><td><code class="hljs">u64</code></td></tr>
<tr><td>128-bit</td><td><code class="hljs">i128</code></td><td><code class="hljs">u128</code></td></tr>
<tr><td>architecture dependent</td><td><code class="hljs">isize</code></td><td><code class="hljs">usize</code></td></tr>
</tbody></table>

#### Key Points:
- **Signed** (`i8`, `i16`, etc.): store positive and negative numbers
- **Unsigned** (`u8`, `u16`, etc.): store only zero and positive numbers  
- **`isize`/`usize`**: depend on computer architecture (32/64-bit)
- **Range**: Signed: -(2^n-1) to 2^n-1 - 1, Unsigned: 0 to 2^n - 1
- **Example**: `i8` range is -128 to 127, `u8` range is 0 to 255

#### Integer Overflow
- **Debug mode**: Rust panics on overflow
- **Release mode**: Two's complement wrapping (256 becomes 0 for `u8`)
- **Safe methods**:
  - `wrapping_*` methods (e.g., `wrapping_add`)
  - `checked_*` methods return `None` on overflow
  - `overflowing_*` methods return value + boolean
  - `saturating_*` methods clamp to min/max values

### Floating-Point Types
- **`f32`**: 32-bit single precision
- **`f64`**: 64-bit double precision (default)

### Boolean Type
- **Values**: `true` or `false`
- **Size**: 1 byte

### Character Type (`char`)
- **Size**: 4 bytes (Unicode scalar value)
- **Can store**: letters, numbers, emojis, Unicode characters

```rust
let c = 'z';
let z: char = 'ℤ'; 
let heart_eyed_cat = '😻';
```

## Compound Types
Group multiple values into one type.

### Tuples
- **Fixed length** - cannot grow or shrink
- **Mixed types** allowed
- **Access methods**:
  - Destructuring: `let (x, y, z) = tup;`
  - Index access: `tup.0`, `tup.1`, `tup.2`

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;           // destructuring
let five_hundred = tup.0;      // index access
```

### Arrays
- **Fixed length** - cannot grow or shrink
- **Same type** for all elements
- **Less flexible** than vectors
- **Access**: by index `arr[0]`, `arr[1]`

```rust
let a = [1, 2, 3, 4, 5];      // explicit values
let b = [3; 5];               // [3, 3, 3, 3, 3]
let first = a[0];
```

#### Arrays vs Vectors
- **Arrays**: Fixed size, stack-allocated
- **Vectors**: Dynamic size, heap-allocated, growable
