use std::{collections::BTreeMap, io::Read};


fn main() {
    println!("Hello, world!");

    let args = std::env::args().collect::<Vec<String>>();
    let file_name = match args.get(1) {
        Some(file_name) => file_name,
        None => {
            eprintln!("Expected file name in program arguments");
            return;
        },
    };

    let file = match std::fs::File::open(file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Unable to read file {}: {}", file_name, e);
            return;
        },
    };

    let mut map: BTreeMap<String, Vec<usize>>  = BTreeMap::new();
    let mut index = 0;

    let reader = std::io::BufReader::new(file);
    let mut current_word = String::new();

    for byte in reader.bytes() {
        match byte {
            Ok(byte) => {
                // Read byte as char
                match char::from_u32(byte as u32) {
                    Some(char) => match char.is_whitespace() {
                        // If current byte is whitespace, add to the map.
                        true => {
                            if map.contains_key(&current_word) {
                                let mapped_word = map.get_mut(&current_word);
                                match mapped_word {
                                    Some(mapped_word) => mapped_word.push(index - current_word.len()),
                                    None => unreachable!(),
                                }
                            } else {
                                map.insert(current_word.clone(), vec![index - current_word.len()]);
                                current_word.clear();
                            }
                            
                        },
                        // Else push the char onto the current word buffer
                        false => current_word.push(char),
                    },
                    None => {
                        eprintln!("Failed to convert byte {} into char.", byte);
                    },
                }
            },
            Err(e) => {
                eprintln!("Failed to read byte: {e}");
                return;
            },
        };

        index += 1;
    }

    dbg!(map);

}
