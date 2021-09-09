fn main() {
    // Variables and Mutability

    // variables are immutable by default.
    // let x = 5;
    // println!("The value of x is: {}", x);

    // x = 6; // error: cannot assign twice to immutable variable `x`
    // println!("The value of x is: {}", x);

    // Declaring a mutable variable.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);


    // Difference between constants and variables
    
    // - constants are always immutable.
    // - type of value must be annotated.
    // - constants can be declared in any scope.
    // - constants should only be set to a constant expression, not to a result of a fucntion call or
    //   any other value that could only be computed at runtime.

    // Declaring a constant variable.
    const MAX_POINTS: u32 = 100_000;
    // underscores can be inserted in numeric literals to improve readability.
    // constants are valid for the entire time a program runs, within the scope they were declared
    // in.


    // Shadowing

    // you can declare a new varible with the same name as a previous variable.
    // first varibale is shadowed by the second, which means that the second variable's value is
    // what appears when the variable is used.
    let y = 5;

    let y = y + 4;

    let y = y * 2;

    // Shadowing is different from marking the variable as `mut` , because we'll get a compile time
    // error if we accidentally try to reassign to this varible without using the `let` keyword.
    // By using `let`, we can perform a few transformations on a value but have the variable be
    // immutable after those transformations have been completed.

    // The other difference between `mut` and shadowing is that because we're effectively creating
    // a new variable when we use the `let` keyword again, we can change the type of the value but
    // reuse the same name.

    // let mut spaces = "   ";
    // spaces = spaces.len(); // error: mismatched types.

    let spaces = "   "; // type: &str
    let spaces = spaces.len(); // type: usize
}
