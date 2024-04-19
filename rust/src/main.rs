use std::io;
fn main() {
    println!("Enter temperature in Fahrenheit");

    let mut temp_fah = String::new();

    // Read the user input from standard input 
    //(stdin) and store it in the string
    io::stdin()
    .read_line(&mut temp_fah)
    .expect("Failed to read line");

     // Convert the string to a f64 (floating point number)
    let temp_fah: f64 = temp_fah
    .trim()
    .parse()
    .expect("Please type a number!");

    // Call the conversion function
    let temp_cel = fahrenheit_to_celsius(temp_fah);
    println!("Temperature in Celsius: {}", temp_cel)
}

fn fahrenheit_to_celsius(fahrenteit: f64) -> f64 {
    //convert the Fahrenheit temp to Celsius
    let celsius: f64 = (fahrenteit - 32.0) * 5.0/9.0;
    celsius
}
