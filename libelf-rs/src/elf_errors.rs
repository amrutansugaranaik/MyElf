// @author: Amrutansu Garanaik

/// List of all errors emited by the library.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ElfErrors {
    /// Unknown or Unsupported FileType for ELF.
    InvalidElfFileType,
    // More to be added.
}
