// @author: Amrutansu Garanaik

/// List of all errors emited by the library.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ElfErrors {
    /// Unknown or unsupported ELF class.
    InvalidElfClass,
    /// Unknown or unsupported ELF data encoding.
    InvalidElfEndian,
    /// Unknown or Unsupported FileType for ELF.
    InvalidElfFileType,
    /// Unknown or unsupported ELF instruction set architecture.
    InvalidElfIsa,
    /// Unknown or unsupported ELF version.
    InvalidElfVersion,
    /// Unknown or Unsupported OS ABI.
    InvalidOsAbi,
    /// Unknown or unsupported program header type.
    InvalidProgramHeaderType,
    /// Unsupported program header flag bits.
    InvalidProgramHeaderFlags,
    /// Unknown or unsupported section header type.
    InvalidSectionHeaderType,
    /// Unsupported section header flag bits.
    InvalidSectionHeaderFlags,
    /// Elf header marker is not present.
    InvalidMagicNumber,
    // More to be added.
}
