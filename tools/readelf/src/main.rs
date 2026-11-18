// @author: Amrutansu Garanaik

use libelf_rs::structures::elf32::Elf32;
use libelf_rs::structures::elf32::{
    ELF32_HEADER_SIZE, ELF32_PROGRAM_HEADER_SIZE, ELF32_SECTION_HEADER_SIZE,
    ELF32_SYMBOL_TABLE_SIZE,
};
use libelf_rs::structures::elf64::Elf64;
use libelf_rs::structures::elf64::{
    ELF64_HEADER_SIZE, ELF64_PROGRAM_HEADER_SIZE, ELF64_SECTION_HEADER_SIZE,
    ELF64_SYMBOL_TABLE_SIZE,
};
use libelf_rs::structures::program_header_64::ElfProgramHeader64;
use libelf_rs::structures::section_header_64::ElfSectionHeader64;
use libelf_rs::structures::symbol_table_64::ElfSymbolTable64;
use libelf_rs::types::sh_type::ElfShType;
use std::fs::File;
use std::io::Read;
use clap::{Arg, ArgAction, Command};
use libelf_rs::types::ei_class::ElfClass;

#[rustfmt::skip]
const CMD_OPTIONS: &[(&str, char, &str, &str)] = &[
//   NAME       SHORT    LONG               HELP
    ("header",   'H',    "file-header",     "Display ELF header"),
    ("sections", 'S',    "sections",        "Display section headers"),
    ("programs", 'l',    "program-headers", "Display program headers"),
    ("symbols",  's',    "symbols",         "Display symbol table"),
    ("all",      'a',    "all",             "Display all information"),
];

// TODO: Move section header parsing to a function as it is used by multiple things.
// TODO: Instead of copying buffer to array using copy_from_slice, pass the buffer directly.
// TODO: Add 32-bit ELF support and create a trait so that same functions can be used directly instead of multiple code duplication.
// TODO: Refactor section/program header parsing etc to separate functions.
// TODO: Move public mods to different location, if required.
// TODO: Fix the sh_link hack
// TODO: Improve comments and docs.
// TODO: Add support for other features.
// TODO: Rename strtab and symbol table.
// TODO: Clean the print format.
// TODO: Clean unused variables and clippy errors.

pub mod errors;
pub mod parser;
pub mod printer;

fn main() {
    let mut cmd = Command::new("readelf-rs")
        .version("0.1.0")
        .about("Display information about the contents of ELF format files");
    // Add supported arguments to command.
    for (name, short, long, help) in CMD_OPTIONS {
        cmd = cmd.arg(
            Arg::new(name)
                .short(*short)
                .long(long)
                .help(help)
                .action(ArgAction::SetTrue), // All commands here don't take any argument
        );
    }
    // The final argument is the file name (single file supported) and it is mandatory.
    cmd = cmd.arg(Arg::new("file_name").required(true).index(1));
    let matches = cmd.get_matches(); // parse the cmd arguments to check which commands are there.

    // Boolean variables from command line to know which values to print and which to ignore.
    let mut cmd_print_header = matches.get_flag("header");
    let mut cmd_print_sections = matches.get_flag("sections");
    let mut cmd_print_symbols = matches.get_flag("symbols");
    let mut cmd_print_program_headers = matches.get_flag("programs");
    let cmd_print_all = matches.get_flag("all");

    // if all commands are required, then others can be updated.
    if cmd_print_all {
        cmd_print_header = true;
        cmd_print_sections = true;
        cmd_print_symbols = true;
        cmd_print_program_headers = true;
    }

    // File name is mandaroty, so unwrap() is safe here.
    let file_name = matches.get_one::<String>("file_name").unwrap();

    // If the file cannot be opened, the application cannot proceed. So an unwrap_or_else
    // is safe here as we are bailing from here anyway.
    let mut file = File::open(file_name).unwrap_or_else(|e| {
        eprintln!("Error opening file: {}", e);
        std::process::exit(1);
    });

    // Similar to file open, we are reading the full file at once and if that fails, then
    // also we cannot proceed and need to bail out. So we are using unwrap_or_else here as well.

    let mut buffer: Vec<u8> = Vec::new();
    let read_to_end = file.read_to_end(&mut buffer).unwrap_or_else(|e| {
        eprintln!("Error reading file: {}", e);
        std::process::exit(2);
    });

    // At minimum, the file must contain a valid ELF header and the minimum size is
    if read_to_end < ELF32_HEADER_SIZE {
        eprintln!(
            "File size {} is less than minimum ELF file size {}",
            read_to_end, ELF32_HEADER_SIZE
        );
        std::process::exit(3);
    }

    // Optional headers and sections in ELF file. All may not be present, so using Option here.
    let mut elf64_hdr: Option<Elf64> = None;
    let mut elf32_hdr: Option<Elf32> = None;
    let mut strtab: Option<ElfSectionHeader64> = None; // string table for sections : shstrtab
    let mut symtab: Option<ElfSectionHeader64> = None; // symbol table : symtab
    let mut symtab_str: Option<ElfSectionHeader64> = None; // strings table for symbols : strtab
    let mut strtab_offset: usize = 0; // string table offset in file. 0 means it is not present.
    let mut strtab_size: usize = 0; // size of the table. 0 means no data.
    let mut symtab_offset: usize = 0; // string table offset in file. 0 means it is not present.
    let mut symtab_size: usize = 0; // size of the table. 0 means no data.
    let mut symtab_str_offset: usize = 0; // string table offset for symbols in file. 0 means it is not present.
    let mut symtab_str_size: usize = 0; // size of the table. 0 means no data.

    // parse the identifier to find the header type - 32 bit or 64 bit
    // unwrap() is safe here as the file size must be greater than ELF32_HEADER_SIZE (52)
    let class =
        parser::elf_parse_identifier(buffer[0..16].try_into().unwrap()).unwrap_or_else(|e| {
            eprintln!(
                "Unsupported ELF class. It is neither 32-bit not 64-bit {:?}",
                e
            );
            std::process::exit(4);
        });

    // FIXME: Both if and else block have same logic and only the function changes between 32 bit and 64 bit
    // variant. Anyway to make it compact???
    if class == ElfClass::ElfClass64 {
        // parse for 64-bit data. If header parsing fails, there is no need to proceed further
        // and exit.
        let elf64_hdr = parser::elf64_parse_elf_header(&buffer[0..64]).unwrap_or_else(|e| {
            eprintln!("Invalid 64-bit header {:?}", e);
            std::process::exit(5);
        });

        // If header detail is to be printed, it is printed at the top. So print it first.
        if cmd_print_header {
            printer::print_elf64_header(&elf64_hdr);
        }

        // There can be multiple section headers, program headers etc. They can use the same buffer of required type.
        let mut section_hdr_buffer: [u8; ELF64_SECTION_HEADER_SIZE] =
            [0; ELF64_SECTION_HEADER_SIZE];
        let mut program_hdr_buffer: [u8; ELF64_PROGRAM_HEADER_SIZE] =
            [0; ELF64_PROGRAM_HEADER_SIZE];

        // The string table is used by other sections and segments. Hence its location is present in the
        // header. So first parse the string section header and then store a pointer to the string section.
        let string_section_header_offset =
            elf64_hdr.shoff as usize + (elf64_hdr.shstrndx as usize * elf64_hdr.shentsize as usize);
        section_hdr_buffer.copy_from_slice(
            &buffer[string_section_header_offset
                ..string_section_header_offset + ELF64_SECTION_HEADER_SIZE],
        );
        let str_hdr_try = parser::elf64_parse_section_header(&section_hdr_buffer);
        match str_hdr_try {
            Ok(hdr) => {
                strtab = Some(hdr);
                strtab_offset = hdr.sh_offset as usize;
                strtab_size = hdr.sh_size as usize;
            }
            Err(e) => {} // Not a fatal error, ignore for now.
        }

        // hack:
        let mut symbol_table_idx: u32 = u32::MAX; // MAX MEANS NOT FOUND

        // Once the string table is found, the section headers can be parsed in a loop.
        if cmd_print_sections {
            print!("\n");
            println!("Section Headers: ");
            if elf64_hdr.shnum == 0 {
                println!("No valid section header present.")
            } else {
                println!(
                    "  {:5}{:10}{:20}{:20}{:20}",
                    "[NR]", "Name", "Type", "Address", "Offset"
                );
                println!(
                    "  {:5}{:10}{:20}{:20}{:20}",
                    "", "Size", "EntSize", "Flags Link Info", "Align"
                );
            }
        }
        for i in 0..elf64_hdr.shnum {
            let offset = elf64_hdr.shoff as usize + (i as usize * elf64_hdr.shentsize as usize);
            section_hdr_buffer.copy_from_slice(&buffer[offset..offset + ELF64_SECTION_HEADER_SIZE]);
            let curr_hdr_try = parser::elf64_parse_section_header(&section_hdr_buffer);
            match curr_hdr_try {
                Ok(hdr) => {
                    // Print the section only if required.
                    if cmd_print_sections {
                        printer::elf64_printer_section_header(
                            &hdr,
                            &buffer[strtab_offset..strtab_offset + strtab_size],
                        );
                    }
                    // If the header data points to something else like symbol table, handle that here.
                    if hdr.sh_type == ElfShType::ElfShTypeSymTab {
                        println!("Here 1");
                        symtab = Some(hdr);
                        symtab_offset = hdr.sh_offset as usize;
                        symtab_size = hdr.sh_size as usize;
                        // TODO: Follow the sh_link to find symbol string table
                        // HACK: Right now I am assuming the sh_link >= i
                        if i as u32 <= hdr.sh_link {
                            symbol_table_idx = hdr.sh_link as u32;
                        }
                    }
                    if i as u32 == symbol_table_idx {
                        symtab_str = Some(hdr);
                        symtab_str_offset = hdr.sh_offset as usize;
                        symtab_str_size = hdr.sh_size as usize;
                    }
                }
                Err(e) => {
                    println!("failed to parse the section at index {}", i);
                    println!("Error in the section : {:?}", e);
                    println!("Moving on to the next section");
                }
            }
        }

        // Parse Program header next.
        if cmd_print_program_headers {
            println!("\n");
            println!("Program Headers: ");
            if elf64_hdr.phnum == 0 {
                println!("No valid program header present.")
            } else {
                println!(
                    "  {:16}{:20}{:20}{:20}",
                    "Type", "Offset", "VirtAddr", "PhysAddr"
                );
                println!(
                    "  {:16}{:20}{:20}{:20}",
                    "", "FileSiz", "MemSiz", "Flags Align"
                );
            }
        }
        for i in 0..elf64_hdr.phnum {
            let offset = elf64_hdr.phoff as usize + (i as usize * elf64_hdr.phentsize as usize);
            program_hdr_buffer.copy_from_slice(&buffer[offset..offset + ELF64_PROGRAM_HEADER_SIZE]);
            let curr_hdr_try = parser::elf64_parse_program_header(&program_hdr_buffer);
            match curr_hdr_try {
                Ok(hdr) => {
                    // Print the program headers only if required.
                    if cmd_print_program_headers {
                        printer::elf64_printer_program_header(&hdr);
                    }
                }
                Err(e) => {
                    println!("failed to parse the program header at index {}", i);
                    println!("Error in the program header : {:?}", e);
                    println!("Moving on to the next program header");
                }
            }
        }

        // If Symbol table is found, print is next.
        if symtab.is_some() && cmd_print_symbols {
            printer::elf64_printer_symbol_table(
                &buffer[symtab_offset..symtab_offset + symtab_size],
                &buffer[strtab_offset..strtab_offset + strtab_size],
                &buffer[symtab_str_offset..symtab_str_offset + symtab_str_size],
            );
        }
    }
}

// ~/MyProjects/del/hello
// cargo run -p readelf -- ~/MyProjects/del/hello
