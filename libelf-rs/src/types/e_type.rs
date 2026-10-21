// @author: Amrutansu Garanaik

use crate::elf_errors::ElfErrors;

/// ELF file type (`e_type`).
///
/// Each entry indicates one of the possible ELF file types.
/// ElfTypeUnknown(u16) is used to indicate any file type is unknown for ELF.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u16)]
pub enum ElfFileType {
    /// Unknown variant
    ElfTypeNone = 0,
    /// Relocatable object file
    ElfTypeRel = 1,
    /// Executable file
    ElfTypeExec = 2,
    /// Dynamic shared object file
    ElfTypeDynRel = 3,
    /// Core dump file
    ElfTypeCore = 4,
    /// Unknown or Unsupported type
    ElfTypeUnknown(u16),
}

impl From<u16> for ElfFileType {
    /// Converts a raw `u16` ELF file type value into an [`ElfFileType`].
    fn from(value: u16) -> Self {
        match value {
            0 => ElfFileType::ElfTypeNone,
            1 => ElfFileType::ElfTypeRel,
            2 => ElfFileType::ElfTypeExec,
            3 => ElfFileType::ElfTypeDynRel,
            4 => ElfFileType::ElfTypeCore,
            _ => ElfFileType::ElfTypeUnknown(value),
        }
    }
}

impl From<ElfFileType> for u16 {
    /// Converts an `ElfFileType` enum into raw u16 value.
    fn from(value: ElfFileType) -> u16 {
        match value {
            ElfFileType::ElfTypeNone => 0,
            ElfFileType::ElfTypeRel => 1,
            ElfFileType::ElfTypeExec => 2,
            ElfFileType::ElfTypeDynRel => 3,
            ElfFileType::ElfTypeCore => 4,
            ElfFileType::ElfTypeUnknown(val) => val,
        }
    }
}

impl ElfFileType {
    /// Create a new instance of `ElfFileType` from a raw u16 value.
    pub fn new(value: u16) -> Self {
        return ElfFileType::from(value);
    }

    /// Checks if the value is a valid ELF file type.
    pub fn validate(&self) -> Result<(), ElfErrors> {
        match self {
            ElfFileType::ElfTypeRel
            | ElfFileType::ElfTypeExec
            | ElfFileType::ElfTypeDynRel
            | ElfFileType::ElfTypeCore => {
                return Ok(());
            }
            ElfFileType::ElfTypeNone | ElfFileType::ElfTypeUnknown(_) => {
                return Err(ElfErrors::InvalidElfFileType);
            }
        }
    }

    /// Converts an `ElfFileType` enum into a readable string.
    pub fn as_str(&self) -> &'static str {
        match self {
            ElfFileType::ElfTypeNone => "None",
            ElfFileType::ElfTypeRel => "Rel",
            ElfFileType::ElfTypeExec => "Exec",
            ElfFileType::ElfTypeDynRel => "DynRel",
            ElfFileType::ElfTypeCore => "Core",
            ElfFileType::ElfTypeUnknown(_) => "Unknown",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const KNOWN: &[(u16, ElfFileType, &str)] = &[
        (0, ElfFileType::ElfTypeNone, "None"),
        (1, ElfFileType::ElfTypeRel, "Rel"),
        (2, ElfFileType::ElfTypeExec, "Exec"),
        (3, ElfFileType::ElfTypeDynRel, "DynRel"),
        (4, ElfFileType::ElfTypeCore, "Core"),
    ];

    #[test]
    fn test_elf_file_type_conversions_and_strings() {
        for (raw, file_type, name) in KNOWN {
            assert_eq!(ElfFileType::from(*raw), *file_type);
            assert_eq!(u16::from(*file_type), *raw);
            assert_eq!(ElfFileType::new(*raw), *file_type);
            assert_eq!(file_type.as_str(), *name);
        }
        for raw in [5, 6, u16::MAX] {
            let unknown = ElfFileType::ElfTypeUnknown(raw);
            assert_eq!(ElfFileType::from(raw), unknown);
            assert_eq!(u16::from(unknown), raw);
            assert_eq!(ElfFileType::new(raw), unknown);
            assert_eq!(unknown.as_str(), "Unknown");
        }
    }

    #[test]
    fn test_elf_file_type_validity() {
        for (raw, file_type, _) in KNOWN {
            let expected = if *raw == 0 {
                Err(ElfErrors::InvalidElfFileType)
            } else {
                Ok(())
            };
            assert_eq!(file_type.validate(), expected);
        }
        assert_eq!(
            ElfFileType::ElfTypeUnknown(0xff).validate(),
            Err(ElfErrors::InvalidElfFileType)
        );
    }
}
