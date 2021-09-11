fn main(){
    // Data Types

    // Rust is a statically typed language, which means that it must know the types of all
    // variables at compile time.

    // In cases when many types are possible, such as when converting a `String` to a numeric type,
    // we must add type annotation.

    // let guess = "42".parse().expect("Not a number!"); // error: type annotations needed.

    let guess: u32 = "42".parse().expect("Not a number!");

    // Scalar Types

    // A scalar type represents a single value. Rust has four primary scalar types:
    // 1. integers
    // 2. floating-point numbers
    // 3. Booleans
    // 4. characters

    // Integer Types in Rust:
    //
    //  Lenght    Signed   Unsigned
    //  8-bit     i8       u8
    //  16-bit    i16      u16
    //  32-bit    u32      i32
    //  64-bit    u64      i64
    //  128-bit   u128     i128
    //  arch      isize    usize
    //
    // Signed variant can store numbers from    -(2^(n-1)) to 2^(n-1) - 1 (inclusive)
    // Unsigned variant can store numbers from  0 to 2^n - 1              (inclusive)
    //
    // Additionally, the `isize` and `usize` types depends on the kind architecture:
    // - 64 bits on 64-bit architecture.
    // - 32 bits on 32-bit architecture.

    // Floating-Point Types
    //
    // Rust's floating-point types are:
    //
    //  Lenght    Type
    //  32-bit    f32
    //  64-bit    f64
    //
    //  By default `f64` type is used, because on modern CPUs it's roughly the same speed as `f32`
    //  but is capable of more precision.

    let x = 2.0; // f64

    let y: f32 = 2.0; // f32

    // Boolean Type

    let t = true;

    let f: bool = false; // with explicit type annotation.

    // The Character Type
    //
    // `char` literals are specified with single quotes.

    let c = 'z';

    let z = 'Z';

    // Rust's `char` type is four bytes in size and represents a Unicode Scalar Value.


    // Compound Types

    // Compound types can group multiple values into one type. Rust has two primitive compound
    // types:
    // 1. Tuples
    // 2. Arrays

    // The Tuple Type
    //
    // A tuple is a general way of grouping together a number of values with a variety of types
    // into one compound type.
    // Tuples have a fixed lenght: once declared, they cannot grow or shrink in size.

    let my_tup: (i32, f64, u32) = (2, 5.3, 43);

    // use pattern matching to destructure a tuple value:

    let (p, q, r) = my_tup;

    println!("The value of y is: {}", y);

    // In addition to destructuring through pattern matching, we can access a tuple element
    // directly by using a period (.) followed by the index of the value we want to access.

    let two = my_tup.0;

    let five_point_three = my_tup.1;

    let fourty_three = my_tup.2;

    // The Array Type
    //
    // An Array is a collection of multiple values with the same type.
    // In Rust Array have fixed lenght, like tuples.

    let list_a = [1, 2, 3, 4];

    let list_b: [u32; 4] = [1, 2, 3, 4];

    // Here, `u32` is the type of each element. After the semicolon, the number `4` indicates the
    // size of the array
    //
    // Here, `u32` is the type of each element. After the semicolon, the number `4` indicates
    // the size of the array.

    // Initialize the Array with all values set to 44;
    let list_c: [i32; 10] = [44; 10];

    println!("{:?}", list_c); // print an Array

    // Arrays are allocated on stack.
    //
    // If an out of bound index is accessed the program panic's and results in a runtime error.
    // This check has to happen at runtime at runtime, especially in a case when user enters the
    // value of the index to be accessed, beacuse the compiler can't possibly know what value a
    // user will enter when they run the program.
}
