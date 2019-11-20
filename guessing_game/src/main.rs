//Libary to obtain user input
use std::io;
//Library to generate random numbers
use rand::Rng;
//Library to allow comparisons
use std::cmp::Ordering;
//Main function
fn main(){

    println!("Guess the number!");
    //Generates a random number and stores in secret_number from 1 - 101 exclusive
    let secret_number = rand::thread_rng().gen_range(1,101);
    //Allows the user to guess multiple times
    loop{
        //Print lines that prompts user to input a number
        println!("Please input your guess:");
        //Variable to store user input
        let mut guess = String::new();
        //Reads in the line with an error if no line is read in
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");
        //Converts the string into an int
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        //Prints the user input
        println!("You guessed: {}", guess);
        //Compares the guess to secret_number and tells user if it
        //is too small or too large
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
