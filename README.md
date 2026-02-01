# Northwood Take Home Project

## Build & Run Instructions

Ensure you have `cargo` and the latest Rust compiler.

You can build or test the program by running:

```bash
cargo run --release -- <filename>
# or
cargo test
```

## Architecture & Design

The program iterates through the file byte by byte, using a streaming approach with `BufReader`, instead of loading the entire file into memory. This keeps memory usage very low, as the memory usage will be proportional to the amount of unique words, and constant relative to file size.

While iterating through the file, the program accumulates bytes to form a valid UTF-8 character. This allows for processing multi-byte characters while keeping the streaming architecture.

For the data structure, I chose to use a `BTreeMap` to store the words and their indices, as the keys are inserted in dictionary order, and the time complexity stays at `O(n * log(n))`.


## AI Use

While I implemented the core logic for the app, I used AI to improve my final implementation, specifically with potential UTF-8 encoding issues. I ended up refactoring my core loop to be able to handle all UTF-8 encodings, not just ASCII, by swapping my direct cast from byte to `char` (which can only handle single byte text like ASCII) to the `str::from_utf8` method instead (which can handle up to 4 byte UTF-8 characters) to ensure the program is resilient.