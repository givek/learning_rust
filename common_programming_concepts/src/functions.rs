fn main {
    // Functions

    // Function Bodies Contain Statements and Expressions
    //
    // Function bodies are made up of statements optionally ending in an expression. Because Rust
    // is an expression based-language, this is an important distinction to understand.

    // - Statements are instructions that perform some action and do not return a value.

    let y = 6; // is a statement

    // Function definitions are also statements.
    // Statements do not return values. Therefore, you cannot assign a `let` statement to another
    // variable.

    // let x = (let z = 8); error: expected expression, found statement

    // The `let z = 8` statement does not return a value, so there isn't anything for `x` to bind
    // to. This is different from what happens in other languages, such as C and Ruby, where the
    // assignment returns the value of the assignment.
    // In those languages, you can write `x = y = 6` and have both `x` and `y` have the value `8`.

    // - Expressions evaluate to a resulting value.
    //
    // Consider a simple mathn operation, such as `5 + 6`, which is an expression that evaluates to
    // the value `11`.
    // Expression can be part of statement.
    // Calling a fucntion is an expression. Calling a macro is an expression. The block that we use
    // to create new scopes, `{}` is an expression, for example:

    let e = {
        let s = 3;
        s + 1 // return value
    };

    println!("The value of e is: {}", e);

    // Functions with Return Values
    //
    // Functions can return values to the code that calls them.
    // Type of the return value is declared after an arrow (`->`).
    // In Rust, the return value of the fucntion is synonymous with the value of the final
    // expression in the block of the body of a function.
    // You can return early from a fucntion by using the `return` keyword and specifying a value.

    let f = plus_one(4);

    println!("{}", f);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
