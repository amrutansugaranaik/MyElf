// @author: Amrutansu Garanaik

use crate::elf_errors::ElfErrors;

/// ELF identification class (`EI_CLASS`), selecting the ELF32 or ELF64 format.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum ElfClass {
    /// 32-bit objects.
    ElfClass32 = 1,
    /// 64-bit objects.
    ElfClass64 = 2,
    /// Unknown or unsupported class.
    ElfClassUnknown(u8),
}

impl From<u8> for ElfClass {
    fn from(value: u8) -> Self {
        match value {
            1 => ElfClass::ElfClass32,
            2 => ElfClass::ElfClass64,
            _ => ElfClass::ElfClassUnknown(value),
        }
    }
}

impl From<ElfClass> for u8 {
    fn from(value: ElfClass) -> u8 {
        match value {
            ElfClass::ElfClass32 => 1,
            ElfClass::ElfClass64 => 2,
            ElfClass::ElfClassUnknown(value) => value,
        }
    }
}

impl ElfClass {
    /// Create an `ElfClass` from its ELF identification byte.
    pub fn new(value: u8) -> Self {
        Self::from(value)
    }

    /// Check whether this is a recognized ELF class.
    pub fn validate(&self) -> Result<(), ElfErrors> {
        match self {
            ElfClass::ElfClass32 | ElfClass::ElfClass64 => Ok(()),
            ElfClass::ElfClassUnknown(_) => Err(ElfErrors::InvalidElfClass),
        }
    }

    /// Convert this class into a readable string.
    pub fn as_str(&self) -> &'static str {
        match self {
            ElfClass::ElfClass32 => "ELF32",
            ElfClass::ElfClass64 => "ELF64",
            ElfClass::ElfClassUnknown(_) => "Unknown ELF class",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_elf_class_as_string() {
        assert_eq!(ElfClass::ElfClass32.as_str(), "ELF32");
        assert_eq!(ElfClass::ElfClass64.as_str(), "ELF64");
        assert_eq!(ElfClass::ElfClassUnknown(3).as_str(), "Unknown ELF class");
    }

    #[test]
    fn test_elf_class_conversions() {
        for (value, class) in [
            (0, ElfClass::ElfClassUnknown(0)),
            (1, ElfClass::ElfClass32),
            (2, ElfClass::ElfClass64),
            (3, ElfClass::ElfClassUnknown(3)),
            (u8::MAX, ElfClass::ElfClassUnknown(u8::MAX)),
        ] {
            assert_eq!(ElfClass::from(value), class);
            assert_eq!(u8::from(class), value);
            assert_eq!(ElfClass::new(value), class);
        }
    }

    #[test]
    fn test_elf_class_validity() {
        assert_eq!(
            ElfClass::ElfClassUnknown(0).validate(),
            Err(ElfErrors::InvalidElfClass)
        );
        assert_eq!(ElfClass::ElfClass32.validate(), Ok(()));
        assert_eq!(ElfClass::ElfClass64.validate(), Ok(()));
        assert_eq!(
            ElfClass::ElfClassUnknown(3).validate(),
            Err(ElfErrors::InvalidElfClass)
        );
    }
}
