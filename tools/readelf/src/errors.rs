// @author: Amrutansu Garanaik

#[derive(Copy, Clone, Debug)]
pub enum ReadElfErrors {
    /// ELF marker not present, file is not an ELF file
    MissingElfMarker,
    /// Not 32 and 64 bit ELF file
    UnsupportedClass,
    /// Not enough space in the buffer to proceed.
    BufferTooSmall,
    /// validate() of Elf64 header failed
    InvalidElf64Header,
    /// validate() of Elf32 header failed
    InvalidElf32Header,
    /// Invalid of Elf64 Section Header failed
    InvalidElf64SectionHeader,
    /// Invalid of Elf32 Section Header failed
    InvalidElf32SectionHeader,
    /// Invalid of Elf64 Program Header failed
    InvalidElf64ProgramHeader,
    /// Invalid of Elf32 Program Header failed
    InvalidElf32ProgramHeader,
}
