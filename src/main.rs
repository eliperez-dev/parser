use std::{collections::BTreeMap, io::Read};


fn parse_file<R: Read>(source: R) -> Result<BTreeMap<String, Vec<usize>>, String> {
    // Using btree to automatically sort lexographiclly as we insert new keys
    let mut map: BTreeMap<String, Vec<usize>>  = BTreeMap::new();
    let mut current_char = 0;

    // Using buf reader to limit memory usage & use less sys calls
    let reader = std::io::BufReader::new(source);
    let mut current_word = String::new();

    let mut char_bytes = Vec::new(); 
    let mut word_start_index: Option<usize> = None;

    
    for byte in reader.bytes() {
        let b = match byte {
            Ok(b) => b,
            Err(e) => return Err(format!("Failed to read: {}", e))
        };

        // Accumulate bytes into temporary buffer
        char_bytes.push(b);

        // Try to Convert accumulated bytes to a valid string
        if let Ok(s) = std::str::from_utf8(&char_bytes) {
            let char = s.chars().next().unwrap();
            
            // Calculate where this character started 
            let char_start_pos = current_char - (char_bytes.len() - 1);

            if char.is_whitespace() {
                if !current_word.is_empty() {
                    if let Some(start) = word_start_index {
                        map.entry(current_word.clone())
                           .or_default()
                           .push(start);
                    }
                    current_word.clear();
                    word_start_index = None;
                }
            } else {
                if word_start_index.is_none() {
                    word_start_index = Some(char_start_pos);
                }
                current_word.push(char);
            }
            
            // Clear the byte buffer because we successfully used the character
            char_bytes.clear();
        } 
        else if char_bytes.len() > 4 {
             char_bytes.clear();
        }

        current_char += 1;
    }

    // Handle if last word is not white space
    if !current_word.is_empty() {
        if let Some(start) = word_start_index {
            map.entry(current_word).or_default().push(start);
        }
    }   
    Ok(map)

}

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

    let map = match parse_file(file) {
        Ok(map) => map,
        Err(e) => {
            eprintln!("{}", e);
            return;
        },
    };

    // Output
    for (key, indices) in map.iter() {
        print!("{key} ");
        for index in indices {
            print!("{index} ");
        }
        println!();
    }
}


#[cfg(test)]
mod test {
    use crate::parse_file;

    #[test]
    fn test_file() {
        let map = parse_file(std::fs::File::open("test.txt").unwrap()).unwrap();
        assert_eq!(map.get("aaa"), Some(&vec![4, 12]));
        assert_eq!(map.get("bbb"), Some(&vec![8]));
        assert_eq!(map.get("zzz"), Some(&vec![0]));
    }

    #[test]
    fn test_utf8() {
        let data = "héllo world";
        let map = parse_file(data.as_bytes()).unwrap();
        assert_eq!(map.get("héllo"), Some(&vec![0]));
        assert_eq!(map.get("world"), Some(&vec![7]));
    }
}