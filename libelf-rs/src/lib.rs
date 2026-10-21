// @author : Amrutansu Garanaik

pub mod elf_errors;
pub mod structures {
    pub mod elf32;
    pub mod elf64;
    pub mod elf_identifier;
    pub mod program_header_32;
    pub mod program_header_64;
    pub mod section_header_32;
    pub mod section_header_64;
}
pub mod types {
    pub mod e_machine;
    pub mod e_type;
    pub mod ei_class;
    pub mod ei_data;
    pub mod ei_osabi;
    pub mod ei_version;
    pub mod p_flags;
    pub mod p_type;
    pub mod sh_flags;
    pub mod sh_type;
}
