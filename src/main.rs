#![allow(unused_imports)]
use std::{
    fs::File,
    io::{BufReader, Read},
};

mod utils;
use utils::ValidNums;

#[derive(Default)]
struct ELFHeader {
    // 32 or 64 bit
    ei_class: u8,
    // endianess of the elf
    ei_data: u8,
    // ELF version
    ei_version: u8,
    // target os-abi
    ei_osabi: u8,
    // further specification about the ABI version
    ei_abiversion: u8,
    // type of obj/elf file.
    ei_type: u16,
    // specific target instruction set arch
    ei_machine: u16,
    // mem address of the entry point from where the program starts executing
    ei_entry: u64,
    // pointing to the start of the program header
    ei_phoff: u64,
    // points to the start of the section header. Absolute offset into the file
    ei_shoff: u64,
    // platform dependent flags
    ei_flags: u32,
    // size of elf header
    ei_ehsize: u16,
    // size of program header
    ei_phentsize: u16,
    // number of program headers
    ei_phnum: u16,
    // size of section headers
    ei_shentsize: u16,
    // number of section headers
    ei_shnum: u16,
    // index of the section header table entry that contains the section names
    ei_shstrndx: u16,
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

    // 0x7f, E, L, F
    if file_contents[0] == 0x7f
        && file_contents[1] == 0x45
        && file_contents[2] == 0x4C
        && file_contents[3] == 0x46
    {
        println!("\x1b[1;32mELF Magic: \x1b[0m\x1b[1;37m0x7F, E, L, F\x1b[0m");
    } else {
        println!("\x1b[1;31mThe file is NOT in ELF format\x1b[0m");
        return;
    }
    let mut elf_hdr = ELFHeader::default();
    let step: usize;
    if file_contents[4] == 1 {
        elf_hdr.ei_class = 1;
        step = 4;
        println!("\x1b[1;32mELF bit: \x1b[37m32-bit\x1b[0m");
    } else {
        elf_hdr.ei_class = 2;
        step = 8;
        println!("\x1b[1;32mELF bit: \x1b[37m64-bit\x1b[0m");
    }
    if file_contents[5] == 1 {
        elf_hdr.ei_data = 1;
        println!("\x1b[1;32mEndianness \x1b[37mlittle-endian\x1b[0m");
    } else if file_contents[5] == 2 {
        elf_hdr.ei_data = 2;
        println!("\x1b[1;32mEndianness \x1b[37mbig-endian\x1b[0m");
    } else {
        println!("\x1b[1;31mThe file is NOT in ELF format or is corrupted\x1b[0m");
        return;
    }
    elf_hdr.ei_version = file_contents[6];
    if file_contents[6] == 1 {
        println!("\x1b[1;32mELF version: \x1b[37m1 (original and current version)\x1b[0m");
    }

    print!("\x1b[1;32mTarget operating system is:\x1b[0m ");
    elf_hdr.ei_osabi = file_contents[7];
    match file_contents[7] {
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

    println!("\x1b[1;32mABI Version: \x1b[37m{}\x1b[0m", file_contents[8]);
    elf_hdr.ei_abiversion = file_contents[8];
    let obj_type = if elf_hdr.ei_data == 2 {
        u16::from_big_bytes(&file_contents[16..18])
    } else {
        u16::from_little_bytes(&file_contents[16..18])
    };
    elf_hdr.ei_type = obj_type;
    print!("\x1b[1;32mELF object file type:\x1b[0m ");
    match obj_type {
        0x01 => println!("\x1b[1mRelocatable\x1b[0m"),
        0x02 => println!("\x1b[1mExecutable\x1b[0m"),
        0x03 => println!("\x1b[1mShared object\x1b[0m"),
        0x04 => println!("\x1b[1mCore file\x1b[0m"),
        0xFE00 | 0xFEFF => println!("\x1b[1mReserved inclusive range(OS-specific)\x1b[0m"),
        0xFF00 | 0xFFFF => println!("\x1b[1mReserved include range (Processor specific)\x1b[0m"),
        _ => println!("\x1b[1mUNKNOWN\x1b[0m"),
    }
    let machine_type = if elf_hdr.ei_data == 2 {
        u16::from_big_bytes(&file_contents[18..20])
    } else {
        u16::from_little_bytes(&file_contents[18..20])
    };
    elf_hdr.ei_machine = machine_type;

    print!("\x1b[1;32mELF Instruction set machine:\x1b[0m \x1b[1m");
    match machine_type {
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
        file_contents[20] | file_contents[23] // to work with both big endian and little endian
    );

    let mut cursor = 24_usize;
    // elf_hdr.ei_entry = if elf_hdr.ei_class == 1 {
    //     cursor += 4;
    //     if elf_hdr.ei_data == 2 {
    //         u32::from_big_bytes(&file_contents[(cursor - 4)..cursor]) as u64
    //     } else {
    //         u32::from_little_bytes(&file_contents[(cursor - 4)..cursor]) as u64
    //     }
    // } else {
    //     cursor += 8;
    //     if elf_hdr.ei_data == 2 {
    //         u64::from_big_bytes(&file_contents[(cursor - 8)..cursor])
    //     } else {
    //         u64::from_little_bytes(&file_contents[(cursor - 8)..cursor])
    //     }
    // };
    elf_hdr.ei_entry = if elf_hdr.ei_data == 2 {
        u64::from_big_bytes(&file_contents[cursor..(cursor + step)])
    } else {
        u64::from_little_bytes(&file_contents[cursor..(cursor + step)])
    };
    cursor += step;
    println!(
        "\x1b[1;32mStarting Address:\x1b[0m {:#x}\x1b[1m",
        elf_hdr.ei_entry
    );
    elf_hdr.ei_phoff = if elf_hdr.ei_data == 2 {
        u64::from_big_bytes(&file_contents[cursor..(cursor + step)])
    } else {
        u64::from_little_bytes(&file_contents[cursor..(cursor + step)])
    };
    println!(
        "\x1b[1;32mProgram header table start:\x1b[0m {}(Absolute)\x1b[1m",
        elf_hdr.ei_phoff
    );
    cursor += step;
    elf_hdr.ei_shoff = if elf_hdr.ei_data == 2 {
        u64::from_big_bytes(&file_contents[cursor..(cursor + step)])
    } else {
        u64::from_little_bytes(&file_contents[cursor..(cursor + step)])
    };
    println!(
        "\x1b[1;32mSection header table start:\x1b[0m {}(Absolute)\x1b[1m",
        elf_hdr.ei_shoff
    );
    cursor += step;
    elf_hdr.ei_flags = if elf_hdr.ei_data == 2 {
        u32::from_big_bytes(&file_contents[cursor..(cursor + 4)])
    } else {
        u32::from_little_bytes(&file_contents[cursor..(cursor + 4)])
    };
    println!("\x1b[1;32mFlags:\x1b[0m {:#x}\x1b[1m", elf_hdr.ei_flags);
    cursor += 4;
    elf_hdr.ei_ehsize = if elf_hdr.ei_data == 2 {
        u16::from_big_bytes(&file_contents[cursor..(cursor + 2)])
    } else {
        u16::from_little_bytes(&file_contents[cursor..(cursor + 2)])
    };
    println!(
        "\x1b[1;32mELF Header Size:\x1b[0m {} bytes\x1b[1m",
        elf_hdr.ei_ehsize
    );
    cursor += 2;
    elf_hdr.ei_phentsize = if elf_hdr.ei_data == 2 {
        u16::from_big_bytes(&file_contents[cursor..(cursor + 2)])
    } else {
        u16::from_little_bytes(&file_contents[cursor..(cursor + 2)])
    };
    println!(
        "\x1b[1;32mProgram Header Size:\x1b[0m {} bytes\x1b[1m",
        elf_hdr.ei_phentsize
    );
    cursor += 2;
    elf_hdr.ei_phnum = if elf_hdr.ei_data == 2 {
        u16::from_big_bytes(&file_contents[cursor..(cursor + 2)])
    } else {
        u16::from_little_bytes(&file_contents[cursor..(cursor + 2)])
    };
    println!(
        "\x1b[1;32mNumber of program headers:\x1b[0m {}\x1b[1m",
        elf_hdr.ei_phnum
    );
    cursor += 2;
    elf_hdr.ei_shentsize = if elf_hdr.ei_data == 2 {
        u16::from_big_bytes(&file_contents[cursor..(cursor + 2)])
    } else {
        u16::from_little_bytes(&file_contents[cursor..(cursor + 2)])
    };
    println!(
        "\x1b[1;32mSize of section headers:\x1b[0m {} bytes\x1b[1m",
        elf_hdr.ei_shentsize
    );
    cursor += 2;
    elf_hdr.ei_shnum = if elf_hdr.ei_data == 2 {
        u16::from_big_bytes(&file_contents[cursor..(cursor + 2)])
    } else {
        u16::from_little_bytes(&file_contents[cursor..(cursor + 2)])
    };
    println!(
        "\x1b[1;32mNumber of section headers:\x1b[0m {}\x1b[1m",
        elf_hdr.ei_shnum
    );
    cursor += 2;
    elf_hdr.ei_shstrndx = if elf_hdr.ei_data == 2 {
        u16::from_big_bytes(&file_contents[cursor..(cursor + 2)])
    } else {
        u16::from_little_bytes(&file_contents[cursor..(cursor + 2)])
    };
    println!(
        "\x1b[1;32mIndex of section headers:\x1b[0m {}\x1b[1m",
        elf_hdr.ei_shstrndx
    );
    cursor += 2;
    cursor += 56; // second program header (Readable) 0x4
    cursor += 56; // third program header (Exec + Read) 0x5
                  // cursor += 56; // fourth program header (Readable) 0x4
                  // cursor += 56; // fifth program header (Readable and Writable) 0x6
                  // cursor += 56; // fifth program header (Readable and Writable) 0x6
    println!("exec");
    let type_ph = u32::from_little_bytes(&file_contents[cursor..(cursor + 4)]);
    cursor += 4;

    let mut flags = u32::from_little_bytes(&file_contents[cursor..(cursor + 4)]);
    cursor += 4; // resetting

    // while flags != 0x5 {
    //     cursor += 48; // 56 - 8
    //     cursor += 4;
    //     flags = u32::from_little_bytes(&file_contents[cursor..(cursor + 4)]);
    //     cursor += 4;
    // }
    dbg!(cursor);
    println!("\x1b[1;32mFlag:\x1b[0m {:#x}\x1b[1m", flags);
    println!("Program header type: {:#x}", type_ph); // works
    let mut ph_offset = if elf_hdr.ei_data == 2 {
        u64::from_big_bytes(&file_contents[cursor..(cursor + step)]) as usize
    } else {
        u64::from_little_bytes(&file_contents[cursor..(cursor + step)]) as usize
    };
    println!(
        "\x1b[1;32mprogram segment offset:\x1b[0m {}\x1b[1m",
        ph_offset
    );
    ph_offset += 16;
    cursor += step;
    _ = cursor; // IMPORTANT: Comment this line
}
