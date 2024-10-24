use std::io; //that's a library - technically called a prelude or preprocessor directive/Including extra features.
use rand::Rng; // here 'Rng' is a trait which defines a function and implements it for random number functionality
use std::cmp::Ordering; //here ordering is a enum type and has variants like greater, less and equal - ordering is directly related to the cmp - comparing module!
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(0..=100); // here thread::rng function gives us the random number : one that is local to the current thread execution and seeded in the OS. and then another function gen_range which genrates a number and return back it to the thread_rng() - here arguement is passed through the following syntax : (start..=end) that inclusively add the last values. this means if (1..=100) that means 1 and 100 are also included!

    println!("The secret number is {secret_number}");

    println!("Please enter a number here : ");

    let mut guess = String::new(); //in rust var are unchangeable - but you can make them mutable by adding a predefined keyword 'mut' - stands for mutable, this gives extrafunctionality to let to make it changeable! moreover, 'string::new()' is a function that returns a object/instance of type string provided by standard library template! here 'new' keyword creates function with the string type empty return object! --- overall this line has created a variable which is mutable named guess which is directly in bound with the empty instance of string type!

    io::stdin() //that thing directly related to the 'use std::io' line cause it imports functionlaity of recieving value from the user end  here 'io' referred as module, as per the variable system - 'stdin' returns a instance of the std::io - to handle the standard input for your terminal

        .read_line(&mut guess) // here 'read_line(&mut guess)' a method which directly taking input of string type from the user end storing it directly to the mutable variable '&mut guess' - the string argument needs to be mutable so it'll be easier for the method to change the content of the string's content - over all the '&' - reference opreator telling us that change will be reflected to the original variable not as copied! - but here's a trick to enable referecing are characterized by the keyword 'mut' - '&mut guess' is a refernce to mutable guess but if '&guess' - this one is default and reference to immutable guess. write mut to make it mutable that's it 

        .expect("Failed to read line"); // that's also a part of the io::stdin() - this may results the error handling info if the above method releases the invalid error response, that's not a function - that's a enum type 'result' - defaultly this result got two variants one is 'ok' and one is 'err' - if successfull returns ok otherwise err - but these values are returned from the function 'read_line' - if read_line method return ok to expect it'll not be executed, but if due to some reasons it results into error it'll return 'err' and the expect will be executed with the provided msg. using this expect is something that usally depends on you but if you don't use that you'll recieve an waring after the execution. you must have a habit of handling errors at the sametime cause it reduces the number of the errors in your program - that's all!

    let guess: u32 = guess.trim().parse().expect("please type a number!"); // you can declare the a variable multipul times with same name it allowed by the rust - here the second guess is the orginal string got into string - eventully that's an string we must need to trim it and must removes the whitespace all the way and makes it comparable to u32 which is a numerical value w/out any whitspace 

    println!("You Guessed the {guess}"); // that's again and print statement (here '!' is a macro which prints the string to the terminal) and then we've got a round braces which got a string msg to print along with a placeholder - some curly braces which gonna hold the sum value returned by the variable guess ok - that place holder indicates where you've to print that value to your msg!

    match guess.cmp(&secret_number) { // you can compare anything which is comparable and like here - guess number is comparable to secret number - then it return a variant from the ordering - to call both functionally we use the keyword 'match' which indirectly related to the both cmp and ordereing as well and returns the valid variant which matches the condition (guess === secret_number) - match uses the arms pattern : basically the less,equal and greater are variants for the ordering but eventually for match they're considerd as the arms - on matching them whenever it completely matches the condition returns that arm and ignore all the arms - and the matched arm pattern which asscociated with the conditon will be executed.

        Ordering::Less => println!("Too Small!"),
        Ordering::Equal => println!("You winn!"),
        Ordering::Greater => println!("Too Big!"),
    }
}


//use of the crates - let's assume that you want a program that guesses the secret number generated by the rust program itself but it can't be done itself to add this functionality to our program we must need to add the crate - some other code which can be imported to our code and increases the functionality of the code! - they're treated as the dependencies and putted to the file named 'cargo.toml' which contains all the dependices and other factors!

