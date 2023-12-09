use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("\x1b[1;31mError: Missing file name.\x1b[0m");
        return;
    }

    let file = match File::open(&args[1]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let mut file_buf = BufReader::new(file);
    let mut file_contents = Vec::new();
    let _file_size = if let Ok(s) = file_buf.read_to_end(&mut file_contents) {
        s
    } else {
        eprintln!("\x1b[1;31mError: Cann't read the file\x1b[0m");
        return;
    };
    // 0x7f, E, L, F
    if file_contents[0] == 0x7f
        && file_contents[1] == 0x45
        && file_contents[2] == 0x4C
        && file_contents[3] == 0x46
    {
        println!("\x1b[1;32mThe file is in ELF format\x1b[0m");
    } else {
        println!("\x1b[1;31mThe file is NOT in ELF format\x1b[0m");
    }
}
