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

While I implemented the core logic for the app, I used AI to improve my final implementation, specifically regarding UTF-8 compliance and system testability. I refactored my core loop to handle multi-byte encodings by swapping my direct cast from byte to char (which is limited to ASCII) to the str::from_utf8 method (which handles full UTF-8 sequences) to ensure resilience. Additionally, I consulted AI on best practices for unit testing, which led me to refactor the main function to accept the generic impl Read trait instead of a concrete File type. This allows the unit tests to run strictly against in-memory byte streams rather than relying on external file creation, making the test suite fully self-contained.