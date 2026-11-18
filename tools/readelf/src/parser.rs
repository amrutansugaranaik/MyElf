// @author: Amrutansu Garanaik

use crate::errors::ReadElfErrors;
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

use libelf_rs::structures::elf32::Elf32;
use libelf_rs::structures::elf64::Elf64;
use libelf_rs::structures::program_header_32::ElfProgramHeader32;
use libelf_rs::structures::program_header_64::ElfProgramHeader64;
use libelf_rs::structures::section_header_32::ElfSectionHeader32;
use libelf_rs::structures::section_header_64::ElfSectionHeader64;

/// Parses the ElfIdentifier from a buffer. It only checks the the
/// header is valid, and if so, returns the Class to identify 32 or 64 bit elf.
pub fn elf_parse_identifier(elf_ident_buffer: &[u8; 16]) -> Result<ElfClass, ReadElfErrors> {
    if elf_ident_buffer.len() < 16 {
        return Err(ReadElfErrors::BufferTooSmall);
    }
    let ei = ElfIdentifier::from(elf_ident_buffer);
    if ei.magic != [0x7F, b'E', b'L', b'F'] {
        return Err(ReadElfErrors::MissingElfMarker);
    }
    if ei.class.validate().is_err() {
        return Err(ReadElfErrors::UnsupportedClass);
    }
    // FIXME: Should we only check magic and class or validate the full identifier?
    return Ok(ei.class);
}

/// Take a buffer and parse it assuming it has the elf header and finally check if the header is proper.
pub fn elf64_parse_elf_header(elf64_buffer: &[u8]) -> Result<Elf64, ReadElfErrors> {
    let mut data: [u8; ELF64_HEADER_SIZE] = [0; ELF64_HEADER_SIZE];
    data.copy_from_slice(&elf64_buffer[0..ELF64_HEADER_SIZE]);
    let e64 = Elf64::from(&data);
    let ret = e64.validate();
    if ret.is_err() {
        return Err(ReadElfErrors::InvalidElf64Header);
    }
    Ok(e64)
}

/// Take a buffer and parse it assuming it has the elf header and finally check if the header is proper.
pub fn elf32_parse_elf_header(elf32_buffer: &[u8]) -> Result<Elf32, ReadElfErrors> {
    if elf32_buffer.len() < ELF64_HEADER_SIZE {
        return Err(ReadElfErrors::BufferTooSmall);
    }
    let mut data: [u8; ELF32_HEADER_SIZE] = [0; ELF32_HEADER_SIZE];
    data.copy_from_slice(&elf32_buffer[0..ELF32_HEADER_SIZE]);
    let e32 = Elf32::from(&data);
    let ret = e32.validate();
    if ret.is_err() {
        return Err(ReadElfErrors::InvalidElf32Header);
    }
    Ok(e32)
}

/// Take a buffer and parse it assuming it has the 64-bit program header.
pub fn elf64_parse_program_header(phdr_buffer: &[u8]) -> Result<ElfProgramHeader64, ReadElfErrors> {
    if phdr_buffer.len() < ELF64_PROGRAM_HEADER_SIZE {
        return Err(ReadElfErrors::BufferTooSmall);
    }
    let mut data: [u8; ELF64_PROGRAM_HEADER_SIZE] = [0; ELF64_PROGRAM_HEADER_SIZE];
    data.copy_from_slice(&phdr_buffer[0..ELF64_PROGRAM_HEADER_SIZE]);
    let ph_e64 = ElfProgramHeader64::from(&data);
    let ret = ph_e64.validate();
    if ret.is_err() {
        return Err(ReadElfErrors::InvalidElf64ProgramHeader);
    }
    Ok(ph_e64)
}

/// Take a buffer and parse it assuming it has the 32-bit program header.
pub fn elf32_parse_program_header(phdr_buffer: &[u8]) -> Result<ElfProgramHeader32, ReadElfErrors> {
    if phdr_buffer.len() < ELF32_PROGRAM_HEADER_SIZE {
        return Err(ReadElfErrors::BufferTooSmall);
    }
    let mut data: [u8; ELF32_PROGRAM_HEADER_SIZE] = [0; ELF32_PROGRAM_HEADER_SIZE];
    data.copy_from_slice(&phdr_buffer[0..ELF32_PROGRAM_HEADER_SIZE]);
    let ph_e32 = ElfProgramHeader32::from(&data);
    let ret = ph_e32.validate();
    if ret.is_err() {
        return Err(ReadElfErrors::InvalidElf32ProgramHeader);
    }
    Ok(ph_e32)
}

/// Take a buffer and parse it assuming it has the 64-bit section header.
pub fn elf64_parse_section_header(shdr_buffer: &[u8]) -> Result<ElfSectionHeader64, ReadElfErrors> {
    if shdr_buffer.len() < ELF64_SECTION_HEADER_SIZE {
        return Err(ReadElfErrors::BufferTooSmall);
    }
    let mut data: [u8; ELF64_SECTION_HEADER_SIZE] = [0; ELF64_SECTION_HEADER_SIZE];
    data.copy_from_slice(&shdr_buffer[0..ELF64_SECTION_HEADER_SIZE]);
    let sh_e64 = ElfSectionHeader64::from(&data);
    let ret = sh_e64.validate();
    if ret.is_err() {
        return Err(ReadElfErrors::InvalidElf64SectionHeader);
    }
    Ok(sh_e64)
}

/// Take a buffer and parse it assuming it has the 32-bit section header.
pub fn elf32_parse_section_header(shdr_buffer: &[u8]) -> Result<ElfSectionHeader32, ReadElfErrors> {
    if shdr_buffer.len() < ELF32_SECTION_HEADER_SIZE {
        return Err(ReadElfErrors::BufferTooSmall);
    }
    let mut data: [u8; ELF32_SECTION_HEADER_SIZE] = [0; ELF32_SECTION_HEADER_SIZE];
    data.copy_from_slice(&shdr_buffer[0..ELF32_SECTION_HEADER_SIZE]);
    let sh_e32 = ElfSectionHeader32::from(&data);
    let ret = sh_e32.validate();
    if ret.is_err() {
        return Err(ReadElfErrors::InvalidElf32SectionHeader);
    }
    Ok(sh_e32)
}

/// Parse a string name from the buffer. All the string tables use the same format.
/// Index tells the start location of the string and the string is null-terminated.
/// If the index is outside the string_buffer, an empty string is returned. The API
/// is common for 32 and 64 bit variants of the ELF file.
///
/// NOTE: It is the caller's responsibility to ensure that string_buffer is a complete
/// buffer of strings.
pub fn elf_parse_string_name(string_buffer: &[u8], index: usize) -> String {
    let mut s = "".to_string();
    if string_buffer.len() < index || string_buffer.len() == 0 {
        return s;
    }
    let mut idx = index;

    while string_buffer[idx] != 0 {
        s.push(string_buffer[idx] as char);
        idx += 1;
    }
    return s;
}

/// Section Index in the symbol table contains value which will be mapped to
/// a readable string based on its value. If value is not defined, it is just
/// converted to string and returned.
pub fn elf_parse_section_index(ndx: usize) -> String {
    let mut str = String::new();
    if ndx == 0 {
        str = "UND".to_string();
    } else if ndx == 0xfff1 {
        str = "ABS".to_string();
    } else if ndx == 0xfff2 {
        str = "COMMON".to_string();
    } else if ndx == 0xffff {
        str = "XINDEX".to_string();
    } else {
        str = ndx.to_string();
    }
    return str;
}
