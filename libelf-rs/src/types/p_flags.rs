// @author: Amrutansu Garanaik

use std::ops::{BitOr, BitOrAssign};

use crate::elf_errors::ElfErrors;

/// Program header permissions and extension flags (`p_flags`).
///
/// Program flags are a bitmask and may be combined. Unknown OS- or
/// processor-specific bits are retained when converting from the raw value.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ElfPhFlags(u32);

impl ElfPhFlags {
    /// Executable segment.
    pub const PF_X: Self = Self(0x1);
    /// Writable segment.
    pub const PF_W: Self = Self(0x2);
    /// Readable segment.
    pub const PF_R: Self = Self(0x4);
    /// OS-specific flag mask.
    pub const PF_MASKOS: u32 = 0x0ff0_0000;
    /// Processor-specific flag mask.
    pub const PF_MASKPROC: u32 = 0xf000_0000;

    const VALID_MASK: u32 = 0x0ff0_0007 | 0xf000_0000;

    /// Create flags from their raw ELF bit field.
    pub const fn new(bits: u32) -> Self {
        Self(bits)
    }

    /// Return the raw ELF bit field.
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Check whether all bits in `other` are set.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Check that no reserved flag bits are set.
    pub fn validate(&self) -> Result<(), ElfErrors> {
        if self.0 & !Self::VALID_MASK == 0 {
            Ok(())
        } else {
            Err(ElfErrors::InvalidProgramHeaderFlags)
        }
    }

    /// Return a readable list of the active flag names.
    pub fn as_str(&self) -> String {
        let mut names = Vec::new();
        if self.contains(Self::PF_R) {
            names.push("R");
        }
        if self.contains(Self::PF_W) {
            names.push("W");
        }
        if self.contains(Self::PF_X) {
            names.push("X");
        }
        if self.0 & Self::PF_MASKOS != 0 {
            names.push("OS-specific");
        }
        if self.0 & Self::PF_MASKPROC != 0 {
            names.push("Processor-specific");
        }
        if names.is_empty() {
            names.push("None");
        }
        names.join("|")
    }
}

impl From<u32> for ElfPhFlags {
    fn from(bits: u32) -> Self {
        Self(bits)
    }
}

impl From<ElfPhFlags> for u32 {
    fn from(flags: ElfPhFlags) -> Self {
        flags.0
    }
}

impl BitOr for ElfPhFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for ElfPhFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const KNOWN: &[(u32, ElfPhFlags, &str)] = &[
        (0, ElfPhFlags::new(0), "None"),
        (1, ElfPhFlags::PF_X, "X"),
        (2, ElfPhFlags::PF_W, "W"),
        (4, ElfPhFlags::PF_R, "R"),
        (7, ElfPhFlags::new(7), "R|W|X"),
        (0x0010_0000, ElfPhFlags::new(0x0010_0000), "OS-specific"),
        (
            0x1000_0000,
            ElfPhFlags::new(0x1000_0000),
            "Processor-specific",
        ),
    ];

    #[test]
    fn test_program_header_flags_conversions_and_strings() {
        for (raw, flags, name) in KNOWN {
            assert_eq!(ElfPhFlags::from(*raw), *flags);
            assert_eq!(u32::from(*flags), *raw);
            assert_eq!(ElfPhFlags::new(*raw), *flags);
            assert_eq!(flags.as_str(), *name);
        }
    }

    #[test]
    fn test_program_header_flags_operations_and_validity() {
        let flags = ElfPhFlags::PF_R | ElfPhFlags::PF_W | ElfPhFlags::PF_X;
        assert_eq!(flags.bits(), 0x7);
        assert!(flags.contains(ElfPhFlags::PF_R | ElfPhFlags::PF_W));
        assert_eq!(flags.validate(), Ok(()));

        let mut flags = ElfPhFlags::PF_R;
        flags |= ElfPhFlags::PF_X;
        assert_eq!(flags, ElfPhFlags::PF_R | ElfPhFlags::PF_X);

        assert_eq!(
            ElfPhFlags::from(0x8).validate(),
            Err(ElfErrors::InvalidProgramHeaderFlags)
        );
        assert_eq!(
            ElfPhFlags::from(u32::MAX).validate(),
            Err(ElfErrors::InvalidProgramHeaderFlags)
        );
    }
}
