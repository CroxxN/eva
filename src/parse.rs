use crate::error::ParseError;
use crate::utils::{Endian, ValidNums};
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

#[derive(Debug, Default, Eq, PartialEq)]
pub enum Arch {
    B32,
    #[default]
    B64,
}

#[derive(Default, Debug)]
pub struct ELFHeader {
    // 32 or 64 bit
    pub ei_class: Arch,
    // endianess of the elf
    pub ei_data: Endian,
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
        let _elf_header = ELFHeader::default();
        return Err(ParseError::BadMagic);
    }
}

impl ELFHeader {
    // parse the header
    pub fn parse(&mut self, contents: &[u8]) -> Result<usize, ParseError> {
        let step;
        let mut cursor = 4;
        let end;
        if contents[cursor] == 1 {
            self.ei_class = Arch::B32;
            step = 4;
        } else {
            self.ei_class = Arch::B64;
            step = 8;
        }
        cursor += 1;
        if contents[cursor] == 1 {
            self.ei_data = Endian::Little;
            end = Endian::Little;
        } else if contents[cursor] == 2 {
            self.ei_data = Endian::Big;
            end = Endian::Big;
        } else {
            return Err(ParseError::UnsupportedEndianess);
        }
        cursor += 1;
        self.ei_version = contents[cursor];
        cursor += 1;
        self.ei_osabi = contents[cursor];
        cursor += 1;
        self.ei_abiversion = contents[cursor];
        cursor = 16;

        let obj_type = u16::from_bytes(end, &contents[cursor..cursor + 2]);

        self.e_type = obj_type;
        cursor += 2;
        let machine_type = u16::from_bytes(end, &contents[cursor..cursor + 2]);
        self.e_machine = machine_type;
        // cursor += 2;
        let version = contents[20] | contents[23];
        self.e_version = version;
        let mut cursor = 24_usize;
        self.e_entry = u64::from_bytes(end, &contents[cursor..(cursor + step)]);
        cursor += step;
        self.e_phoff = u64::from_bytes(end, &contents[cursor..(cursor + step)]);
        cursor += step;
        self.e_shoff = u64::from_bytes(end, &contents[cursor..(cursor + step)]);
        cursor += step;
        self.e_flags = u32::from_bytes(end, &contents[cursor..(cursor + 4)]);
        cursor += 4;
        self.e_ehsize = u16::from_bytes(end, &contents[cursor..(cursor + 2)]);
        cursor += 2;
        self.e_phentsize = u16::from_bytes(end, &contents[cursor..(cursor + 2)]);
        cursor += 2;
        self.e_phnum = u16::from_bytes(end, &contents[cursor..(cursor + 2)]);
        cursor += 2;
        self.e_shentsize = u16::from_bytes(end, &contents[cursor..(cursor + 2)]);
        cursor += 2;
        self.e_shnum = u16::from_bytes(end, &contents[cursor..(cursor + 2)]);
        cursor += 2;
        self.e_shstrndx = u16::from_bytes(end, &contents[cursor..(cursor + 2)]);
        Ok(cursor + 2)
    }

    pub fn display(&self) {
        // we're sure this file is ELF; if not, the `parse`
        // function already errors out by now
        println!("\x1b[1;32mELF Magic: \x1b[0m\x1b[1;37m0x7F, E, L, F\x1b[0m");

        if self.ei_class == Arch::B32 {
            println!("\x1b[1;32mELF bit: \x1b[37m32-bit\x1b[0m");
        } else {
            println!("\x1b[1;32mELF bit: \x1b[37m64-bit\x1b[0m");
        }
        if self.ei_data == Endian::Big {
            println!("\x1b[1;32mEndianness \x1b[37mbig-endian\x1b[0m");
        } else {
            println!("\x1b[1;32mEndianness \x1b[37mlittle-endian\x1b[0m");
        }
        if self.ei_version == 1 {
            println!("\x1b[1;32mELF version: \x1b[37m1 (original and current version)\x1b[0m");
        }
        print!("\x1b[1;32mTarget operating system is:\x1b[0m ");
        match self.ei_osabi {
            0x00 => println!("\x1b[1mUnix - System V\x1b[0m"),
            0x01 => println!("\x1b[1mHP-UX\x1b[0m"),
            0x02 => println!("\x1b[1mNetBSD\x1b[0m"),
            0x03 => println!("\x1b[1mLinux\x1b[0m"),
            0x04 => println!("\x1b[1mGNU Hurd\x1b[0m"),
            0x06 => println!("\x1b[1mSolaris\x1b[0m"),
            0x07 => println!("\x1b[1mAIX(Monterey)\x1b[0m"),
            0x08 => println!("\x1b[1mIRIX\x1b[0m"),
            0x09 => println!("\x1b[1mFreeBSD\x1b[0m"),
            0x0A => println!("\x1b[1mTru64\x1b[0m"),
            0x0B => println!("\x1b[1mNovell Modesto\x1b[0m"),
            0x0C => println!("\x1b[1mOpenBSD\x1b[0m"),
            0x0D => println!("\x1b[1mOpenVMS\x1b[0m"),
            0x0E => println!("\x1b[1mNonStop Kernel\x1b[0m"),
            0x0F => println!("\x1b[1mAROS\x1b[0m"),
            0x010 => println!("\x1b[1mFenixOS\x1b[0m"),
            0x011 => println!("\x1b[1mNuxi CloudABI\x1b[0m"),
            0x012 => println!("\x1b[1mStratus Technologies OpenVOS\x1b[0m"),
            _ => println!("\x1b[1mUNRECOGNIZED\x1b[0m"),
        }

        println!(
            "\x1b[1;32mABI Version: \x1b[37m{}\x1b[0m",
            self.ei_abiversion
        );
        print!("\x1b[1;32mELF object file type:\x1b[0m ");
        match self.e_type {
            0x01 => println!("\x1b[1mRelocatable\x1b[0m"),
            0x02 => println!("\x1b[1mExecutable\x1b[0m"),
            0x03 => println!("\x1b[1mShared object\x1b[0m"),
            0x04 => println!("\x1b[1mCore file\x1b[0m"),
            0xFE00 | 0xFEFF => println!("\x1b[1mReserved inclusive range(OS-specific)\x1b[0m"),
            0xFF00 | 0xFFFF => {
                println!("\x1b[1mReserved include range (Processor specific)\x1b[0m")
            }
            _ => println!("\x1b[1mUNKNOWN\x1b[0m"),
        }
        print!("\x1b[1;32mELF Instruction set machine:\x1b[0m \x1b[1m");
        match self.e_machine {
            // source: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
            0x01 => println!("AT&T WE 32100"),
            0x02 => println!("SPARC"),
            0x03 => println!("x86"),
            0x04 => println!("Motorola 68000 (M68k)"),
            0x05 => println!("Motorola 88000 (M88k)"),
            0x06 => println!("Intel MCU"),
            0x07 => println!("Intel 80860"),
            0x08 => println!("MIPS"),
            0x09 => println!("IBM System/370"),
            0x0A => println!("MIPS RS3000 Little-endian"),
            0x0F => println!("Hewlett-Packard PA-RISC"),
            0x13 => println!("Intel 80960"),
            0x14 => println!("PowerPC"),
            0x15 => println!("PowerPC(64-bit)"),
            0x16 => println!("S390"),
            0x17 => println!("IBM SPU/SPC"),
            0x24 => println!("NEC V800"),
            0x25 => println!("Fujitsu FR20"),
            0x26 => println!("TRW RH-32"),
            0x27 => println!("Motorola RCE"),
            0x28 => println!("Arm"),
            0x29 => println!("Digital Alpha"),
            0x2A => println!("SuperH"),
            0x2B => println!("SPARC Version 9"),
            0x2C => println!("Siemens TriCore embedded processor"),
            0x2D => println!("Argonaut RISC Core"),
            0x2E => println!("Hitachi H8/300"),
            0x2F => println!("Hitachi H8/300H"),
            0x30 => println!("Hitachi H8S"),
            0x31 => println!("Hitachi H8/500"),
            0x32 => println!("IA-64"),
            0x33 => println!("Stanford MIPS-X"),
            0x34 => println!("Motorola ColdFire"),
            0x35 => println!("Motorola M68HC12"),
            0x36 => println!("Fujitsu MMA Multimedia Accelerator"),
            0x3E => println!("AMD x86-64"),
            0xB7 => println!("Arm64"),
            0xF3 => println!("RISC-V"),
            _ => println!("Unspecified/Unknown"),
        }
        print!("\x1b[0m");
        println!(
            "\x1b[1;32mELF Version:\x1b[0m {}\x1b[1m",
            self.e_version // to work with both big endian and little endian
        );
        println!(
            "\x1b[1;32mStarting Address:\x1b[0m {:#x}\x1b[1m",
            self.e_entry
        );
        println!(
            "\x1b[1;32mProgram header table start:\x1b[0m {} (Absolute)\x1b[1m",
            self.e_phoff
        );
        println!(
            "\x1b[1;32mSection header table start:\x1b[0m {} (Absolute)\x1b[1m",
            self.e_shoff
        );
        println!("\x1b[1;32mFlags:\x1b[0m {:#x}\x1b[1m", self.e_flags);
        println!(
            "\x1b[1;32mELF Header Size:\x1b[0m {} bytes\x1b[1m",
            self.e_ehsize
        );
        println!(
            "\x1b[1;32mProgram Header Size:\x1b[0m {} bytes\x1b[1m",
            self.e_phentsize
        );
        println!(
            "\x1b[1;32mNumber of program headers:\x1b[0m {}\x1b[1m",
            self.e_phnum
        );
        println!(
            "\x1b[1;32mSize of section headers:\x1b[0m {} bytes\x1b[1m",
            self.e_shentsize
        );
        println!(
            "\x1b[1;32mNumber of section headers:\x1b[0m {}\x1b[1m",
            self.e_shnum
        );
        println!(
            "\x1b[1;32mIndex of section headers:\x1b[0m {}\x1b[1m",
            self.e_shstrndx
        );
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
        let mut elf_head = ELFHeader::default();
        self.cursor = elf_head.parse(&self);
        elf_head
    }

    fn parse_program_headers(&self) -> Option<Vec<Pheader>> {
        let container: Vec<Pheader> = Vec::new();
        Some(container)
    }
}
