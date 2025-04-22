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
    let mut verbose = false;
    let mut files: Vec<String> = Vec::new();

    // Basic argument loop
    for arg in args.iter().skip(1) {
        if arg == "-f" { mode = OverwriteMode::Force; }
        else if arg == "-i" { mode = OverwriteMode::Interactive; }
        else if arg == "-n" { mode = OverwriteMode::NoClobber; }
        else if arg == "-v" { verbose = true; }
        else if arg.starts_with('-') {
            // Handle potential combined flags simply or error out
            if arg.contains('f') { mode = OverwriteMode::Force; }
            if arg.contains('i') { mode = OverwriteMode::Interactive; }
            if arg.contains('n') { mode = OverwriteMode::NoClobber; }
            if arg.contains('v') { verbose = true; }

            for c in arg.chars().skip(1) {
                if !"finv".contains(c) {
                     eprintln!("rcp: error: invalid option -- '{}'", c);
                     exit(1);
                }
            }
        } else {
            files.push(arg.clone());
        }
    }

    // Validate File Arguments
    if files.len() != 2 {
        eprintln!("rcp: error: missing file operands (expected 2, got {})", files.len());
        exit(1);
    }
    let source_str = &files[0];
    let target_str = &files[1];
    let source_path = Path::new(source_str);
    let target_path = Path::new(target_str);

    // Pre-Copy Checks
    // Check source exists and is a file
    if !source_path.is_file() {
        eprintln!("rcp: error: '{}: No such file or directory or not a regular file", source_str);
        exit(1);
    }
    // self-copy check
    if source_str == target_str {
         eprintln!("rcp: error: '{}' and '{}' are the same file", source_str, target_str);
         exit(1);
    }

    // Handle target existence and overwrite logic
    let mut proceed_with_copy = true;
    if target_path.exists() {
        // Prevent overwriting directories
        if target_path.is_dir() {
             eprintln!("rcp: error: {}: is a directory", target_str);
             exit(1);
        }

        match mode {
            OverwriteMode::NoClobber => {
                if verbose { println!("rcp: {}: not overwritten", target_str); }
                proceed_with_copy = false;
            }
            OverwriteMode::Interactive => {
                eprint!("rcp: overwrite {}? ", target_str);
                io::stderr().flush().expect("Failed to flush stderr"); 
                let mut response = String::new();
                io::stdin().read_line(&mut response).expect("Failed to read from stdin");
                if !response.trim_start().to_lowercase().starts_with('y') {
                    proceed_with_copy = false;
                }
            }
            OverwriteMode::Force => {
                // Remove existing file, panic on error for simplicity
                fs::remove_file(target_path)
                    .map_err(|e| format!("cannot remove {}: {}", target_str, e))
                    .expect("Failed to remove target file in force mode");
            }
            OverwriteMode::Default => {}
        }
    }

    // Perform Copy
    if proceed_with_copy {
        if verbose {
            println!("{} -> {}", source_str, target_str);
        }
        // Perform copy
        match fs::copy(source_path, target_path) {
            Ok(_) => { /* Success */ }
            Err(e) => {
                eprintln!("rcp: error: failed to copy {} to {}: {}", source_str, target_str, e);
                exit(1);
            }
        }
    }
}
