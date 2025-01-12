use std::io;

fn main() {
    loop {
        println!("Type a Fahrenheit temperature");
        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");
        let temperature: u32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!(
            "The temperature in Celsius is {}ºC",
            fahrenheit_to_celsius(temperature)
        );
        break;
    }
}
fn fahrenheit_to_celsius(temperature: u32) -> u32 {
    return (temperature - 32) * 5 / 9;
}
