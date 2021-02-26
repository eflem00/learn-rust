use std::io;

fn main() {
    println!("Please provide a temperature in Farenheit");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("input not provided");

        let farenheit: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        let celcius = (farenheit - 32f64) * (5f64 / 9f64);

        println!("{:.2} farenheit is {:.2} celcius", farenheit, celcius);
    }
}
