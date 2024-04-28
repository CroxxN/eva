#![allow(unused_imports)]
use std::{
    fs::File,
    io::{BufReader, Read},
};

// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

mod utils;
use utils::ValidNums;
mod parse;
use parse::{ELFHeader, ELFParser, Pheader};

mod error;
use error::ParseError;

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
    println!();
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
    let mut elf_header = ELFHeader::default();
    _ = elf_header.parse(&file_contents);
    elf_header.display();
}

fn parse_pheader64(slice: &[u8]) -> Pheader {
    let mut cursor = 0;
    let mut pheader = Pheader::default();

    pheader.p_type = u32::from_little_bytes(&slice[cursor..(cursor + 4)]);

    cursor += 4;

    pheader.p_flags = u32::from_little_bytes(&slice[cursor..(cursor + 4)]);

    cursor += 4;

    pheader.p_offset = u64::from_little_bytes(&slice[cursor..(cursor + 8)]);

    cursor += 8;

    pheader.p_vaddr = u64::from_little_bytes(&slice[cursor..(cursor + 8)]);
    cursor += 8;

    pheader.p_paddr = u64::from_little_bytes(&slice[cursor..(cursor + 8)]);
    cursor += 8;

    pheader.p_filesz = u64::from_little_bytes(&slice[cursor..(cursor + 8)]);
    cursor += 8;

    pheader.p_memsz = u64::from_little_bytes(&slice[cursor..(cursor + 8)]);
    cursor += 8;

    pheader.p_align = u64::from_little_bytes(&slice[cursor..(cursor + 8)]);

    pheader
}

fn _disassemble_elf(exec_section: &[u8]) {
    _ = exec_section;
    todo!()
}
