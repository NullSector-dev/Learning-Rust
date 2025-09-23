use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!!");

    // Random number Logic.
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("HINT: The Secret Number is {secret_number}");

    println!("Enter Your Number!");
    //Loop
    loop{

        let mut guess = String::new();

        //Input Logic
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //Parsing Logic
        let guess: u32 = match guess.trim().parse() { Ok(num) => num, Err(_) =>  {println!("Please Type in a Number"); continue;}, };

        // Guess and Comparison Logic.
        println!("You Guessed : {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("The secret number is {secret_number}. You guessed too Small, You Lose!");
                println!("Good Attempt, try again later!");
                break;
            }
            Ordering::Greater => {
                println!("The secret number is {secret_number}. You guessed too Big, You Lose!");
                println!("Good Attempt, try again later");
                break;
            }
            Ordering::Equal => {
                println!("The secret number is {secret_number}. You guessed correct!, You Win!");
                break;
            }
        }
    }
}
