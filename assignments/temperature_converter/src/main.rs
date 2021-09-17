use std::io;

fn main() {
    loop {
        println!("1. Convert from Celsius to Fahrenheit.");
        println!("2. Convert from Fahrenheit to Celsius.");

        let mut choice = String::new();
        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice != 1 && choice != 2 {
            println!();
            println!("Please enter a valid choice.");
            println!();
            continue;
        }

        println!();
        println!("Enter Temperature.");

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line.");

        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 1 {
            convert_celsius_to_fahrenheit(temperature);
            break;
        } else if choice == 2 {
            convert_fahrenheit_to_celsius(temperature);
            break;
        }
    }
}

fn convert_celsius_to_fahrenheit(temprature_celsius: f32) {
    let temprature_fahrenheit: f32 = (temprature_celsius * (9.0 / 5.0)) + 32.0;

    println!();
    println!(
        "{} Celsius is {} Fahrenhit",
        temprature_celsius, temprature_fahrenheit
    );
}

fn convert_fahrenheit_to_celsius(temprature_fahrenheit: f32) {
    let temprature_celsius: f32 = (temprature_fahrenheit - 32.0) / (9.0 / 5.0);

    println!();
    println!(
        "{} Fahrenheit is {} Celsius",
        temprature_fahrenheit, temprature_celsius
    );
}
