use std::io;
fn main() {
    println!("Welcome to the Calculator!");

    println!("Enter the first number -- A : ");
    let mut a = String::new();
    
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line!");
    

    let a : i32 = a
        .trim()
        .parse()
        .expect("Invalid Number, Re-Enter Please!");

    println!("Enter the Second number -- B : ");
    let mut b = String::new();

    io::stdin()
    .read_line(&mut b)
    .expect("Failed to read line!");

    let b : i32 = b
        .trim()
        .parse()
        .expect("Invalid Number, Re-Enter Please!");
    
    
    println!("Enter the operator (+, -, *, /): ");
    let mut opreator = String::new();

    io::stdin()
    .read_line(&mut opreator)
    .expect("Failed to read line!");

    let opreator = opreator.trim();

    let result = match opreator {
        "+" => Some(a + b),
        "-" => Some(a - b),
        "*" => Some(a * b),
        "/" => {
            if b != 0 {
                Some(a / b)
            } else {
                println!("This'll be infinte!");
                None
            }
        }
        _ => {
            println!("Enter a Valid Opreator, Please!!");
            None
        }
    };
    

    if let Some(res) = result {
        println!("The result of {} {} {} is: {}", a, opreator, b, res);
    }
    
}