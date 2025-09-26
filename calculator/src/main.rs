use anyhow::{Result, Context};
use std::io;

// Input Helper
fn read_num(prompt: &str) -> Result<f64>
    {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).context("Failed to read the Input!")?;

        let num: f64 = input.trim().parse().context("Please enter a valid Integer or Float!")?;

        Ok(num)
    }



fn main() -> Result<()>
    {
        println!("Welcome to Terminal Calculator");

        loop
        {


            println!("Whafunction would you like to invoke!?");
            println!("ADD | SUBTRACT | MULTIPLY | DIVIDE | QUIT");

            //Input handler

            let mut func = String::new();

            io::stdin().read_line(&mut func).context("Failed to read the Input!")?;

            let func = func.trim().to_lowercase();

            match func.as_str()
            {
                "add" =>
                {
                    println!("ADDITION!");
                    let num1 = read_num("Enter the First Number:")?;
                    let num2 = read_num("Enter the Second Number:")?;
                    println!("{num1} + {num2} = {}", num1 + num2);
                }
                "subtract" =>
                {
                    println!("SUBTRACTION");
                    let num1 = read_num("Enter the First Number:")?;
                    let num2 = read_num("Enter the Second Number:")?;
                    println!("{num1} - {num2} = {}", num1 - num2);
                }
                "multiply" =>
                {
                    println!("MULTIPLICATION");
                    let num1 = read_num("Enter your First Number:")?;
                    let num2 = read_num("Enter your Second Number:")?;
                    println!("{num1} * {num2} = {}", num1 * num2);
                }
                "divide" =>
                {
                    println!("DIVITION");
                    let num1 = read_num("Enter your First Number:")?;
                    let num2 = read_num("Enter your Second Number:")?;
                    println!("{num1} / {num2} = {}", num1 / num2);
                }
                "quit" =>
                {
                    println!("Exiting.....");
                    break;
                }
                _ =>
                {
                    println!("Please select a Valid choice from the List");
                    println!("ADD | SUBTRACT | MULTIPLY | DIVIDE | QUIT");
                }
            }
        }
        Ok(())




    }
