// @author: Amrutansu Garanaik

use crate::elf_errors::ElfErrors;

/// ElfFileType enum
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
            ElfFileType::ElfTypeNone
            | ElfFileType::ElfTypeRel
            | ElfFileType::ElfTypeExec
            | ElfFileType::ElfTypeDynRel
            | ElfFileType::ElfTypeCore => {
                return Ok(());
            }
            _ => {
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

mod test {
    use super::*;

    // Check the string values of each type.
    #[test]
    fn test_elf_file_type_as_string() {
        assert_eq!(ElfFileType::ElfTypeNone.as_str(), "None");
        assert_eq!(ElfFileType::ElfTypeRel.as_str(), "Rel");
        assert_eq!(ElfFileType::ElfTypeExec.as_str(), "Exec");
        assert_eq!(ElfFileType::ElfTypeDynRel.as_str(), "DynRel");
        assert_eq!(ElfFileType::ElfTypeCore.as_str(), "Core");
        assert_eq!(ElfFileType::ElfTypeUnknown(0).as_str(), "Unknown");
    }

    // Check elf type to u16 conversion.
    #[test]
    fn test_elf_file_type_as_u16() {
        assert_eq!(u16::from(ElfFileType::ElfTypeNone), 0);
        assert_eq!(u16::from(ElfFileType::ElfTypeRel), 1);
        assert_eq!(u16::from(ElfFileType::ElfTypeExec), 2);
        assert_eq!(u16::from(ElfFileType::ElfTypeDynRel), 3);
        assert_eq!(u16::from(ElfFileType::ElfTypeCore), 4);
        // Any value passed to UnknownType including valid values are just returned back.
        assert_eq!(u16::from(ElfFileType::ElfTypeUnknown(0xff)), 0xff);
        assert_eq!(u16::from(ElfFileType::ElfTypeUnknown(0x1e)), 0x1e);
        assert_eq!(u16::from(ElfFileType::ElfTypeUnknown(0x00)), 0x00);
    }

    // Check u16 to elf type conversion.
    #[test]
    fn test_elf_file_type_from_u16() {
        assert_eq!(ElfFileType::from(0), ElfFileType::ElfTypeNone);
        assert_eq!(ElfFileType::from(1), ElfFileType::ElfTypeRel);
        assert_eq!(ElfFileType::from(2), ElfFileType::ElfTypeExec);
        assert_eq!(ElfFileType::from(3), ElfFileType::ElfTypeDynRel);
        assert_eq!(ElfFileType::from(4), ElfFileType::ElfTypeCore);
        // Any unsupported value will store as UnknownType with that value.
        assert_eq!(ElfFileType::from(5), ElfFileType::ElfTypeUnknown(5));
        assert_eq!(ElfFileType::from(6), ElfFileType::ElfTypeUnknown(6));
    }

    // Check instance creation with new()
    #[test]
    fn test_elf_file_type_new() {
        let my_elf_type = ElfFileType::new(0);
        assert_eq!(my_elf_type, ElfFileType::ElfTypeNone);
        let my_elf_type = ElfFileType::new(1);
        assert_eq!(my_elf_type, ElfFileType::ElfTypeRel);
        let my_elf_type = ElfFileType::new(2);
        assert_eq!(my_elf_type, ElfFileType::ElfTypeExec);
        let my_elf_type = ElfFileType::new(3);
        assert_eq!(my_elf_type, ElfFileType::ElfTypeDynRel);
        let my_elf_type = ElfFileType::new(4);
        assert_eq!(my_elf_type, ElfFileType::ElfTypeCore);
        let my_elf_type = ElfFileType::new(5);
        assert_eq!(my_elf_type, ElfFileType::ElfTypeUnknown(5));
    }

    // Check validity of elf file type
    #[test]
    fn test_elf_file_type_validity() {
        assert_eq!(ElfFileType::ElfTypeNone.validate(), Ok(()));
        assert_eq!(ElfFileType::ElfTypeRel.validate(), Ok(()));
        assert_eq!(ElfFileType::ElfTypeExec.validate(), Ok(()));
        assert_eq!(ElfFileType::ElfTypeDynRel.validate(), Ok(()));
        assert_eq!(ElfFileType::ElfTypeCore.validate(), Ok(()));
        assert_eq!(
            ElfFileType::ElfTypeUnknown(1).validate(),
            Err(ElfErrors::InvalidElfFileType)
        );
        assert_eq!(
            ElfFileType::ElfTypeUnknown(0xff).validate(),
            Err(ElfErrors::InvalidElfFileType)
        );
    }
}
