// @author: Amrutansu Garanaik

use crate::elf_errors::ElfErrors;

/// ELF identification data encoding (`EI_DATA`), describing byte order.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum ElfEndian {
    /// Little-endian data.
    ElfEndianLittle = 1,
    /// Big-endian data.
    ElfEndianBig = 2,
    /// Unknown or unsupported data encoding.
    ElfEndianUnknown(u8),
}

impl From<u8> for ElfEndian {
    fn from(value: u8) -> Self {
        match value {
            1 => ElfEndian::ElfEndianLittle,
            2 => ElfEndian::ElfEndianBig,
            _ => ElfEndian::ElfEndianUnknown(value),
        }
    }
}

impl From<ElfEndian> for u8 {
    fn from(value: ElfEndian) -> u8 {
        match value {
            ElfEndian::ElfEndianLittle => 1,
            ElfEndian::ElfEndianBig => 2,
            ElfEndian::ElfEndianUnknown(value) => value,
        }
    }
}

impl ElfEndian {
    /// Create an `ElfEndian` from its ELF identification byte.
    pub fn new(value: u8) -> Self {
        Self::from(value)
    }

    /// Check whether this is a recognized ELF data encoding.
    pub fn validate(&self) -> Result<(), ElfErrors> {
        match self {
            ElfEndian::ElfEndianLittle | ElfEndian::ElfEndianBig => Ok(()),
            ElfEndian::ElfEndianUnknown(_) => Err(ElfErrors::InvalidElfEndian),
        }
    }

    /// Convert this data encoding into a readable string.
    pub fn as_str(&self) -> &'static str {
        match self {
            ElfEndian::ElfEndianLittle => "Little-endian",
            ElfEndian::ElfEndianBig => "Big-endian",
            ElfEndian::ElfEndianUnknown(_) => "Unknown data encoding",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_elf_endian_as_string() {
        assert_eq!(ElfEndian::ElfEndianLittle.as_str(), "Little-endian");
        assert_eq!(ElfEndian::ElfEndianBig.as_str(), "Big-endian");
        assert_eq!(
            ElfEndian::ElfEndianUnknown(3).as_str(),
            "Unknown data encoding"
        );
    }

    #[test]
    fn test_elf_endian_conversions() {
        for (value, endian) in [
            (0, ElfEndian::ElfEndianUnknown(0)),
            (1, ElfEndian::ElfEndianLittle),
            (2, ElfEndian::ElfEndianBig),
            (3, ElfEndian::ElfEndianUnknown(3)),
            (u8::MAX, ElfEndian::ElfEndianUnknown(u8::MAX)),
        ] {
            assert_eq!(ElfEndian::from(value), endian);
            assert_eq!(u8::from(endian), value);
            assert_eq!(ElfEndian::new(value), endian);
        }
    }

    #[test]
    fn test_elf_endian_validity() {
        assert_eq!(
            ElfEndian::ElfEndianUnknown(0).validate(),
            Err(ElfErrors::InvalidElfEndian)
        );
        assert_eq!(ElfEndian::ElfEndianLittle.validate(), Ok(()));
        assert_eq!(ElfEndian::ElfEndianBig.validate(), Ok(()));
        assert_eq!(
            ElfEndian::ElfEndianUnknown(3).validate(),
            Err(ElfErrors::InvalidElfEndian)
        );
    }
}
