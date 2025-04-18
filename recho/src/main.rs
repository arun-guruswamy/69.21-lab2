use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Get command line args
    let args: Vec<String> = env::args().collect();

    // Check if too many arguments were provided
    if args.len() > 3 {
        return Err("Too many arguments were provided".into());
    }

    // Handle -n flag if it exists
    if args.len() == 3 {
        if args[1] == "-n" {
            print!("{}", args[2]);
        } else {
            return Err(format!("{} is not a valid flag", args[1]).into());
        }
    } 

    // Handle single argument
    if args.len() == 2 && args[1] != "-n" {
        println!("{}", args[1]);
    } 

    // Handle no arguments
    if args.len() == 1 {
        print!("");
    }

    return Ok(());
}
