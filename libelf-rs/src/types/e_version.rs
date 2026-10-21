// @author: Amrutansu Garanaik

use crate::elf_errors::ElfErrors;

/// ELF header version (`e_version`).
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum ElfVersion {
    /// Current ELF version.
    ElfVersionCurrent = 1,
    /// Unknown or unsupported version.
    ElfVersionUnknown(u32),
}

impl From<u32> for ElfVersion {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::ElfVersionCurrent,
            _ => Self::ElfVersionUnknown(value),
        }
    }
}

impl From<ElfVersion> for u32 {
    fn from(value: ElfVersion) -> Self {
        match value {
            ElfVersion::ElfVersionCurrent => 1,
            ElfVersion::ElfVersionUnknown(value) => value,
        }
    }
}

impl ElfVersion {
    /// Create an `ElfVersion` from its ELF version field.
    pub fn new(value: u32) -> Self {
        Self::from(value)
    }

    /// Check whether this is a recognized ELF version.
    pub fn validate(&self) -> Result<(), ElfErrors> {
        match self {
            Self::ElfVersionCurrent => Ok(()),
            Self::ElfVersionUnknown(_) => Err(ElfErrors::InvalidElfVersion),
        }
    }

    /// Convert this version into a readable string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ElfVersionCurrent => "Current",
            Self::ElfVersionUnknown(_) => "Unknown version",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_elf_version_conversions_and_strings() {
        for (raw, version, name) in [
            (1, ElfVersion::ElfVersionCurrent, "Current"),
            (2, ElfVersion::ElfVersionUnknown(2), "Unknown version"),
            (
                u32::MAX,
                ElfVersion::ElfVersionUnknown(u32::MAX),
                "Unknown version",
            ),
        ] {
            assert_eq!(ElfVersion::from(raw), version);
            assert_eq!(u32::from(version), raw);
            assert_eq!(ElfVersion::new(raw), version);
            assert_eq!(version.as_str(), name);
        }
    }

    #[test]
    fn test_elf_version_validity() {
        assert_eq!(ElfVersion::ElfVersionCurrent.validate(), Ok(()));
        assert_eq!(
            ElfVersion::ElfVersionUnknown(0).validate(),
            Err(ElfErrors::InvalidElfVersion)
        );
        assert_eq!(
            ElfVersion::ElfVersionUnknown(2).validate(),
            Err(ElfErrors::InvalidElfVersion)
        );
    }
}
