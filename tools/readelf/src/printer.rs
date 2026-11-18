// @author: Amrutansu Garanaik

use crate::errors::ReadElfErrors;
use crate::parser;
use libelf_rs::elf_errors::ElfErrors;

use libelf_rs::structures::elf32::{
    ELF32_HEADER_SIZE, ELF32_PROGRAM_HEADER_SIZE, ELF32_SECTION_HEADER_SIZE,
    ELF32_SYMBOL_TABLE_SIZE,
};

use libelf_rs::structures::elf_identifier::ElfIdentifier;
use libelf_rs::structures::elf64::{
    ELF64_HEADER_SIZE, ELF64_PROGRAM_HEADER_SIZE, ELF64_SECTION_HEADER_SIZE,
    ELF64_SYMBOL_TABLE_SIZE,
};
use libelf_rs::types::ei_class::ElfClass;
use libelf_rs::types::st_type::SymTabInfoType;

use libelf_rs::structures::elf32::Elf32;
use libelf_rs::structures::elf64::Elf64;
use libelf_rs::structures::program_header_32::ElfProgramHeader32;
use libelf_rs::structures::program_header_64::ElfProgramHeader64;
use libelf_rs::structures::section_header_32::ElfSectionHeader32;
use libelf_rs::structures::section_header_64::ElfSectionHeader64;

use libelf_rs::structures::symbol_table_32::ElfSymbolTable32;
use libelf_rs::structures::symbol_table_64::ElfSymbolTable64;

/// internal function to print the identifier.
/// FIXME: Replace the println spaces with {:x} for better formatting.
fn elf_printer_header_identifier(ei: &ElfIdentifier) {
    let hdr_bytes: [u8; 16] = From::from(*ei);

    println!("ELF Header:");

    // Print magic : All the 16 bytes of the header in hex
    print!("  Magic:  ");
    for i in 0..16 {
        print!(" {:02x}", hdr_bytes[i]);
    }
    println!("");

    // Remaining data in the header
    println!("  Class:                             {}", ei.class.as_str());
    println!("  Data:                              {}", ei.data.as_str());
    println!(
        "  Version:                           {}",
        ei.version.as_str()
    );
    println!(
        "  OS/ABI:                            {}",
        ei.os_abi.as_str()
    );
    println!(
        "  ABI Version:                       {}",
        ei.abi_version as u8
    );
}

pub fn print_elf64_header(hdr: &Elf64) {
    elf_printer_header_identifier(&hdr.identifier);

    // Print elf64 data
    println!(
        "  Type:                              {}",
        hdr.file_type.as_str()
    );
    println!(
        "  Machine:                           {}",
        hdr.machine.as_str()
    );
    println!(
        "  Version:                           0x{:x}",
        hdr.version as u32
    );
    println!(
        "  Entry point address:               0x{:x}",
        hdr.entry as u64
    );
    println!(
        "  Start of program headers:          {} (bytes into file)",
        hdr.phoff as u64
    );
    println!(
        "  Start of section headers:          {} (bytes into file)",
        hdr.shoff as u64
    );
    println!(
        "  Flags:                             0x{:x}",
        hdr.flags as u8
    );
    println!(
        "  Size of this header:               {} (bytes)",
        hdr.ehsize as u16
    );
    println!(
        "  Size of program headers:           {} (bytes)",
        hdr.phentsize as u16
    );
    println!("  Number of program headers:         {}", hdr.phnum as u16);
    println!(
        "  Size of section headers:           {} (bytes)",
        hdr.shentsize as u16
    );
    println!("  Number of section headers:         {}", hdr.shnum as u16);
    println!(
        "  Section header string table index: {}",
        hdr.shstrndx as u16
    );
}

pub fn elf64_printer_program_header(phdr: &ElfProgramHeader64) {
    print!("  {:16}", phdr.p_type.as_str());
    print!("0x{:016x}", phdr.p_offset);
    print!("  0x{:016x}", phdr.p_vaddr);
    print!("  0x{:016x}", phdr.p_paddr);
    print!("\n  {:016}", ""); // Move to next line and have 16 blank spaces.
    print!("0x{:016x}", phdr.p_filesz);
    print!("  0x{:016x}", phdr.p_memsz);
    print!("   {}", phdr.p_flags.as_str_rwe());
    print!("  0x{:x}", phdr.p_align);
    print!("\n");
}

pub fn elf32_printer_program_header(phdr: &ElfProgramHeader64) {
    print!("  {:16}", phdr.p_type.as_str());
    print!("0x{:016x}", phdr.p_offset);
    print!("  0x{:016x}", phdr.p_vaddr);
    print!("  0x{:016x}", phdr.p_paddr);
    print!("\n  {:016}", ""); // Move to next line and have 16 blank spaces.
    print!("0x{:016x}", phdr.p_filesz);
    print!("  0x{:016x}", phdr.p_memsz);
    print!("   {}", phdr.p_flags.as_str_rwe());
    print!("  0x{:x}", phdr.p_align);
    print!("\n");
}

pub fn elf64_printer_section_header(shdr: &ElfSectionHeader64, string_buffer: &[u8]) {
    let name = parser::elf_parse_string_name(string_buffer, shdr.sh_name as usize);
    println!("{}", name);
    print!("      {:016}", shdr.sh_type.as_str());
    print!("  {:016x}", shdr.sh_addr);
    print!("  {:016x}", shdr.sh_addralign);
    print!("\n  {:016}", ""); // Move to next line and have 16 blank spaces.
    print!("0x{:016x}", shdr.sh_size);
    print!("  0x{:016x}", shdr.sh_entsize);
    // print! ("   {}", shdr.sh_flags.as_str());    // TODO FIXME
    print!("  {}", shdr.sh_link);
    print!("  {}", shdr.sh_info);
    print!("  {}", shdr.sh_addralign);
    print!("\n");
}

pub fn elf64_printer_symbol_table(
    sym_table_buffer: &[u8],
    sh_str_table_buffer: &[u8],
    str_table_buffer: &[u8],
) {
    let size = sym_table_buffer.len();
    println!(
        "{} {} {}",
        sym_table_buffer.len(),
        sh_str_table_buffer.len(),
        str_table_buffer.len()
    );
    // each entry is ELF64_SYMBOL_TABLE_SIZE=24 bytes.
    if size % ELF64_SYMBOL_TABLE_SIZE != 0 {
        println!("Invalid size of data. Please check");
    }

    let entry_cnt = size / ELF64_SYMBOL_TABLE_SIZE;
    println!("Number of entries in the symbol table = {}", entry_cnt);

    println!(
        "{:4} {:16} {:5} {:8} {:6} {:7} {:6} {}",
        "Num:", "Value", "Size", "Type", "Bind", "Vis", "Ndx", "Name"
    );

    for i in 0..entry_cnt {
        let offset = i * ELF64_SYMBOL_TABLE_SIZE;
        let mut data: [u8; ELF64_SYMBOL_TABLE_SIZE] = [0; ELF64_SYMBOL_TABLE_SIZE];
        data.copy_from_slice(&sym_table_buffer[offset..offset + ELF64_SYMBOL_TABLE_SIZE]);
        let table_entry = ElfSymbolTable64::from(&data);
        let string_name;
        // SttSymbol means the entry is about a section
        if table_entry.st_info.st_type == SymTabInfoType::SttSymbol {
            // FIXME : st_shndx tells the index number of the section to which this points to.
            // Get the name from that section and use it here. For now, I have passed 0 as the
            // first character is NULL in sh_str table.
            string_name = parser::elf_parse_string_name(sh_str_table_buffer, 0);
        } else {
            string_name = parser::elf_parse_string_name(str_table_buffer, table_entry.st_name);
        }
        println!(
            "{:3}: {:016x} {:4} {:8} {:6} {:7} {:6}  {}",
            i,
            table_entry.st_value,
            table_entry.st_size,
            table_entry.st_info.st_type.as_str(),
            table_entry.st_info.st_bind.as_str(),
            table_entry.st_other.as_str(),
            parser::elf_parse_section_index(table_entry.st_shndx),
            string_name
        );
    }
}
