use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::exit;

// Enum for overwrite modes
#[derive(PartialEq, Clone, Copy)]
enum OverwriteMode { Force, Interactive, NoClobber, Default }

fn main() {
    // Argument Parsing
    let args: Vec<String> = env::args().collect();
    let mut mode = OverwriteMode::Default;
    let mut files: Vec<String> = Vec::new();

    // Basic argument loop
    for arg in args.iter().skip(1) {
        if arg == "-f" { mode = OverwriteMode::Force; }
        else if arg == "-i" { mode = OverwriteMode::Interactive; }
        else if arg == "-n" { mode = OverwriteMode::NoClobber; }
        else if arg.starts_with('-') {
             // Handle potential combined flags simply or error out
            if arg.contains('f') { mode = OverwriteMode::Force; }
            if arg.contains('i') { mode = OverwriteMode::Interactive; }
            if arg.contains('n') { mode = OverwriteMode::NoClobber; }
             // Error on any other unrecognized flag character
            for c in arg.chars().skip(1) {
                if !"fin".contains(c) {
                     eprintln!("rmv: error: invalid option -- '{}'", c);
                     exit(1);
                }
            }
        } else {
            files.push(arg.clone());
        }
    }

    // Validate File Arguments
    if files.len() != 2 {
        eprintln!("rmv: error: missing file operands (expected 2, got {})", files.len());
        exit(1);
    }
    let source_str = &files[0];
    let target_str = &files[1];
    let source_path = Path::new(source_str);
    let target_path = Path::new(target_str);

    // Pre-Move Checks
    // Check source existence
    if fs::metadata(source_path).is_err() {
        eprintln!("rmv: error: cannot stat '{}': No such file or directory", source_str);
        exit(1);
    }
    // self-move check
    if source_str == target_str {
         eprintln!("rmv: error: '{}' and '{}' are the same file", source_str, target_str);
         exit(1);
    }

    // Handle Target Existence & Overwrite Logic
    let mut proceed_with_move = true;
    if target_path.exists() {
        match mode {
            OverwriteMode::NoClobber => {
                // do not overwrite, just exit successfully
                proceed_with_move = false;
            }
            OverwriteMode::Interactive => {
                // prompt user
                eprint!("rmv: overwrite {}? ", target_str);
                io::stderr().flush().expect("Failed to flush stderr");
                let mut response = String::new();
                io::stdin().read_line(&mut response).expect("Failed to read from stdin");
                if !response.trim_start().to_lowercase().starts_with('y') {
                    proceed_with_move = false; // Don't move if not 'y' or 'Y'
                }
                // If 'y' or 'Y', proceed_with_move remains true
            }
            OverwriteMode::Force => {}
            OverwriteMode::Default => {}
        }
    }

    // Perform Move
    if proceed_with_move {
        // Attempt the rename operation
        match fs::rename(source_path, target_path) {
            Ok(_) => { /* Success */ }
            Err(e) => {
                // Report error during rename
                eprintln!("rmv: error: failed to move '{}' to '{}': {}", source_str, target_str, e);
                exit(1);
            }
        }
    }
}
