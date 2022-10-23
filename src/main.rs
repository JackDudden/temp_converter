use std::io;

const OFFSET: f64 = 32.0;
const FACTOR: f64 = 9.0 / 5.0;

fn main() {
    loop {
        println!("Please input the number to convert");

        let mut value = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line.");

        let unit = match value.trim().chars().last() {
            Some(char) => char,
            None => {
                println!("No unit included");
                continue;
            }
        };

        let value: f64 = match value.trim().get(0..value.len() - 2) {
            Some(s) => match s.parse() {
                Ok(v) => v,
                Err(err) => {
                    println!("{s}");
                    panic!("{err}");
                }
            },
            None => {
                println!("String cutting failed");
                continue;
            }
        };

        if !['F', 'C'].contains(&unit) {
            println!("Unit not valid, must be either Fahrenheit (F) or Celcius (C)");
            continue;
        } else if unit == 'C' {
            let result = (value * FACTOR) + OFFSET;
            println!("{result}F");
            break;
        } else {
            let result = (value - OFFSET) / FACTOR;
            println!("{result}C");
            break;
        }
    }
}
