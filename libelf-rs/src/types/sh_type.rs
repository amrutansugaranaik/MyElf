// @author: Amrutansu Garanaik

use crate::elf_errors::ElfErrors;

/// ELF section header type (`sh_type`).
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum ElfShType {
    /// Inactive section header.
    ElfShTypeNull = 0,
    /// Program-defined data.
    ElfShTypeProgBits = 1,
    /// Symbol table.
    ElfShTypeSymTab = 2,
    /// String table.
    ElfShTypeStrTab = 3,
    /// Relocations with explicit addends.
    ElfShTypeRela = 4,
    /// Symbol hash table.
    ElfShTypeHash = 5,
    /// Dynamic linking information.
    ElfShTypeDynamic = 6,
    /// Note records.
    ElfShTypeNote = 7,
    /// No file data (for example, BSS).
    ElfShTypeNoBits = 8,
    /// Relocations without explicit addends.
    ElfShTypeRel = 9,
    /// Reserved.
    ElfShTypeShLib = 10,
    /// Dynamic symbol table.
    ElfShTypeDynSym = 11,
    /// Initialization function pointers.
    ElfShTypeInitArray = 14,
    /// Finalization function pointers.
    ElfShTypeFiniArray = 15,
    /// Pre-initialization function pointers.
    ElfShTypePreinitArray = 16,
    /// Section group.
    ElfShTypeGroup = 17,
    /// Extended symbol section indexes.
    ElfShTypeSymTabShndx = 18,
    /// Packed relative relocations.
    ElfShTypeRelr = 19,
    /// GNU object attributes.
    ElfShTypeGnuAttributes = 0x6fff_fff5,
    /// GNU hash table.
    ElfShTypeGnuHash = 0x6fff_fff6,
    /// GNU library list.
    ElfShTypeGnuLibList = 0x6fff_fff7,
    /// Section checksum.
    ElfShTypeChecksum = 0x6fff_fff8,
    /// GNU version definitions.
    ElfShTypeGnuVerDef = 0x6fff_fffd,
    /// GNU version requirements.
    ElfShTypeGnuVerNeed = 0x6fff_fffe,
    /// GNU version symbols.
    ElfShTypeGnuVerSym = 0x6fff_ffff,
    /// OS-specific section type in the `SHT_LOOS` to `SHT_HIOS` range.
    ElfShTypeOsSpecific(u32),
    /// Processor-specific section type in the `SHT_LOPROC` to `SHT_HIPROC`
    /// range.
    ElfShTypeProcessorSpecific(u32),
    /// Application-specific section type in the `SHT_LOUSER` to `SHT_HIUSER`
    /// range.
    ElfShTypeUserSpecific(u32),
    /// Unknown value outside recognized ABI ranges.
    ElfShTypeUnknown(u32),
}

impl From<u32> for ElfShType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::ElfShTypeNull,
            1 => Self::ElfShTypeProgBits,
            2 => Self::ElfShTypeSymTab,
            3 => Self::ElfShTypeStrTab,
            4 => Self::ElfShTypeRela,
            5 => Self::ElfShTypeHash,
            6 => Self::ElfShTypeDynamic,
            7 => Self::ElfShTypeNote,
            8 => Self::ElfShTypeNoBits,
            9 => Self::ElfShTypeRel,
            10 => Self::ElfShTypeShLib,
            11 => Self::ElfShTypeDynSym,
            14 => Self::ElfShTypeInitArray,
            15 => Self::ElfShTypeFiniArray,
            16 => Self::ElfShTypePreinitArray,
            17 => Self::ElfShTypeGroup,
            18 => Self::ElfShTypeSymTabShndx,
            19 => Self::ElfShTypeRelr,
            0x6fff_fff5 => Self::ElfShTypeGnuAttributes,
            0x6fff_fff6 => Self::ElfShTypeGnuHash,
            0x6fff_fff7 => Self::ElfShTypeGnuLibList,
            0x6fff_fff8 => Self::ElfShTypeChecksum,
            0x6fff_fffd => Self::ElfShTypeGnuVerDef,
            0x6fff_fffe => Self::ElfShTypeGnuVerNeed,
            0x6fff_ffff => Self::ElfShTypeGnuVerSym,
            0x6000_0000..=0x6fff_ffff => Self::ElfShTypeOsSpecific(value),
            0x7000_0000..=0x7fff_ffff => Self::ElfShTypeProcessorSpecific(value),
            0x8000_0000..=u32::MAX => Self::ElfShTypeUserSpecific(value),
            _ => Self::ElfShTypeUnknown(value),
        }
    }
}

impl From<ElfShType> for u32 {
    fn from(value: ElfShType) -> Self {
        match value {
            ElfShType::ElfShTypeNull => 0,
            ElfShType::ElfShTypeProgBits => 1,
            ElfShType::ElfShTypeSymTab => 2,
            ElfShType::ElfShTypeStrTab => 3,
            ElfShType::ElfShTypeRela => 4,
            ElfShType::ElfShTypeHash => 5,
            ElfShType::ElfShTypeDynamic => 6,
            ElfShType::ElfShTypeNote => 7,
            ElfShType::ElfShTypeNoBits => 8,
            ElfShType::ElfShTypeRel => 9,
            ElfShType::ElfShTypeShLib => 10,
            ElfShType::ElfShTypeDynSym => 11,
            ElfShType::ElfShTypeInitArray => 14,
            ElfShType::ElfShTypeFiniArray => 15,
            ElfShType::ElfShTypePreinitArray => 16,
            ElfShType::ElfShTypeGroup => 17,
            ElfShType::ElfShTypeSymTabShndx => 18,
            ElfShType::ElfShTypeRelr => 19,
            ElfShType::ElfShTypeGnuAttributes => 0x6fff_fff5,
            ElfShType::ElfShTypeGnuHash => 0x6fff_fff6,
            ElfShType::ElfShTypeGnuLibList => 0x6fff_fff7,
            ElfShType::ElfShTypeChecksum => 0x6fff_fff8,
            ElfShType::ElfShTypeGnuVerDef => 0x6fff_fffd,
            ElfShType::ElfShTypeGnuVerNeed => 0x6fff_fffe,
            ElfShType::ElfShTypeGnuVerSym => 0x6fff_ffff,
            ElfShType::ElfShTypeOsSpecific(value)
            | ElfShType::ElfShTypeProcessorSpecific(value)
            | ElfShType::ElfShTypeUserSpecific(value)
            | ElfShType::ElfShTypeUnknown(value) => value,
        }
    }
}

impl ElfShType {
    /// Create an `ElfShType` from a decoded `sh_type` value.
    pub fn new(value: u32) -> Self {
        Self::from(value)
    }

    /// Check whether this is a defined generic type or an ABI extension value.
    pub fn validate(&self) -> Result<(), ElfErrors> {
        match self {
            Self::ElfShTypeNull
            | Self::ElfShTypeProgBits
            | Self::ElfShTypeSymTab
            | Self::ElfShTypeStrTab
            | Self::ElfShTypeRela
            | Self::ElfShTypeHash
            | Self::ElfShTypeDynamic
            | Self::ElfShTypeNote
            | Self::ElfShTypeNoBits
            | Self::ElfShTypeRel
            | Self::ElfShTypeShLib
            | Self::ElfShTypeDynSym
            | Self::ElfShTypeInitArray
            | Self::ElfShTypeFiniArray
            | Self::ElfShTypePreinitArray
            | Self::ElfShTypeGroup
            | Self::ElfShTypeSymTabShndx
            | Self::ElfShTypeRelr
            | Self::ElfShTypeGnuAttributes
            | Self::ElfShTypeGnuHash
            | Self::ElfShTypeGnuLibList
            | Self::ElfShTypeChecksum
            | Self::ElfShTypeGnuVerDef
            | Self::ElfShTypeGnuVerNeed
            | Self::ElfShTypeGnuVerSym => Ok(()),
            Self::ElfShTypeOsSpecific(value) if (0x6000_0000..=0x6fff_ffff).contains(value) => {
                Ok(())
            }
            Self::ElfShTypeProcessorSpecific(value)
                if (0x7000_0000..=0x7fff_ffff).contains(value) =>
            {
                Ok(())
            }
            Self::ElfShTypeUserSpecific(value) if *value >= 0x8000_0000 => Ok(()),
            _ => Err(ElfErrors::InvalidSectionHeaderType),
        }
    }

    /// Convert this section header type into a readable string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ElfShTypeNull => "Null",
            Self::ElfShTypeProgBits => "Program data",
            Self::ElfShTypeSymTab => "Symbol table",
            Self::ElfShTypeStrTab => "String table",
            Self::ElfShTypeRela => "Relocations with addends",
            Self::ElfShTypeHash => "Hash table",
            Self::ElfShTypeDynamic => "Dynamic",
            Self::ElfShTypeNote => "Note",
            Self::ElfShTypeNoBits => "No bits",
            Self::ElfShTypeRel => "Relocations",
            Self::ElfShTypeShLib => "Shared library",
            Self::ElfShTypeDynSym => "Dynamic symbols",
            Self::ElfShTypeInitArray => "Init array",
            Self::ElfShTypeFiniArray => "Fini array",
            Self::ElfShTypePreinitArray => "Preinit array",
            Self::ElfShTypeGroup => "Group",
            Self::ElfShTypeSymTabShndx => "Extended symbol indexes",
            Self::ElfShTypeRelr => "Packed relative relocations",
            Self::ElfShTypeGnuAttributes => "GNU attributes",
            Self::ElfShTypeGnuHash => "GNU hash",
            Self::ElfShTypeGnuLibList => "GNU library list",
            Self::ElfShTypeChecksum => "Checksum",
            Self::ElfShTypeGnuVerDef => "GNU version definitions",
            Self::ElfShTypeGnuVerNeed => "GNU version requirements",
            Self::ElfShTypeGnuVerSym => "GNU version symbols",
            Self::ElfShTypeOsSpecific(_) => "OS-specific",
            Self::ElfShTypeProcessorSpecific(_) => "Processor-specific",
            Self::ElfShTypeUserSpecific(_) => "User-specific",
            Self::ElfShTypeUnknown(_) => "Unknown",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const KNOWN: &[(u32, ElfShType, &str)] = &[
        (0, ElfShType::ElfShTypeNull, "Null"),
        (1, ElfShType::ElfShTypeProgBits, "Program data"),
        (2, ElfShType::ElfShTypeSymTab, "Symbol table"),
        (3, ElfShType::ElfShTypeStrTab, "String table"),
        (4, ElfShType::ElfShTypeRela, "Relocations with addends"),
        (5, ElfShType::ElfShTypeHash, "Hash table"),
        (6, ElfShType::ElfShTypeDynamic, "Dynamic"),
        (7, ElfShType::ElfShTypeNote, "Note"),
        (8, ElfShType::ElfShTypeNoBits, "No bits"),
        (9, ElfShType::ElfShTypeRel, "Relocations"),
        (10, ElfShType::ElfShTypeShLib, "Shared library"),
        (11, ElfShType::ElfShTypeDynSym, "Dynamic symbols"),
        (14, ElfShType::ElfShTypeInitArray, "Init array"),
        (15, ElfShType::ElfShTypeFiniArray, "Fini array"),
        (16, ElfShType::ElfShTypePreinitArray, "Preinit array"),
        (17, ElfShType::ElfShTypeGroup, "Group"),
        (
            18,
            ElfShType::ElfShTypeSymTabShndx,
            "Extended symbol indexes",
        ),
        (19, ElfShType::ElfShTypeRelr, "Packed relative relocations"),
        (
            0x6fff_fff5,
            ElfShType::ElfShTypeGnuAttributes,
            "GNU attributes",
        ),
        (0x6fff_fff6, ElfShType::ElfShTypeGnuHash, "GNU hash"),
        (
            0x6fff_fff7,
            ElfShType::ElfShTypeGnuLibList,
            "GNU library list",
        ),
        (0x6fff_fff8, ElfShType::ElfShTypeChecksum, "Checksum"),
        (
            0x6fff_fffd,
            ElfShType::ElfShTypeGnuVerDef,
            "GNU version definitions",
        ),
        (
            0x6fff_fffe,
            ElfShType::ElfShTypeGnuVerNeed,
            "GNU version requirements",
        ),
        (
            0x6fff_ffff,
            ElfShType::ElfShTypeGnuVerSym,
            "GNU version symbols",
        ),
        (
            0x6000_0000,
            ElfShType::ElfShTypeOsSpecific(0x6000_0000),
            "OS-specific",
        ),
        (
            0x6fff_fffc,
            ElfShType::ElfShTypeOsSpecific(0x6fff_fffc),
            "OS-specific",
        ),
        (
            0x7000_0000,
            ElfShType::ElfShTypeProcessorSpecific(0x7000_0000),
            "Processor-specific",
        ),
        (
            0x7fff_ffff,
            ElfShType::ElfShTypeProcessorSpecific(0x7fff_ffff),
            "Processor-specific",
        ),
        (
            0x8000_0000,
            ElfShType::ElfShTypeUserSpecific(0x8000_0000),
            "User-specific",
        ),
        (
            u32::MAX,
            ElfShType::ElfShTypeUserSpecific(u32::MAX),
            "User-specific",
        ),
        (12, ElfShType::ElfShTypeUnknown(12), "Unknown"),
    ];

    #[test]
    fn test_section_header_type_conversions() {
        for (raw, kind, _) in KNOWN {
            assert_eq!(ElfShType::from(*raw), *kind);
            assert_eq!(u32::from(*kind), *raw);
            assert_eq!(ElfShType::new(*raw), *kind);
        }
    }

    #[test]
    fn test_section_header_type_strings() {
        for (_, kind, name) in KNOWN {
            assert_eq!(kind.as_str(), *name);
        }
    }

    #[test]
    fn test_section_header_type_unknown_round_trips() {
        for raw in [12, 13] {
            let kind = ElfShType::ElfShTypeUnknown(raw);
            assert_eq!(ElfShType::from(raw), kind);
            assert_eq!(u32::from(kind), raw);
            assert_eq!(ElfShType::new(raw), kind);
        }
    }

    #[test]
    fn test_section_header_type_strings_and_validity() {
        for (_, kind, _) in KNOWN {
            let expected = if matches!(kind, ElfShType::ElfShTypeUnknown(_)) {
                Err(ElfErrors::InvalidSectionHeaderType)
            } else {
                Ok(())
            };
            assert_eq!(kind.validate(), expected);
        }
        assert_eq!(
            ElfShType::ElfShTypeUnknown(12).validate(),
            Err(ElfErrors::InvalidSectionHeaderType)
        );
        assert_eq!(
            ElfShType::ElfShTypeOsSpecific(0x5fff_ffff).validate(),
            Err(ElfErrors::InvalidSectionHeaderType)
        );
        assert_eq!(
            ElfShType::ElfShTypeOsSpecific(0x7000_0000).validate(),
            Err(ElfErrors::InvalidSectionHeaderType)
        );
        assert_eq!(
            ElfShType::ElfShTypeProcessorSpecific(0x6fff_ffff).validate(),
            Err(ElfErrors::InvalidSectionHeaderType)
        );
        assert_eq!(
            ElfShType::ElfShTypeProcessorSpecific(0x8000_0000).validate(),
            Err(ElfErrors::InvalidSectionHeaderType)
        );
        assert_eq!(
            ElfShType::ElfShTypeUserSpecific(0x7fff_ffff).validate(),
            Err(ElfErrors::InvalidSectionHeaderType)
        );
    }
}
