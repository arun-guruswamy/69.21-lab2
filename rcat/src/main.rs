use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::exit;

fn main() {
    // Argument Parsing
    let args: Vec<String> = env::args().collect();
    let mut number_all = false;
    let mut number_non_blank = false;
    let mut squeeze_blank = false;
    let mut file_paths: Vec<String> = Vec::new();

    // Basic argument loop, skipping program name
    for arg in args.iter().skip(1) {
        if arg.starts_with('-') && arg.len() > 1 && arg != "-" {
            // Handle combined flags or error out
            for c in arg.chars().skip(1) {
                match c {
                    'n' => number_all = true,
                    'b' => number_non_blank = true,
                    's' => squeeze_blank = true,
                    _ => {
                        eprintln!("rcat: error: invalid option -- '{}'", c);
                        exit(1);
                    }
                }
            }
        } else {
            // Treat as file path 
            file_paths.push(arg.clone());
        }
    }

    // Apply rule: -b is ignored if -n is present
    if number_all {
        number_non_blank = false;
    }

    // These need to be mutable and passed across calls to process_source
    let mut line_number = 1;
    let mut previous_line_was_blank = false;

    // --- Input Processing ---
    let process_source = |reader: Box<dyn BufRead>, source_name: &str, number_all: bool, number_non_blank: bool, squeeze_blank: bool, line_num: &mut usize, prev_blank: &mut bool| {
        for line_result in reader.lines() {
            let line = match line_result {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("rcat: error reading from {}: {}", source_name, e);
                    exit(1);
                }
            };

            let is_blank = line.trim().is_empty();

            // Handle -s
            if squeeze_blank && is_blank && *prev_blank {
                continue; // Skip this redundant blank line
            }

            // Handle -n: Number all lines
            if number_all {
                println!("{:6}\t{}", line_num, line);
                *line_num += 1;
            }
            // Handle -b: Number non-blank lines
            else if number_non_blank {
                if is_blank {
                    println!("{}", line); // Print blank lines without numbers
                } else {
                    println!("{:6}\t{}", line_num, line);
                    *line_num += 1; // Increment only for non-blank lines
                }
            }
            // No numbering
            else {
                println!("{}", line);
            }

            // Update state for next lines -s check
            *prev_blank = is_blank;
        }
    };


    // Process stdin or files
    if file_paths.is_empty() {
        let stdin = io::stdin();
        let reader = Box::new(stdin.lock()); // Box to create trait object
        process_source(reader, "stdin", number_all, number_non_blank, squeeze_blank, &mut line_number, &mut previous_line_was_blank);
    } else {
        for file_path in file_paths {
            if file_path == "-" {
                let stdin = io::stdin();
                let reader = Box::new(stdin.lock());
                process_source(reader, "stdin", number_all, number_non_blank, squeeze_blank, &mut line_number, &mut previous_line_was_blank);
            } else {
                match File::open(&file_path) {
                    Ok(file) => {
                        let reader = Box::new(BufReader::new(file)); // Box to create trait object
                        process_source(reader, &file_path, number_all, number_non_blank, squeeze_blank, &mut line_number, &mut previous_line_was_blank);
                    }
                    Err(e) => {
                        eprintln!("rcat: {}: {}", file_path, e);
                        exit(1); // Exit if a file cannot be opened
                    }
                }
            }
        }
    }
}
