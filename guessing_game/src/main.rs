extern crate rand; //Import our Random Crate with all our random functions

use std::io; //Calls the library for input handling
use std::cmp::Ordering; //Calls library for ordering and comparing
use rand::Rng; //Calls imported library Rng from our random crate.

fn main() {
    println!("Guess the number!"); //Print with new line

    let secret_number = rand::thread_rng().gen_range(1,101); // Let the variable 'secret_number' be an immutable variable that is a random 
	    													 //number between 1 and 100
	let mut count = 1; //Counter interger.

    loop {
	    let mut guess = String::new(); //Let the variable 'guess' be a mutable variable and set it equal to a new String.

	    println!("Please input your guess: "); //Print without new line
	    io::stdin().read_line(&mut guess).expect("Failed to read line.");

	    let guess: u32 = match guess.trim().parse() { //Parse guess into an unsigned-32 bit integer. trim() removes leading and following white 
	   												  //spaces and newlines.
	    	Ok(num) => num, //If it was a success, set guess to num.
	    	Err(_) => continue, //If it was an error, jump to the next loop.
	    };

	    println!("You guessed {}.", guess); //Printing to the screen with variables in the print statement.

	    //A match statement, similar to a switch statement. Uses the .cmp function, which returns an Ordering type based on the comparison.
	    //We use this ordering type to see if the number is less than, greater than, or equal to the secret number.
	    match guess.cmp(&secret_number) {
	    	Ordering::Less    => {
	    				      		println!("Guess is smaller then number.");
	    				      		count = count + 1;
	    					     }
	    	Ordering::Greater => {
	    				      		println!("Guess is larger then number.");
	    				      		count = count + 1;
	    					     }
	    	Ordering::Equal   => {
	    				      		println!("Your guess of {} matches the secret number. It took you {} guesses", guess, count);
	    				      		break;
	    					  	 }
	    }
    }
}
