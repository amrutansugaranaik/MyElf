// @author: Amrutansu Garanaik

use std::ops::{BitOr, BitOrAssign};

use crate::elf_errors::ElfErrors;

/// Section header attributes (`sh_flags`).
///
/// Section flags are a bitmask and may be combined. The raw value is stored
/// as `u64` to preserve ELF64 flags; ELF32 values can be widened losslessly.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ElfShFlags(u64);

impl ElfShFlags {
    /// Section data is writable during execution.
    pub const SHF_WRITE: Self = Self(0x1);
    /// Section occupies memory during execution.
    pub const SHF_ALLOC: Self = Self(0x2);
    /// Section contains executable instructions.
    pub const SHF_EXECINSTR: Self = Self(0x4);
    /// Section may be merged.
    pub const SHF_MERGE: Self = Self(0x10);
    /// Section contains null-terminated strings.
    pub const SHF_STRINGS: Self = Self(0x20);
    /// `sh_info` contains a section header index.
    pub const SHF_INFO_LINK: Self = Self(0x40);
    /// Preserve ordering constraints during linking.
    pub const SHF_LINK_ORDER: Self = Self(0x80);
    /// Requires OS-specific handling.
    pub const SHF_OS_NONCONFORMING: Self = Self(0x100);
    /// Section is a member of a section group.
    pub const SHF_GROUP: Self = Self(0x200);
    /// Section contains thread-local storage.
    pub const SHF_TLS: Self = Self(0x400);
    /// Section contents are compressed.
    pub const SHF_COMPRESSED: Self = Self(0x800);
    /// Retain this section during garbage collection.
    pub const SHF_GNU_RETAIN: Self = Self(0x0020_0000);
    /// Exclude this section from linking when appropriate.
    pub const SHF_EXCLUDE: Self = Self(0x8000_0000);
    /// OS-specific flag mask.
    pub const SHF_MASKOS: u64 = 0x0ff0_0000;
    /// Processor-specific flag mask.
    pub const SHF_MASKPROC: u64 = 0xf000_0000;

    const STANDARD_MASK: u64 =
        0x1 | 0x2 | 0x4 | 0x10 | 0x20 | 0x40 | 0x80 | 0x100 | 0x200 | 0x400 | 0x800;
    const VALID_MASK: u64 = Self::STANDARD_MASK | Self::SHF_MASKOS | Self::SHF_MASKPROC;

    /// Create flags from their raw ELF bit field.
    pub const fn new(bits: u64) -> Self {
        Self(bits)
    }

    /// Return the raw ELF bit field.
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Check whether all bits in `other` are set.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Return the flags as an ELF32 word when the value fits.
    pub const fn to_u32(self) -> Option<u32> {
        if self.0 <= u32::MAX as u64 {
            Some(self.0 as u32)
        } else {
            None
        }
    }

    /// Check that no reserved flag bits are set.
    pub fn validate(&self) -> Result<(), ElfErrors> {
        if self.0 & !Self::VALID_MASK == 0 {
            Ok(())
        } else {
            Err(ElfErrors::InvalidSectionHeaderFlags)
        }
    }

    /// Return a readable list of the active flag names.
    pub fn as_str(&self) -> String {
        let mut names = Vec::new();
        for (flag, name) in [
            (Self::SHF_WRITE, "WRITE"),
            (Self::SHF_ALLOC, "ALLOC"),
            (Self::SHF_EXECINSTR, "EXECINSTR"),
            (Self::SHF_MERGE, "MERGE"),
            (Self::SHF_STRINGS, "STRINGS"),
            (Self::SHF_INFO_LINK, "INFO_LINK"),
            (Self::SHF_LINK_ORDER, "LINK_ORDER"),
            (Self::SHF_OS_NONCONFORMING, "OS_NONCONFORMING"),
            (Self::SHF_GROUP, "GROUP"),
            (Self::SHF_TLS, "TLS"),
            (Self::SHF_COMPRESSED, "COMPRESSED"),
            (Self::SHF_GNU_RETAIN, "GNU_RETAIN"),
            (Self::SHF_EXCLUDE, "EXCLUDE"),
        ] {
            if self.contains(flag) {
                names.push(name);
            }
        }
        if self.0 & Self::SHF_MASKOS & !Self::SHF_GNU_RETAIN.0 != 0 {
            names.push("OS-specific");
        }
        if self.0 & Self::SHF_MASKPROC & !Self::SHF_EXCLUDE.0 != 0 {
            names.push("Processor-specific");
        }
        if names.is_empty() {
            names.push("None");
        }
        names.join("|")
    }
}

impl From<u64> for ElfShFlags {
    fn from(bits: u64) -> Self {
        Self(bits)
    }
}

impl From<u32> for ElfShFlags {
    fn from(bits: u32) -> Self {
        Self(u64::from(bits))
    }
}

impl From<ElfShFlags> for u64 {
    fn from(flags: ElfShFlags) -> Self {
        flags.0
    }
}

impl BitOr for ElfShFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for ElfShFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const KNOWN: &[(u64, ElfShFlags, &str)] = &[
        (0, ElfShFlags::new(0), "None"),
        (0x1, ElfShFlags::SHF_WRITE, "WRITE"),
        (0x2, ElfShFlags::SHF_ALLOC, "ALLOC"),
        (0x4, ElfShFlags::SHF_EXECINSTR, "EXECINSTR"),
        (0x10, ElfShFlags::SHF_MERGE, "MERGE"),
        (0x20, ElfShFlags::SHF_STRINGS, "STRINGS"),
        (0x40, ElfShFlags::SHF_INFO_LINK, "INFO_LINK"),
        (0x80, ElfShFlags::SHF_LINK_ORDER, "LINK_ORDER"),
        (0x100, ElfShFlags::SHF_OS_NONCONFORMING, "OS_NONCONFORMING"),
        (0x200, ElfShFlags::SHF_GROUP, "GROUP"),
        (0x400, ElfShFlags::SHF_TLS, "TLS"),
        (0x800, ElfShFlags::SHF_COMPRESSED, "COMPRESSED"),
        (0x0020_0000, ElfShFlags::SHF_GNU_RETAIN, "GNU_RETAIN"),
        (0x8000_0000, ElfShFlags::SHF_EXCLUDE, "EXCLUDE"),
        (0x6, ElfShFlags::new(0x6), "ALLOC|EXECINSTR"),
        (0x0010_0000, ElfShFlags::new(0x0010_0000), "OS-specific"),
        (
            0x1000_0000,
            ElfShFlags::new(0x1000_0000),
            "Processor-specific",
        ),
    ];

    #[test]
    fn test_section_header_flags_conversions_and_strings() {
        for (raw, flags, name) in KNOWN {
            assert_eq!(ElfShFlags::from(*raw), *flags);
            assert_eq!(u64::from(*flags), *raw);
            assert_eq!(ElfShFlags::new(*raw), *flags);
            assert_eq!(flags.as_str(), *name);
        }
        assert_eq!(ElfShFlags::from(0x6_u32), ElfShFlags::new(0x6));
        assert_eq!(ElfShFlags::new(0x6).to_u32(), Some(0x6));

        let flags = ElfShFlags::SHF_ALLOC | ElfShFlags::SHF_EXECINSTR;
        assert_eq!(flags.bits(), 0x6);
        assert!(flags.contains(ElfShFlags::SHF_ALLOC));

        let mut flags = ElfShFlags::SHF_WRITE;
        flags |= ElfShFlags::SHF_ALLOC;
        assert_eq!(flags, ElfShFlags::new(0x3));
    }

    #[test]
    fn test_section_header_flags_validity() {
        for (raw, flags, _) in KNOWN {
            assert_eq!(flags.validate(), Ok(()), "raw flags {raw:#x}");
        }
        assert_eq!(
            ElfShFlags::new(0x10_0000_0000).validate(),
            Err(ElfErrors::InvalidSectionHeaderFlags)
        );
        assert_eq!(ElfShFlags::new(0x1_0000_0000).to_u32(), None);
    }
}
