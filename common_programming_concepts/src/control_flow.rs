fn main() {
    // Control Flow

    // if Expressions

    let number = 5;

    if number < 3 {
        println!("condition was true.");
    } else {
        println!("condition was false.");
    }

    // The condition in the if statement must be a `bool`. If the condition isn't a `bool`, we'll
    // get an error.

    // if number { // error: mismatched types  expected `bool`, found integer
    //     println!("The number was 5");
    // }

    // Rust will not automatically try to convert non-Boolean types to a Boolean.

    // Using if in a let Statement

    // Because `if` is an expression, we can use it on the right side of a `let` statement.

    let condition = true;
    let number = if condition { 5 } else { 6 };

    // values that have potential to be result from each arm of the `if` must be the same type;

    // let number = if condition { 5 } else { "six" }; // error: `if` and `else` have incompatible types

    // The expression in the `if` block evaluates to an integer, and the expression in the `else`
    // block evaluates to a string. This won't work because variables must have a single type. Rust
    // needs to know at compile time what type the `number` variable is.


    // Repetition with Loops

    // Rust has three kinds of loops: `loop`, `while`, `for`.

    // Repeating Code with loop
    //
    // The `loop` keyword tells Rust to execute a block of code over and over again forever until
    // you explicitly tell it to stop.
    //
    // You can place the `break` keyword within the loop to tell the program when to stop executing
    // the loop.

    // Returning Values from Loops

    // One of the uses of a `loop` is to retry an operation you know might fail, such as checking
    // wheather a thread has completed its job. However, you might need to pass the result of that
    // operation to the rest of your code.

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    // Conditional Loops with while

    let mut number = 3;

    while number > 0 {
        println!("{}!", number);
        number -= 1;
    }

    // Looping Through a Collection with for

    let a = [19, 34, 53, 2, 23];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}
