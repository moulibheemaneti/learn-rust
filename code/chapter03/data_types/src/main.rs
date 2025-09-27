fn main() {
    println!("=== RUST DATA TYPES DEMONSTRATION ===\n");

    // ========================================
    // SCALAR TYPES
    // ========================================

    println!("1. INTEGER TYPES");
    println!("==================");

    // All integer types
    let a: i8 = -128;
    let b: u8 = 255;
    let _c: i16 = -32768;
    let _d: u16 = 65535;
    let e: i32 = -2147483648;
    let f: u32 = 4294967295;
    let _g: i64 = -9223372036854775808;
    let _h: u64 = 18446744073709551615;
    let _i: i128 = -170141183460469231731687303715884105728;
    let _j: u128 = 340282366920938463463374607431768211455;
    let k: isize = -9223372036854775808; // depends on architecture
    let l: usize = 18446744073709551615; // depends on architecture

    println!("i8 range example: {}", a);
    println!("u8 range example: {}", b);
    println!("i32 example: {}", e);
    println!("u32 example: {}", f);
    println!("isize example: {}", k);
    println!("usize example: {}", l);

    // Integer overflow examples (safe methods)
    println!("\nInteger Overflow Safe Methods:");
    let x: u8 = 250;
    let y: u8 = 10;

    // This would panic in debug mode, but we use safe methods
    let wrapped = x.wrapping_add(y); // 250 + 10 = 4 (wraps around)
    let checked = x.checked_add(y); // Returns Some(4) or None
    let (result, overflowed) = x.overflowing_add(y); // (4, true)
    let saturated = x.saturating_add(y); // 255 (clamped to max)

    println!("wrapping_add: {}", wrapped);
    println!("checked_add: {:?}", checked);
    println!("overflowing_add: ({}, {})", result, overflowed);
    println!("saturating_add: {}", saturated);

    // ========================================
    // FLOATING-POINT TYPES
    // ========================================

    println!("\n2. FLOATING-POINT TYPES");
    println!("========================");

    let float_32: f32 = 3.14159;
    let float_64: f64 = 2.718281828459045; // default type

    println!("f32 example: {}", float_32);
    println!("f64 example: {}", float_64);
    println!("Default float type: {}", 3.14); // f64 by default

    // ========================================
    // BOOLEAN TYPE
    // ========================================

    println!("\n3. BOOLEAN TYPE");
    println!("================");

    let is_rust_cool: bool = true;
    let is_java_better: bool = false;

    println!("Is Rust cool? {}", is_rust_cool);
    println!("Is Java better? {}", is_java_better);
    println!("Boolean size: {} bytes", std::mem::size_of::<bool>());

    // ========================================
    // CHARACTER TYPE
    // ========================================

    println!("\n4. CHARACTER TYPE");
    println!("==================");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    let number_char = '5';
    let unicode_char = '中'; // Chinese character

    println!("ASCII character: {}", c);
    println!("Mathematical symbol: {}", z);
    println!("Emoji: {}", heart_eyed_cat);
    println!("Number as char: {}", number_char);
    println!("Unicode character: {}", unicode_char);
    println!("Character size: {} bytes", std::mem::size_of::<char>());

    // ========================================
    // COMPOUND TYPES
    // ========================================

    println!("\n5. COMPOUND TYPES");
    println!("==================");

    // ========================================
    // TUPLES
    // ========================================

    println!("\nTUPLES");
    println!("------");

    // Tuple declaration with type annotation
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple: {:?}", tup);

    // Tuple without type annotation (type inference)
    let tup2 = (42, 3.14, 'A', true);
    println!("Mixed tuple: {:?}", tup2);

    // Destructuring tuples
    let (x, y, z) = tup;
    println!("Destructured: x={}, y={}, z={}", x, y, z);

    // Accessing tuple elements by index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!(
        "By index - first: {}, second: {}, third: {}",
        five_hundred, six_point_four, one
    );

    // Empty tuple (unit type)
    let empty_tuple = ();
    println!("Empty tuple: {:?}", empty_tuple);

    // ========================================
    // ARRAYS
    // ========================================

    println!("\nARRAYS");
    println!("------");

    // Array with explicit values
    let a = [1, 2, 3, 4, 5];
    println!("Array: {:?}", a);

    // Array with type annotation
    let b: [i32; 5] = [10, 20, 30, 40, 50];
    println!("Typed array: {:?}", b);

    // Array with repeated values
    let c = [3; 5]; // [3, 3, 3, 3, 3]
    println!("Repeated array: {:?}", c);

    // Array with different types (all same type required)
    let chars = ['a', 'b', 'c', 'd'];
    println!("Char array: {:?}", chars);

    // Accessing array elements
    let first = a[0];
    let second = a[1];
    let last = a[4];

    println!("First element: {}", first);
    println!("Second element: {}", second);
    println!("Last element: {}", last);

    // Array bounds checking (will panic at runtime if out of bounds)
    // let out_of_bounds = a[10]; // This would cause a panic!

    // ========================================
    // ARRAYS vs VECTORS (brief mention)
    // ========================================

    println!("\nARRAYS vs VECTORS");
    println!("==================");

    // Array: fixed size, stack-allocated
    let fixed_array = [1, 2, 3, 4, 5];

    // Vector: dynamic size, heap-allocated (from std library)
    let dynamic_vector = vec![1, 2, 3, 4, 5];

    println!("Fixed array: {:?}", fixed_array);
    println!("Dynamic vector: {:?}", dynamic_vector);
    println!("Array size: {} bytes", std::mem::size_of_val(&fixed_array));
    println!(
        "Vector size: {} bytes",
        std::mem::size_of_val(&dynamic_vector)
    );

    // ========================================
    // TYPE INFERENCE EXAMPLES
    // ========================================

    println!("\n6. TYPE INFERENCE");
    println!("==================");

    // Rust can often infer types
    let inferred_int = 42; // i32 by default
    let inferred_float = 3.14; // f64 by default
    let inferred_bool = true; // bool
    let inferred_char = 'A'; // char

    println!("Inferred int: {} (type: i32)", inferred_int);
    println!("Inferred float: {} (type: f64)", inferred_float);
    println!("Inferred bool: {} (type: bool)", inferred_bool);
    println!("Inferred char: {} (type: char)", inferred_char);

    println!("\n=== END OF DEMONSTRATION ===");
}
