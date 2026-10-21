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
    /// Program header file size exceeds its memory size for a loadable segment.
    InvalidProgramHeaderSizes,
    /// Program header alignment is not zero, one, or a power of two.
    InvalidProgramHeaderAlignment,
    /// Unknown or unsupported section header type.
    InvalidSectionHeaderType,
    /// Unsupported section header flag bits.
    InvalidSectionHeaderFlags,
    /// Section header alignment is not zero, one, or a power of two.
    InvalidSectionHeaderAlignment,
    /// Elf header marker is not present.
    InvalidMagicNumber,
    /// ELF header size does not match its ELF class.
    InvalidElfHeaderSize,
    // More to be added.
}
