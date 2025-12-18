fn main() {
    scalar_types();
}

fn scalar_types() {
    // Scalar types represents a single value
    // There are 4 types: integers, floating-point numbers, booleans and characters
    integer_types();
}

fn integer_types() {
    // Unsigned integers: u8, u16, u32, u64, u128, usize
    // Signed integers: i8, i16, i32, i64, i128, isize
    //
    // Unsigned integers can only be positive
    // Signed integers can be positive or negative (those numbers thatneeds to have a sign with it)
    // The size of the integer is determined by the architecture of the computer
    // Each signed variant can store numbers from −(2n − 1) to 2n − 1 − 1 inclusive, where n is the number of bits that variant uses. So an i8 can store numbers from −(27) to 27 − 1, which equals −128 to 127. Unsigned variants can store numbers from 0 to 2n − 1, so a u8 can store numbers from 0 to 28 − 1, which equals 0 to 255.
    // Examples:
    let a: u8 = 255;
    let b: i8 = -128;
    let c: u32 = 1_000_000;
    let d: isize = -42;

    println!("u8 example: {a}");
    println!("i8 example: {b}");
    println!("u32 example: {c}");
    println!("isize example: {d}");
}

fn floating_point_types() {
    // Floating point types represent numbers with fractional parts.
    // The two primitive floating point types in Rust are:
    // - f32: 32-bit single-precision floating point.
    // - f64: 64-bit double-precision floating point (default for floating point literals).
    // Floating point numbers are represented according to the IEEE-754 standard.
    // Common operations (+, -, *, /) are available.
    // Beware of precision errors when using floats for critical calculations!    // Example:
    let x = 2.0; // f64 by default
    let y: f32 = 3.0;

    println!("Default float (x): {x}");
    println!("Explicit f32 (y): {y}");
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}

fn boolean_types() {
    // Boolean types in Rust are simple and straightforward.
    // There is only one boolean type: bool.
    // It can have the values true or false.
    // Useful in conditionals, control flow (if, while), etc.

    let is_rust_fun: bool = true;
    let is_bug_free = false; // Type inferred as bool

    println!("Is Rust fun? {is_rust_fun}");
    println!("Is everything bug free? {is_bug_free}");    
}

fn char_type() {
    // The `char` type in Rust is a single Unicode scalar value.
    // It's always 4 bytes (32 bits) in size and can represent
    // a lot more than just ASCII characters—accents, emoji, and others.
    // Use single quotes to specify: 'a', 'ß', '中', '🦀', etc.
    // Can be useful for working with text, parsing, or matched patterns.

    let letter: char = 'R';
    let emoji = '🦀';
    let greek = 'Ω';
    let newline = '\n';

    println!("Regular char: {letter}");
    println!("Emoji char: {emoji}");
    println!("Greek char: {greek}");
    println!("Char for newline: {:?}", newline);
}

fn compound_types() {
    // Tuples: group values of different types into one compound type. Useful for returning multiple values from a function or logically grouping data.

    // Tuples have a fixed length. Each element can have a different type.
    // Syntax: let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring a tuple:
    let (x, y, z) = tup;
    println!("Destructured tuple: x = {x}, y = {y}, z = {z}");

    // Access tuple elements directly by index:
    println!("Second element (y) with index: {}", tup.1);

    // Example: Function returning a tuple
    fn min_max(numbers: &[i32]) -> (i32, i32) {
        let mut min = numbers[0];
        let mut max = numbers[0];
        for &num in numbers.iter() {
            if num < min { min = num; }
            if num > max { max = num; }
        }
        (min, max)
    }

    let nums = [4, 2, 7, 1, 9];
    let (mi, ma) = min_max(&nums);
    println!("Min: {mi}, Max: {ma}");

    // Arrays: fixed-size, same-type, contiguous elements in memory.
    // Syntax: let arr = [1, 2, 3, 4, 5];
    // Type: [T; N], where T = element type, N = length (fixed at compile time).

    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    println!("Whole array: {:?}", arr);

    // Access by index (zero-based)
    println!("First element: {}", arr[0]);
    println!("Last element: {}", arr[4]);

    // Arrays can be initialized with the same value: [val; len]
    let zeros = [0; 8];
    println!("Zero array: {:?}", zeros);

    // Useful for stack-allocated, predictable-size data (vs Vec for dynamic).
    // Safe: Index OOB panics at runtime, not UB.
    // Example: Get array length
    println!("Array length: {}", arr.len());

    // Iterate over arrays
    for val in arr.iter() {
        print!("{val} ");
    }
    println!();
}
