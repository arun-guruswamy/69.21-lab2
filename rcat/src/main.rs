use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::exit; // For exiting with non-zero status

fn main() {
    // Wrap the main logic in a function that returns Result
    // Handle errors centrally and exit with appropriate status code
    if let Err(e) = run() {
        eprintln!("rcat: error: {}", e);
        exit(1); // Exit with non-zero status on error
    }

    return Ok(());
}

fn run() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect(); // Skip program name
    let mut number_all = false;
    let mut number_non_blank = false;
    let mut squeeze_blank = false;
    let mut file_paths: Vec<String> = Vec::new();

    // Parse arguments: differentiate options from file paths
    for arg in args {
        // Check if it's an option (starts with '-' and is not just "-")
        if arg.starts_with('-') && arg.len() > 1 && arg != "-" {
            // Iterate through characters in the option string (e.g., handle -ns)
            for c in arg.chars().skip(1) {
                match c {
                    'n' => number_all = true,
                    'b' => number_non_blank = true,
                    's' => squeeze_blank = true,
                    _ => {
                        // Invalid option character
                        eprintln!("rcat: invalid option -- '{}'", c);
                        // Provide a hint similar to coreutils
                        eprintln!("Try 'rcat --help' for more information."); // Note: --help is not implemented
                        exit(1); // Exit immediately on invalid option
                    }
                }
            }
        } else {
            // Treat as a file path (including "-")
            file_paths.push(arg);
        }
    }

    // Apply rule: If both -n and -b are specified, -b is ignored (as per lab spec)
    if number_all && number_non_blank {
        number_non_blank = false;
    }

    // Determine if we need to read from stdin initially
    let read_stdin_initially = file_paths.is_empty();

    // Process inputs: stdin or files
    if read_stdin_initially {
        let stdin = io::stdin();
        let reader = stdin.lock(); // Lock stdin for buffered reading
        // Pass "stdin" as the source name for potential error messages
        process_lines(reader, number_all, number_non_blank, squeeze_blank, "stdin")?;
    } else {
        // Iterate through the collected file paths
        for file_path in file_paths {
            if file_path == "-" {
                // Handle "-" argument: read from stdin
                let stdin = io::stdin();
                let reader = stdin.lock();
                process_lines(reader, number_all, number_non_blank, squeeze_blank, "stdin")?;
            } else {
                // Try to open the specified file
                match File::open(&file_path) {
                    Ok(file) => {
                        // File opened successfully, create a buffered reader
                        let reader = BufReader::new(file);
                        // Process the file lines, passing the file_path for error context
                        process_lines(reader, number_all, number_non_blank, squeeze_blank, &file_path)?;
                    }
                    Err(e) => {
                        // File opening failed, report error to stderr
                        eprintln!("rcat: {}: {}", file_path, e);
                        // Return the error to be caught by main, causing exit(1)
                        return Err(e);
                    }
                }
            }
        }
    }

    Ok(()) // Indicate success
}

// Process lines from a given BufRead source
// Changed file_path parameter to &str for efficiency and clarity
fn process_lines<B: BufRead>(
    reader: B,
    number_all: bool,
    number_non_blank: bool,
    squeeze_blank: bool,
    source_name: &str, // Name of the source ("stdin" or filename) for error reporting
) -> io::Result<()> {
    let mut line_number = 1; // Line counter for -n and -b
    let mut previous_line_was_blank = false; // State for -s option

    for line_result in reader.lines() {
        // Handle potential I/O errors during line reading
        let line = match line_result {
            Ok(l) => l,
            Err(e) => {
                // Report error reading from the source
                eprintln!("rcat: error reading from {}: {}", source_name, e);
                return Err(e); // Propagate the error
            }
        };

        let is_blank = line.trim().is_empty(); // Check if the line is blank (ignoring whitespace)

        // Implement -s: Squeeze multiple adjacent empty lines
        if squeeze_blank && is_blank && previous_line_was_blank {
            continue; // Skip printing this redundant blank line
        }

        // Implement -n: Number all output lines
        if number_all {
            // Format: right-aligned number (6 spaces), tab, line content
            println!("{:6}\t{}", line_number, line);
            line_number += 1;
        }
        // Implement -b: Number non-blank output lines (ignored if -n is also set)
        else if number_non_blank {
            if is_blank {
                println!("{}", line); // Print blank lines without numbers
            } else {
                println!("{:6}\t{}", line_number, line);
                line_number += 1; // Increment number only for non-blank lines
            }
        }
        // No numbering options
        else {
            println!("{}", line);
        }

        // Update state for the next line's -s check
        previous_line_was_blank = is_blank;
    }

    Ok(()) // Indicate successful processing of this source
}
