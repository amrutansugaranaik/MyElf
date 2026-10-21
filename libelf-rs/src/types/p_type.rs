// @author: Amrutansu Garanaik

use crate::elf_errors::ElfErrors;

/// ELF program header type (`p_type`).
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum ElfPhType {
    /// Unused program header.
    ElfPhTypeNull = 0,
    /// Loadable segment.
    ElfPhTypeLoad = 1,
    /// Dynamic linking information.
    ElfPhTypeDynamic = 2,
    /// Interpreter path.
    ElfPhTypeInterp = 3,
    /// Auxiliary information.
    ElfPhTypeNote = 4,
    /// Reserved.
    ElfPhTypeShlib = 5,
    /// Program header table.
    ElfPhTypePhdr = 6,
    /// Thread-local storage.
    ElfPhTypeTls = 7,
    /// GNU exception frame header.
    ElfPhTypeGnuEhFrame = 0x6474_e550,
    /// GNU stack permissions.
    ElfPhTypeGnuStack = 0x6474_e551,
    /// GNU read-only-after-relocation segment.
    ElfPhTypeGnuRelro = 0x6474_e552,
    /// GNU property notes.
    ElfPhTypeGnuProperty = 0x6474_e553,
    /// OS-specific program header value in the `PT_LOOS` to `PT_HIOS` range.
    ElfPhTypeOsSpecific(u32),
    /// Processor-specific program header value in the `PT_LOPROC` to
    /// `PT_HIPROC` range.
    ElfPhTypeProcessorSpecific(u32),
    /// Unknown value outside recognized ABI ranges.
    ElfPhTypeUnknown(u32),
}

impl From<u32> for ElfPhType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::ElfPhTypeNull,
            1 => Self::ElfPhTypeLoad,
            2 => Self::ElfPhTypeDynamic,
            3 => Self::ElfPhTypeInterp,
            4 => Self::ElfPhTypeNote,
            5 => Self::ElfPhTypeShlib,
            6 => Self::ElfPhTypePhdr,
            7 => Self::ElfPhTypeTls,
            0x6474_e550 => Self::ElfPhTypeGnuEhFrame,
            0x6474_e551 => Self::ElfPhTypeGnuStack,
            0x6474_e552 => Self::ElfPhTypeGnuRelro,
            0x6474_e553 => Self::ElfPhTypeGnuProperty,
            0x6000_0000..=0x6fff_ffff => Self::ElfPhTypeOsSpecific(value),
            0x7000_0000..=0x7fff_ffff => Self::ElfPhTypeProcessorSpecific(value),
            _ => Self::ElfPhTypeUnknown(value),
        }
    }
}

impl From<ElfPhType> for u32 {
    fn from(value: ElfPhType) -> Self {
        match value {
            ElfPhType::ElfPhTypeNull => 0,
            ElfPhType::ElfPhTypeLoad => 1,
            ElfPhType::ElfPhTypeDynamic => 2,
            ElfPhType::ElfPhTypeInterp => 3,
            ElfPhType::ElfPhTypeNote => 4,
            ElfPhType::ElfPhTypeShlib => 5,
            ElfPhType::ElfPhTypePhdr => 6,
            ElfPhType::ElfPhTypeTls => 7,
            ElfPhType::ElfPhTypeGnuEhFrame => 0x6474_e550,
            ElfPhType::ElfPhTypeGnuStack => 0x6474_e551,
            ElfPhType::ElfPhTypeGnuRelro => 0x6474_e552,
            ElfPhType::ElfPhTypeGnuProperty => 0x6474_e553,
            ElfPhType::ElfPhTypeOsSpecific(value)
            | ElfPhType::ElfPhTypeProcessorSpecific(value)
            | ElfPhType::ElfPhTypeUnknown(value) => value,
        }
    }
}

impl ElfPhType {
    /// Create an `ElfPhType` from a decoded `p_type` value.
    pub fn new(value: u32) -> Self {
        Self::from(value)
    }

    /// Check whether this is a defined generic or recognized extension type.
    pub fn validate(&self) -> Result<(), ElfErrors> {
        match self {
            Self::ElfPhTypeNull
            | Self::ElfPhTypeLoad
            | Self::ElfPhTypeDynamic
            | Self::ElfPhTypeInterp
            | Self::ElfPhTypeNote
            | Self::ElfPhTypeShlib
            | Self::ElfPhTypePhdr
            | Self::ElfPhTypeTls
            | Self::ElfPhTypeGnuEhFrame
            | Self::ElfPhTypeGnuStack
            | Self::ElfPhTypeGnuRelro
            | Self::ElfPhTypeGnuProperty => Ok(()),
            Self::ElfPhTypeOsSpecific(value) if (0x6000_0000..=0x6fff_ffff).contains(value) => {
                Ok(())
            }
            Self::ElfPhTypeProcessorSpecific(value)
                if (0x7000_0000..=0x7fff_ffff).contains(value) =>
            {
                Ok(())
            }
            _ => Err(ElfErrors::InvalidProgramHeaderType),
        }
    }

    /// Convert this program header type into a readable string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ElfPhTypeNull => "Null",
            Self::ElfPhTypeLoad => "Load",
            Self::ElfPhTypeDynamic => "Dynamic",
            Self::ElfPhTypeInterp => "Interpreter",
            Self::ElfPhTypeNote => "Note",
            Self::ElfPhTypeShlib => "Shared library",
            Self::ElfPhTypePhdr => "Program header",
            Self::ElfPhTypeTls => "Thread-local storage",
            Self::ElfPhTypeGnuEhFrame => "GNU EH frame",
            Self::ElfPhTypeGnuStack => "GNU stack",
            Self::ElfPhTypeGnuRelro => "GNU RELRO",
            Self::ElfPhTypeGnuProperty => "GNU property",
            Self::ElfPhTypeOsSpecific(_) => "OS-specific",
            Self::ElfPhTypeProcessorSpecific(_) => "Processor-specific",
            Self::ElfPhTypeUnknown(_) => "Unknown",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const KNOWN: &[(u32, ElfPhType, &str)] = &[
        (0, ElfPhType::ElfPhTypeNull, "Null"),
        (1, ElfPhType::ElfPhTypeLoad, "Load"),
        (2, ElfPhType::ElfPhTypeDynamic, "Dynamic"),
        (3, ElfPhType::ElfPhTypeInterp, "Interpreter"),
        (4, ElfPhType::ElfPhTypeNote, "Note"),
        (5, ElfPhType::ElfPhTypeShlib, "Shared library"),
        (6, ElfPhType::ElfPhTypePhdr, "Program header"),
        (7, ElfPhType::ElfPhTypeTls, "Thread-local storage"),
        (0x6474_e550, ElfPhType::ElfPhTypeGnuEhFrame, "GNU EH frame"),
        (0x6474_e551, ElfPhType::ElfPhTypeGnuStack, "GNU stack"),
        (0x6474_e552, ElfPhType::ElfPhTypeGnuRelro, "GNU RELRO"),
        (0x6474_e553, ElfPhType::ElfPhTypeGnuProperty, "GNU property"),
    ];

    #[test]
    fn test_program_header_type_conversions_and_strings() {
        for (raw, kind, name) in KNOWN {
            assert_eq!(ElfPhType::from(*raw), *kind);
            assert_eq!(u32::from(*kind), *raw);
            assert_eq!(ElfPhType::new(*raw), *kind);
            assert_eq!(kind.as_str(), *name);
        }

        for (raw, kind, name) in [
            (
                0x6000_0000,
                ElfPhType::ElfPhTypeOsSpecific(0x6000_0000),
                "OS-specific",
            ),
            (
                0x6fff_ffff,
                ElfPhType::ElfPhTypeOsSpecific(0x6fff_ffff),
                "OS-specific",
            ),
            (
                0x7000_0000,
                ElfPhType::ElfPhTypeProcessorSpecific(0x7000_0000),
                "Processor-specific",
            ),
            (
                0x7fff_ffff,
                ElfPhType::ElfPhTypeProcessorSpecific(0x7fff_ffff),
                "Processor-specific",
            ),
            (8, ElfPhType::ElfPhTypeUnknown(8), "Unknown"),
            (u32::MAX, ElfPhType::ElfPhTypeUnknown(u32::MAX), "Unknown"),
        ] {
            assert_eq!(ElfPhType::from(raw), kind);
            assert_eq!(u32::from(kind), raw);
            assert_eq!(ElfPhType::new(raw), kind);
            assert_eq!(kind.as_str(), name);
        }
    }

    #[test]
    fn test_program_header_type_validity() {
        for (_, kind, _) in KNOWN {
            assert_eq!(kind.validate(), Ok(()));
        }
        assert_eq!(
            ElfPhType::ElfPhTypeOsSpecific(0x6000_0000).validate(),
            Ok(())
        );
        assert_eq!(
            ElfPhType::ElfPhTypeProcessorSpecific(0x7000_0000).validate(),
            Ok(())
        );
        assert_eq!(
            ElfPhType::ElfPhTypeUnknown(8).validate(),
            Err(ElfErrors::InvalidProgramHeaderType)
        );
        assert_eq!(
            ElfPhType::ElfPhTypeOsSpecific(0x5fff_ffff).validate(),
            Err(ElfErrors::InvalidProgramHeaderType)
        );
        assert_eq!(
            ElfPhType::ElfPhTypeOsSpecific(0x7000_0000).validate(),
            Err(ElfErrors::InvalidProgramHeaderType)
        );
        assert_eq!(
            ElfPhType::ElfPhTypeProcessorSpecific(0x6fff_ffff).validate(),
            Err(ElfErrors::InvalidProgramHeaderType)
        );
        assert_eq!(
            ElfPhType::ElfPhTypeProcessorSpecific(0x8000_0000).validate(),
            Err(ElfErrors::InvalidProgramHeaderType)
        );
    }
}
