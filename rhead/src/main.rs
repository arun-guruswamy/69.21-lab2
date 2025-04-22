use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::exit;

// Default number of lines to print
const DEFAULT_LINE_COUNT: usize = 10;

fn main() {
    // Argument Parsing
    let args: Vec<String> = env::args().collect();
    let mut line_count = DEFAULT_LINE_COUNT;
    let mut file_paths: Vec<String> = Vec::new();
    let mut print_headers = false;

    let mut i = 1; // Index for iterating through args
    while i < args.len() {
        let arg = &args[i];
        if arg == "-n" {
            // Check for count argument
            if i + 1 < args.len() {
                // Parse count, exit on error
                match args[i + 1].parse::<usize>() {
                    Ok(count) if count > 0 => {
                        line_count = count;
                        i += 1; // Skip the count argument next iteration
                    }
                    _ => {
                        eprintln!("rhead: error: invalid line count: '{}'", args[i + 1]);
                        exit(1);
                    }
                }
            } else {
                eprintln!("rhead: error: option requires an argument -- 'n'");
                exit(1);
            }
        } else if arg.starts_with('-') && arg.len() > 1 {
            // Error on any other options
            eprintln!("rhead: error: invalid option -- '{}'", &arg[1..]);
            exit(1);
        } else {
            // Treat as file path
            file_paths.push(arg.clone());
        }
        i += 1;
    }

    // Determine if headers are needed
    if file_paths.len() > 1 {
        print_headers = true;
    }

    // Process Input
    let process_source = |reader: Box<dyn BufRead>, file_name: Option<&str>, print_header: bool| {
        // Print header if needed
        if print_header && file_name.is_some() {
            println!("==> {} <==", file_name.unwrap());
        }
        // Read and print lines, exit on read error
        for line_result in reader.lines().take(line_count) {
            match line_result {
                Ok(line) => println!("{}", line),
                Err(e) => {
                    // Determine source name for error message
                    let source_name = file_name.unwrap_or("stdin");
                    eprintln!("rhead: error reading from {}: {}", source_name, e);
                    exit(1);
                }
            }
        }
    };

    if file_paths.is_empty() {
        // Read from stdin
        let stdin = io::stdin();
        let reader = Box::new(stdin.lock()); 
        process_source(reader, None, false);
    } else {
        // Process files
        for (index, file_path) in file_paths.iter().enumerate() {
            // Print separator between files if headers are enabled
            if print_headers && index > 0 {
                println!();
            }
            // Open file, exit on error
            match File::open(file_path) {
                Ok(file) => {
                    let reader = Box::new(BufReader::new(file)); 
                    process_source(reader, Some(file_path), print_headers);
                }
                Err(e) => {
                    // Report file opening error and exit
                    eprintln!("rhead: {}: {}", file_path, e);
                    exit(1); 
                }
            }
        }
    }
}
