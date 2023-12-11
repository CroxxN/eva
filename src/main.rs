use std::{
    fs::File,
    io::{BufReader, Read},
};

#[derive(Default)]
struct ELFHeader {
    // 32 or 64 bit
    ei_class: u8,
    // endianess of the elf
    ei_data: u8,
}

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
    let file_size = if let Ok(s) = file_buf.read_to_end(&mut file_contents) {
        s
    } else {
        eprintln!("\x1b[1;31mError: Cann't read the file\x1b[0m");
        return;
    };

    if file_size < 4 {
        println!("\x1b[1;31mThe file is NOT in ELF format\x1b[0m");
        return;
    }

    // 0x7f, E, L, F
    if file_contents[0] == 0x7f
        && file_contents[1] == 0x45
        && file_contents[2] == 0x4C
        && file_contents[3] == 0x46
    {
        println!("\x1b[1;32mThe file is in ELF format\x1b[0m");
    } else {
        println!("\x1b[1;31mThe file is NOT in ELF format\x1b[0m");
        return;
    }
    let mut elf_hdr = ELFHeader::default();
    if file_contents[4] == 1 {
        elf_hdr.ei_class = 1;
        println!("\x1b[1;32mThe ELF is \x1b[37m32-bit\x1b[0m");
    } else {
        elf_hdr.ei_class = 2;
        println!("\x1b[1;32mThe ELF is \x1b[37m64-bit\x1b[0m");
    }
    if file_contents[5] == 1 {
        elf_hdr.ei_data = 1;
        println!("\x1b[1;32mThe ELF is \x1b[37mlittle-endian\x1b[0m");
    } else if file_contents[5] == 2 {
        elf_hdr.ei_data = 2;
        println!("\x1b[1;32mThe ELF is \x1b[37mbig-endian\x1b[0m");
    } else {
        println!("\x1b[1;31mThe file is NOT in ELF format or is corrupted\x1b[0m");
        return;
    }
}
