use crate::error::ParseError;
use crate::utils::ValidNums;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

#[derive(Default)]
pub struct ELFHeader {
    // 32 or 64 bit
    pub ei_class: u8,
    // endianess of the elf
    pub ei_data: u8,
    // ELF version
    pub ei_version: u8,
    // target os-abi
    pub ei_osabi: u8,
    // further specification about the ABI version
    pub ei_abiversion: u8,
    // type of obj/elf file.
    pub e_type: u16,
    // constant: 1
    pub e_version: u8,
    // specific target instruction set arch
    pub e_machine: u16,
    // mem address of the entry point from where the program starts executing
    pub e_entry: u64,
    // pointing to the start of the program header
    pub e_phoff: u64,
    // points to the start of the section header. Absolute offset into the file
    pub e_shoff: u64,
    // platform dependent flags
    pub e_flags: u32,
    // size of elf header
    pub e_ehsize: u16,
    // size of program header
    pub e_phentsize: u16,
    // number of program headers
    pub e_phnum: u16,
    // size of section headers
    pub e_shentsize: u16,
    // number of section headers
    pub e_shnum: u16,
    // index of the section header table entry that contains the section names
    pub e_shstrndx: u16,
}

#[derive(Default)]
// aligned to u64 to accomodate both ELF32 and ELF64 program
// headers
pub struct Pheader {
    pub p_type: u32,
    // different location for 32-bit and 64-bit ELF
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

pub struct ELFParser<'a> {
    cursor: u32,
    file_contents: &'a [u8],
    file_size: u32,
    pub elf_header: ELFHeader,
    pub program_headers: Option<Vec<Pheader>>,
    //pub section_headers: Option<Vec<SectionHeader>>,
}

impl<'a> ELFParser<'a> {
    pub fn new(path: PathBuf) -> Result<Self, ParseError> {
        let file = File::open(path)?;
        let mut file_buf = BufReader::new(file);
        let mut file_contents = Vec::new();
        let file_size = file_buf.read_to_end(&mut file_contents)?;

        if file_size < 4 {
            return Err(ParseError::BadMagic);
        }

        if file_contents[0] == 0x7f
            && file_contents[1] == 0x45
            && file_contents[2] == 0x4C
            && file_contents[3] == 0x46
        {
            println!("\x1b[1;32mELF Magic: \x1b[0m\x1b[1;37m0x7F, E, L, F\x1b[0m");
        } else {
            return Err(ParseError::BadMagic);
        }
        let elf_header = ELFHeader::default();
        return Err(ParseError::BadMagic);
    }
}

impl ELFHeader {
    // parse the header
    fn parse(header: &mut Self, contents: &[u8]) -> Result<usize, ParseError> {
        let mut step = 0;
        let mut cursor = 4;
        if contents[cursor] == 1 {
            header.ei_class = 1;
            step = 4;
        } else {
            header.ei_class = 2;
            step = 8;
        }
        cursor += 1;
        if contents[cursor] == 1 {
            header.ei_data = 1;
        } else if contents[cursor] == 2 {
            header.ei_data = 2;
        } else {
            return Err(ParseError::UnsupportedEndianess);
        }
        cursor += 1;
        header.ei_version = contents[cursor];
        cursor += 1;
        header.ei_osabi = contents[cursor];
        cursor += 1;
        header.ei_abiversion = contents[cursor];
        cursor = 16;
        let obj_type = if header.ei_data == 2 {
            u16::from_big_bytes(&contents[cursor..cursor + 2])
        } else {
            u16::from_little_bytes(&contents[cursor..cursor + 2])
        };
        header.e_type = obj_type;
        cursor += 2;
        let machine_type = if header.ei_data == 2 {
            u16::from_big_bytes(&contents[cursor..cursor + 2])
        } else {
            u16::from_little_bytes(&contents[cursor..cursor + 2])
        };
        header.e_machine = machine_type;
        cursor += 2;
        let version = contents[20] | contents[23];
        let mut cursor = 24_usize;
        header.e_entry = if header.ei_data == 2 {
            u64::from_big_bytes(&contents[cursor..(cursor + step)])
        } else {
            u64::from_little_bytes(&contents[cursor..(cursor + step)])
        };
        cursor += step;
        header.e_phoff = if header.ei_data == 2 {
            u64::from_big_bytes(&contents[cursor..(cursor + step)])
        } else {
            u64::from_little_bytes(&contents[cursor..(cursor + step)])
        };
        cursor += step;
        header.e_shoff = if header.ei_data == 2 {
            u64::from_big_bytes(&contents[cursor..(cursor + step)])
        } else {
            u64::from_little_bytes(&contents[cursor..(cursor + step)])
        };
        cursor += step;
        header.e_flags = if header.ei_data == 2 {
            u32::from_big_bytes(&contents[cursor..(cursor + 4)])
        } else {
            u32::from_little_bytes(&contents[cursor..(cursor + 4)])
        };
        cursor += 4;
        header.e_ehsize = if header.ei_data == 2 {
            u16::from_big_bytes(&contents[cursor..(cursor + 2)])
        } else {
            u16::from_little_bytes(&contents[cursor..(cursor + 2)])
        };
        cursor += 2;
        header.e_phentsize = if header.ei_data == 2 {
            u16::from_big_bytes(&contents[cursor..(cursor + 2)])
        } else {
            u16::from_little_bytes(&contents[cursor..(cursor + 2)])
        };
        cursor += 2;
        header.e_phnum = if header.ei_data == 2 {
            u16::from_big_bytes(&contents[cursor..(cursor + 2)])
        } else {
            u16::from_little_bytes(&contents[cursor..(cursor + 2)])
        };
        cursor += 2;
        header.e_shentsize = if header.ei_data == 2 {
            u16::from_big_bytes(&contents[cursor..(cursor + 2)])
        } else {
            u16::from_little_bytes(&contents[cursor..(cursor + 2)])
        };
        cursor += 2;
        header.e_shnum = if header.ei_data == 2 {
            u16::from_big_bytes(&contents[cursor..(cursor + 2)])
        } else {
            u16::from_little_bytes(&contents[cursor..(cursor + 2)])
        };
        cursor += 2;
        header.e_shstrndx = if header.ei_data == 2 {
            u16::from_big_bytes(&contents[cursor..(cursor + 2)])
        } else {
            u16::from_little_bytes(&contents[cursor..(cursor + 2)])
        };
        Ok(cursor + 2)
    }
}

pub trait ELFParserExt {
    fn parse_elf_header(&self) -> ELFHeader;
    fn parse_program_headers(&self) -> Option<Vec<Pheader>>;
}

// We provide separate implementation for file and &[u8] types
// to make it easier for us to accpet ELF file from the stdin.

impl ELFParserExt for [u8] {
    fn parse_elf_header(&self) -> ELFHeader {
        todo!()
    }

    fn parse_program_headers(&self) -> Option<Vec<Pheader>> {
        todo!()
    }
}
