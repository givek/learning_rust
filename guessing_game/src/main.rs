use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess: ");

        // `::` syntax in `::new` line indicates that `new` is a associated function on the `String` type.
        // A associated function is implemented on a type, in this case `String`, rather than on a
        // particular instance of `String`.
        // Some languages call this a static method.
        let mut guess = String::new();

        // The & indicates that this argument is a reference, which gives you a way to let multiple parts
        // of your code access one piece of data without needing to copy that data into memory multiple times.
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line.");

        // The job of read_line is to take whatever the user types into standard input and place that into a string,
        // so it takes that string as an argument.
        // The string argument needs to be mutable so the method can change the string’s content by adding the user input.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // The underscore, _, is a catchall value;
                                // in this example, we’re saying we want to match all Err values, no matter what information they have inside them.
        };
        println!("You've guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
