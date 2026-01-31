use std::io::Read;




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

    let mut reader = std::io::BufReader::new(file);

    for byte in reader.bytes() {
        match byte {
            Ok(byte) => dbg!(byte),
            Err(e) => {
                eprintln!("Failed to read byte: {e}");
                return;
            },
        };
    }


}
