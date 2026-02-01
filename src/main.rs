use std::{collections::BTreeMap, io::Read};


fn main() {
    // Read file name from program args
    let args = std::env::args().collect::<Vec<String>>();
    let file_name = match args.get(1) {
        Some(file_name) => file_name,
        None => {
            eprintln!("Expected file name in program arguments");
            return;
        },
    };

    if args.len() > 2 {
        eprintln!("Program only takes one argument: File name");
        return;
    }

    let file = match std::fs::File::open(file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Unable to read file {}: {}", file_name, e);
            return;
        },
    };

    // Using btree to automatically sort lexographiclly as we insert new keys
    let mut map: BTreeMap<String, Vec<usize>>  = BTreeMap::new();
    let mut current_char = 0;

    // Using buf reader to limit memory usage & use less sys calls
    let reader = std::io::BufReader::new(file);
    let mut current_word = String::new();

    
    for byte in reader.bytes() {
        match byte {
            Ok(byte) => {
                match char::from_u32(byte as u32) {
                    Some(char) => {
                        if char.is_whitespace() {
                            // If btree map contains key, append index to indices list
                            if map.contains_key(&current_word) {
                                let mapped_word = map.get_mut(&current_word);
                                match mapped_word {
                                    Some(mapped_word) => {
                                        mapped_word.push(current_char - current_word.len());
                                        current_word.clear();
                                    },
                                    None => unreachable!(),
                                }
                            } else if !current_word.is_empty() {
                                map.insert(current_word.clone(), vec![current_char - current_word.len()]);
                                current_word.clear();
                            }
                        }
                        // Else push the char onto the current word buffer
                        else {
                            current_word.push(char)
                        }
                    },
                    None => {
                        eprintln!("Failed to convert byte {} into char.", byte);
                        return;
                    },
                }
            },
            Err(e) => {
                eprintln!("Failed to read byte: {e}");
                return;
            },
        };

        // Increment current byte
        current_char += 1;
    }

    // Handle if last word is not white space
    if !current_word.is_empty() {
        if map.contains_key(&current_word) {
            let mapped_word = map.get_mut(&current_word);
            match mapped_word {
                Some(mapped_word) => mapped_word.push(current_char - current_word.len()),
                None => unreachable!(),
            }
        } else if !current_word.is_empty() {
            map.insert(current_word.clone(), vec![current_char - current_word.len()]);
            current_word.clear();
        }
    }

    // Output
    for (key, indices) in map.iter() {
        print!("{key} ");
        for indice in indices {
            print!("{indice} ");
        }
        println!();
    }

}
