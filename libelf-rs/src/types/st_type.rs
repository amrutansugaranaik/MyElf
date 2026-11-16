// @author: Amrutansu Garanaik

use crate::elf_errors::ElfErrors;

// This file is divided into 4 sections for 4 data types mentioned here.

/* SYMBOL TABLE INFO BIND */

// Symbol table entries are 8-bits where lower nibble is symbol type and upper nibble is the binding.
// This file uses two enums to divide them.

/// Symbol Table Bind Info
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SymTabInfoBind {
    /// Local Symbol
    StbLocal = 0,
    /// Global Symbol
    StbGlobal = 1,
    /// Weak Symbol
    StbWeak = 2,
    /// GNU Unique Symbol
    StbGnuUniqueSymbol = 10,
    /// Invalid : Custom type in this lib to handle unsupported situations.
    StbInvalid = 0xFF,
}

/// Symbol type values occupy the lower 4 bits of `st_info`.
impl From<&SymTabInfoType> for u8 {
    fn from(value: &SymTabInfoType) -> u8 {
        *value as u8
    }
}

/// The upper 4 bits are symbol binding and can be parsed from an 8-bit value.
impl From<&u8> for SymTabInfoBind {
    fn from(value: &u8) -> Self {
        match *value >> 4 {
            0 => SymTabInfoBind::StbLocal,
            1 => SymTabInfoBind::StbGlobal,
            2 => SymTabInfoBind::StbWeak,
            10 => SymTabInfoBind::StbGnuUniqueSymbol,
            _ => SymTabInfoBind::StbInvalid,
        }
    }
}

/// Symbol binding values are encoded in the upper 4 bits of `st_info`.
impl From<&SymTabInfoBind> for u8 {
    fn from(value: &SymTabInfoBind) -> u8 {
        (*value as u8) << 4
    }
}

impl SymTabInfoBind {
    pub fn new(value: u8) -> Self {
        SymTabInfoBind::from(&value)
    }
    pub fn validate(&self) -> Result<(), ElfErrors> {
        match self {
            Self::StbLocal
            | SymTabInfoBind::StbGlobal
            | SymTabInfoBind::StbWeak
            | SymTabInfoBind::StbGnuUniqueSymbol => Ok(()),
            _ => Err(ElfErrors::InvalidSymbolBind),
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::StbLocal => "LOCAL",
            Self::StbGlobal => "GLOBAL",
            Self::StbWeak => "WEAK",
            Self::StbGnuUniqueSymbol => "GNU_UNIQUE",
            _ => "STB_UNKNOWN",
        }
    }
}

/* SYMBOL TABLE INFO TYPE */

/// Symbol Table Type Info
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SymTabInfoType {
    /// No specific type
    SttNoType = 0,
    /// Data object / variable
    SttObject = 1,
    /// Function
    SttFunc = 2,
    /// Section Symbol
    SttSymbol = 3,
    /// Source File
    SttFile = 4,
    /// Common Data Object
    SttCommon = 5,
    /// Thread Local Storage
    SttTls = 6,
    /// GNU Indirect Function
    SttGnuIfunc = 10,
    /// SttInvalid : Custom type in this lib to handle unsupported situations.
    SttInvalid = 0xFF,
}

/// The lower 4 bits are symbol type and can be parsed from an 8-bit value.
impl From<&u8> for SymTabInfoType {
    fn from(value: &u8) -> Self {
        match *value & 0x0f {
            0 => SymTabInfoType::SttNoType,
            1 => SymTabInfoType::SttObject,
            2 => SymTabInfoType::SttFunc,
            3 => SymTabInfoType::SttSymbol,
            4 => SymTabInfoType::SttFile,
            5 => SymTabInfoType::SttCommon,
            6 => SymTabInfoType::SttTls,
            10 => SymTabInfoType::SttGnuIfunc,
            _ => SymTabInfoType::SttInvalid,
        }
    }
}

impl SymTabInfoType {
    pub fn new(value: u8) -> Self {
        SymTabInfoType::from(&value)
    }
    pub fn validate(&self) -> Result<(), ElfErrors> {
        match self {
            Self::SttNoType
            | Self::SttObject
            | Self::SttFunc
            | Self::SttSymbol
            | Self::SttFile
            | Self::SttCommon
            | Self::SttTls
            | Self::SttGnuIfunc => Ok(()),
            _ => Err(ElfErrors::InvalidSymbolType),
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SttNoType => "NOTYPE",
            Self::SttObject => "OBJECT",
            Self::SttFunc => "FUNC",
            Self::SttSymbol => "SECTION", // Check documentation, got this value from chatGPT
            Self::SttFile => "FILE",
            Self::SttCommon => "COMMON",
            Self::SttTls => "TLS",
            Self::SttGnuIfunc => "GNU_IFUNC",
            _ => "STT_UNKNOWN",
        }
    }
}

/* SYMBOL TABLE INFO STRUCTURE */

/// Symbol Table Info structure
/// This contains two enum which together form an 8-bit number. This number
/// is used as a type inside symbol table. So this structure is put here with
/// other types instead of with structures.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// A single structure storing both type and bind since the data is received as a single u8
pub struct SymTabInfo {
    /// Info type
    pub st_type: SymTabInfoType,
    /// Info bind
    pub st_bind: SymTabInfoBind,
}

impl SymTabInfo {
    pub fn new(value: u8) -> SymTabInfo {
        SymTabInfo {
            st_type: SymTabInfoType::from(&value),
            st_bind: SymTabInfoBind::from(&value),
        }
    }
    pub fn validate(&self) -> Result<(), ElfErrors> {
        if self.st_type.validate().is_err() {
            Err(ElfErrors::InvalidSymbolType)
        } else if self.st_bind.validate().is_err() {
            Err(ElfErrors::InvalidSymbolBind)
        } else {
            Ok(())
        }
    }
}

impl From<&u8> for SymTabInfo {
    fn from(value: &u8) -> Self {
        SymTabInfo {
            st_type: SymTabInfoType::from(value),
            st_bind: SymTabInfoBind::from(value),
        }
    }
}

impl From<&SymTabInfo> for u8 {
    fn from(value: &SymTabInfo) -> Self {
        let bind = u8::from(&value.st_bind);
        let type_ = u8::from(&value.st_type);
        bind | type_
    }
}

/* SYMBOL TABLE VISIBILITY */

/// Symbol table other field
/// Stores visibility detail
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SymTabVisibility {
    /// Default visibility
    StvDefault = 0,
    /// Internal visibility
    StvInternal = 1,
    /// Hidden
    StvHidden = 2,
    /// protected
    StvProtected = 3,
    /// Invalid
    StvInvalid = 0xFF,
}

/// Symbol visibility values are stored in the lower two bits of `st_other`.
impl From<&SymTabVisibility> for u8 {
    fn from(value: &SymTabVisibility) -> u8 {
        *value as u8
    }
}

/// The lower bits encode the symbol visibility.
impl From<&u8> for SymTabVisibility {
    fn from(value: &u8) -> Self {
        match *value & 0x03 {
            0 => Self::StvDefault,
            1 => Self::StvInternal,
            2 => Self::StvHidden,
            3 => Self::StvProtected,
            _ => Self::StvInvalid,
        }
    }
}

impl SymTabVisibility {
    pub fn new(value: u8) -> Self {
        SymTabVisibility::from(&value)
    }
    pub fn validate(&self) -> Result<(), ElfErrors> {
        match self {
            SymTabVisibility::StvDefault
            | SymTabVisibility::StvInternal
            | SymTabVisibility::StvHidden
            | SymTabVisibility::StvProtected => Ok(()),
            _ => Err(ElfErrors::InvalidSymTabVisibility),
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::StvDefault => "DEFAULT",
            Self::StvInternal => "INTERNAL",
            Self::StvHidden => "HIDDEN",
            Self::StvProtected => "PROTECTED",
            _ => "STV_UNKNOWN",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SymTabInfo, SymTabInfoBind, SymTabInfoType, SymTabVisibility};

    const BINDINGS: &[(u8, SymTabInfoBind, &str)] = &[
        (0x00, SymTabInfoBind::StbLocal, "LOCAL"),
        (0x10, SymTabInfoBind::StbGlobal, "GLOBAL"),
        (0x20, SymTabInfoBind::StbWeak, "WEAK"),
        (0xA0, SymTabInfoBind::StbGnuUniqueSymbol, "GNU_UNIQUE"),
    ];

    const TYPES: &[(u8, SymTabInfoType, &str)] = &[
        (0, SymTabInfoType::SttNoType, "NOTYPE"),
        (1, SymTabInfoType::SttObject, "OBJECT"),
        (2, SymTabInfoType::SttFunc, "FUNC"),
        (3, SymTabInfoType::SttSymbol, "SECTION"),
        (4, SymTabInfoType::SttFile, "FILE"),
        (5, SymTabInfoType::SttCommon, "COMMON"),
        (6, SymTabInfoType::SttTls, "TLS"),
        (10, SymTabInfoType::SttGnuIfunc, "GNU_IFUNC"),
    ];

    const VISIBILITIES: &[(u8, SymTabVisibility, &str)] = &[
        (0, SymTabVisibility::StvDefault, "DEFAULT"),
        (1, SymTabVisibility::StvInternal, "INTERNAL"),
        (2, SymTabVisibility::StvHidden, "HIDDEN"),
        (3, SymTabVisibility::StvProtected, "PROTECTED"),
    ];

    #[test]
    fn test_symbol_table_binding_conversions_and_strings() {
        for (raw, binding, name) in BINDINGS {
            assert_eq!(SymTabInfoBind::new(*raw), *binding);
            assert_eq!(SymTabInfoBind::from(raw), *binding);
            assert_eq!(u8::from(binding), *raw);
            assert_eq!(binding.as_str(), *name);
            assert!(binding.validate().is_ok());
        }

        assert_eq!(SymTabInfoBind::new(0x30), SymTabInfoBind::StbInvalid);
        assert_eq!(SymTabInfoBind::StbInvalid.as_str(), "STB_UNKNOWN");
        assert!(SymTabInfoBind::StbInvalid.validate().is_err());
    }

    #[test]
    fn test_symbol_table_type_conversions_and_strings() {
        for (raw, kind, name) in TYPES {
            assert_eq!(SymTabInfoType::new(*raw), *kind);
            assert_eq!(SymTabInfoType::from(raw), *kind);
            assert_eq!(u8::from(kind), *raw);
            assert_eq!(kind.as_str(), *name);
            assert!(kind.validate().is_ok());
        }

        assert_eq!(SymTabInfoType::new(0x0F), SymTabInfoType::SttInvalid);
        assert_eq!(SymTabInfoType::SttInvalid.as_str(), "STT_UNKNOWN");
        assert!(SymTabInfoType::SttInvalid.validate().is_err());
    }

    #[test]
    fn test_symbol_table_info_round_trip() {
        for (binding_raw, binding, _) in BINDINGS {
            for (type_raw, kind, _) in TYPES {
                let raw = binding_raw | type_raw;
                let info = SymTabInfo::new(raw);
                assert_eq!(info.st_type, *kind);
                assert_eq!(info.st_bind, *binding);
                assert_eq!(u8::from(&info), raw);
                assert!(info.validate().is_ok());
            }
        }

        let invalid = SymTabInfo::new(0x3F);
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_symbol_visibility_conversions() {
        for (raw, visibility, name) in VISIBILITIES {
            assert_eq!(SymTabVisibility::new(*raw), *visibility);
            assert_eq!(SymTabVisibility::from(raw), *visibility);
            assert_eq!(u8::from(visibility), *raw);
            assert_eq!(visibility.as_str(), *name);
            assert!(visibility.validate().is_ok());
        }

        assert_eq!(SymTabVisibility::new(0xFF), SymTabVisibility::StvProtected);
        assert_eq!(SymTabVisibility::StvInvalid.as_str(), "STV_UNKNOWN");
        assert!(SymTabVisibility::StvInvalid.validate().is_err());
    }
}
