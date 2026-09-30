use std::io::{self, Write};

fn print_ascii() {
    println!(r#"
  ___ _____
 | _ \_   _|__ _ __  _ __
 |   / | |/ -_) '  \| '_ \
 |_|_\ |_|\___|_|_|_| .__/
                    |_|
1 - fahrenheit to celsius
2 - celsius to fahrenheit
3 - exit
"#);
}

fn main() {
    print_ascii();
    loop {
        print!("rtemp> ");
        io::stdout().flush().expect("error");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error");

        match input.trim() {
            "1" => {
                println!("fahrenheit to celsius");
                print!("fahrenheit> ");
                io::stdout().flush().expect("error");

                let mut fahrenheit_input = String::new();
                io::stdin()
                    .read_line(&mut fahrenheit_input)
                    .expect("error");

                if let Ok(fahrenheit) = fahrenheit_input.trim().parse::<f64>() {
                    let result = (fahrenheit - 32.0) / 1.8;
                    println!("result: {:.2}°C", result);
                } else {
                    println!("Please enter a valid number.");
                }
            }
            "2" => {
                println!("celsius to fahrenheit");
                print!("celsius> ");
                io::stdout().flush().expect("error");

                let mut celsius_input = String::new();
                io::stdin()
                    .read_line(&mut celsius_input)
                    .expect("error");

                if let Ok(celsius) = celsius_input.trim().parse::<f64>() {
                    let result = (celsius * 1.8) + 32.0;
                    println!("result: {:.2}°F", result);
                } else {
                    println!("Please enter a valid number.");
                }
            }
            "3" => {
                println!("goodbye");
                break;
            }
            _ => {
                println!("Invalid option.");
            }
        }

        println!();
    }
}
