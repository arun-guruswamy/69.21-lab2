# 69.21 Lab 2: Rust Coreutils Implementation

This project contains Rust implementations of several common Unix-like core utilities for Lab 2.

## Implemented Utilities

The following utilities have been implemented with a subset of their original options:

*   **`rcat`**: Concatenate and print files.
    *   Located in: `rcat/`
    *   Options: `-b` (number non-blank lines), `-n` (number all lines), `-s` (squeeze blank lines).
*   **`rcp`**: Copy files.
    *   Located in: `rcp/`
    *   Options: `-f` (force overwrite), `-i` (interactive overwrite), `-n` (no overwrite), `-v` (verbose).
*   **`rhead`**: Display the first lines of a file.
    *   Located in: `rhead/`
    *   Options: `-n count` (specify number of lines, default 10).
*   **`rmv`**: Move (rename) files.
    *   Located in: `rmv/`
    *   Options: `-f` (force overwrite), `-i` (interactive overwrite), `-n` (no overwrite).

## Building

Each utility is a separate Cargo project. To build a specific utility (e.g., `rcat`), navigate to the main lab directory (`69.21-lab2`) in your terminal and use the `--manifest-path` flag with `cargo build`:

```bash
# Example: Build rcat
cargo build --manifest-path rcat/Cargo.toml --bin rcat

# Example: Build rcp
cargo build --manifest-path rcp/Cargo.toml --bin rcp

# Example: Build rhead
cargo build --manifest-path rhead/Cargo.toml --bin rhead

# Example: Build rmv
cargo build --manifest-path rmv/Cargo.toml --bin rmv
```

The compiled executables will be placed in the `target/debug/` subdirectory within each utility's folder (e.g., `rcat/target/debug/rcat.exe`).

## Running

You can run the utilities using `cargo run` or by executing the compiled binary directly.

### Using `cargo run`

Use `cargo run` with the `--manifest-path` flag. Remember to use `--` to separate arguments intended for the utility from arguments for Cargo.

```bash
# Example: Run rcat on file1.txt with line numbers
cargo run --manifest-path rcat/Cargo.toml --bin rcat -- -n file1.txt

# Example: Run rcp interactively
cargo run --manifest-path rcp/Cargo.toml --bin rcp -- -i source.txt target.txt

# Example: Run rhead for 5 lines of file.log
cargo run --manifest-path rhead/Cargo.toml --bin rhead -- -n 5 file.log

# Example: Run rmv forcing overwrite
cargo run --manifest-path rmv/Cargo.toml --bin rmv -- -f oldname.txt newname.txt
```

**Important:** If file paths contain spaces, enclose them in double quotes (`"`):

```bash
cargo run --manifest-path rcp/Cargo.toml --bin rcp -- "source file with spaces.txt" "target file.txt"
```

### Running the Executable Directly

After building, you can run the executable directly from its location in the `target/debug` directory.

```bash
# Example: Run rcat directly (assuming you are in the 69.21-lab2 directory)
./rcat/target/debug/rcat -n file1.txt

# Example: Run rcp directly with quoted paths
./rcp/target/debug/rcp -v "source file with spaces.txt" "target file.txt"
```

## Assumptions and Simplifications

*   **Error Handling:** The implementations use basic error handling, primarily printing messages to stderr and exiting with status code 1 on failure.
*   **Option Parsing:** Argument parsing is done manually or via simple checks. It might not handle all edge cases or combinations of options as robustly as the original coreutils. The "last option wins" rule for conflicting flags (`-f`/`-i`/`-n`) is implemented based on the order they appear in the arguments. Combined flags (e.g., `-fv`) are handled by checking for individual characters within the flag, which might not perfectly match standard behavior or precedence rules.
*   **`rcp` / `rmv` Overwriting:** They rely on `fs::copy` and `fs::rename`'s default behavior for overwriting, which generally works for files but might fail for directories or due to permissions.
*   **Self-Copy/Move:** Checks for copying or moving a file onto itself are basic and might not cover all edge cases.
*   **Directory Handling:** These utilities are primarily designed to work with files. `rcp` and `rmv` specifically check if the target is a directory and error out if attempting to overwrite one. Recursive operations are not supported.
