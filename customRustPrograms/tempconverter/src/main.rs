use std::io;
fn main() {
    println!("One side Temperature Converter : Fahrenheit to Celsius");

    println!("Enter Fahrenheit Temperature : ");
    let mut curr_temp = String::new();

    io::stdin()
        .read_line(&mut curr_temp)
        .expect("Failed to read line!");

    let number: f32 = curr_temp.trim().parse().expect("Invalid Temperature!");

    let celsius = 5.0 / 9.0 * (number - 32.0);

    println!("The Celsius Temperature : {celsius}");
}
