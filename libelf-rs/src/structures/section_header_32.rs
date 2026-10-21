// @author: Amrutansu Garanaik

use crate::elf_errors::ElfErrors;
use crate::structures::elf32::ELF32_SECTION_HEADER_SIZE;
use crate::types::sh_flags::ElfShFlags;
use crate::types::sh_type::ElfShType;

/// ELF32 section header (`Shdr`).
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ElfSectionHeader32 {
    /// Section name string-table offset (`sh_name`).
    pub sh_name: u32,
    /// Section type (`sh_type`).
    pub sh_type: ElfShType,
    /// Section attributes (`sh_flags`).
    pub sh_flags: ElfShFlags,
    /// Section virtual address (`sh_addr`).
    pub sh_addr: u32,
    /// Section file offset (`sh_offset`).
    pub sh_offset: u32,
    /// Section size in bytes (`sh_size`).
    pub sh_size: u32,
    /// Link to another section (`sh_link`).
    pub sh_link: u32,
    /// Extra section information (`sh_info`).
    pub sh_info: u32,
    /// Section address alignment (`sh_addralign`).
    pub sh_addralign: u32,
    /// Size of each entry for fixed-size sections (`sh_entsize`).
    pub sh_entsize: u32,
}

impl ElfSectionHeader32 {
    /// Create a section header with the given type and zeroed remaining fields.
    pub fn new(sh_type: ElfShType) -> Self {
        Self {
            sh_name: 0,
            sh_type,
            sh_flags: ElfShFlags::default(),
            sh_addr: 0,
            sh_offset: 0,
            sh_size: 0,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 0,
            sh_entsize: 0,
        }
    }

    /// Validate the section type, flags, and address alignment.
    pub fn validate(&self) -> Result<(), ElfErrors> {
        self.sh_type.validate()?;
        self.sh_flags.validate()?;
        if self.sh_addralign > 1 && !self.sh_addralign.is_power_of_two() {
            return Err(ElfErrors::InvalidSectionHeaderAlignment);
        }
        Ok(())
    }
}

impl From<[u8; ELF32_SECTION_HEADER_SIZE as usize]> for ElfSectionHeader32 {
    /// Read an ELF32 section header from its little-endian byte representation.
    fn from(bytes: [u8; ELF32_SECTION_HEADER_SIZE as usize]) -> Self {
        Self {
            sh_name: u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            sh_type: ElfShType::from(u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]])),
            sh_flags: ElfShFlags::from(u32::from_le_bytes([
                bytes[8], bytes[9], bytes[10], bytes[11],
            ])),
            sh_addr: u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
            sh_offset: u32::from_le_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]),
            sh_size: u32::from_le_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]),
            sh_link: u32::from_le_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]),
            sh_info: u32::from_le_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]),
            sh_addralign: u32::from_le_bytes([bytes[32], bytes[33], bytes[34], bytes[35]]),
            sh_entsize: u32::from_le_bytes([bytes[36], bytes[37], bytes[38], bytes[39]]),
        }
    }
}

impl From<ElfSectionHeader32> for [u8; ELF32_SECTION_HEADER_SIZE as usize] {
    /// Convert an ELF32 section header to its little-endian byte representation.
    fn from(header: ElfSectionHeader32) -> Self {
        let mut bytes = [0; ELF32_SECTION_HEADER_SIZE as usize];
        bytes[0..4].copy_from_slice(&header.sh_name.to_le_bytes());
        bytes[4..8].copy_from_slice(&u32::from(header.sh_type).to_le_bytes());
        bytes[8..12].copy_from_slice(&(u64::from(header.sh_flags) as u32).to_le_bytes());
        bytes[12..16].copy_from_slice(&header.sh_addr.to_le_bytes());
        bytes[16..20].copy_from_slice(&header.sh_offset.to_le_bytes());
        bytes[20..24].copy_from_slice(&header.sh_size.to_le_bytes());
        bytes[24..28].copy_from_slice(&header.sh_link.to_le_bytes());
        bytes[28..32].copy_from_slice(&header.sh_info.to_le_bytes());
        bytes[32..36].copy_from_slice(&header.sh_addralign.to_le_bytes());
        bytes[36..40].copy_from_slice(&header.sh_entsize.to_le_bytes());
        bytes
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn header() -> ElfSectionHeader32 {
        ElfSectionHeader32::new(ElfShType::ElfShTypeProgBits)
    }

    #[test]
    fn test_section_header_32_new_validate_and_round_trip() {
        let mut header = header();
        assert_eq!(header.validate(), Ok(()));

        header.sh_name = 0x1122_3344;
        header.sh_flags = ElfShFlags::SHF_ALLOC | ElfShFlags::SHF_WRITE;
        header.sh_addr = 0x5566_7788;
        header.sh_offset = 0x99aa_bbcc;
        header.sh_size = 0x100;
        header.sh_link = 2;
        header.sh_info = 3;
        header.sh_addralign = 0x1000;
        header.sh_entsize = 16;

        let bytes: [u8; ELF32_SECTION_HEADER_SIZE as usize] = header.into();
        assert_eq!(
            &bytes,
            &[
                0x44, 0x33, 0x22, 0x11, 1, 0, 0, 0, 3, 0, 0, 0, 0x88, 0x77, 0x66, 0x55, 0xcc, 0xbb,
                0xaa, 0x99, 0, 1, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0x10, 0, 0, 16, 0, 0, 0,
            ]
        );
        let parsed = ElfSectionHeader32::from(bytes);
        assert_eq!(parsed, header);
        assert_eq!(parsed.validate(), Ok(()));
    }

    #[test]
    fn test_section_header_32_unknown_type_round_trips() {
        let mut bytes = [0; ELF32_SECTION_HEADER_SIZE as usize];
        bytes[4..8].copy_from_slice(&0x1234_u32.to_le_bytes());
        let parsed = ElfSectionHeader32::from(bytes);
        assert_eq!(parsed.sh_type, ElfShType::ElfShTypeUnknown(0x1234));
        let encoded: [u8; ELF32_SECTION_HEADER_SIZE as usize] = parsed.into();
        assert_eq!(&encoded[4..8], &0x1234_u32.to_le_bytes());
    }

    #[test]
    fn test_section_header_32_validation_errors() {
        let mut header = header();
        header.sh_type = ElfShType::ElfShTypeUnknown(0x1234);
        assert_eq!(header.validate(), Err(ElfErrors::InvalidSectionHeaderType));

        header.sh_type = ElfShType::ElfShTypeProgBits;
        header.sh_flags = ElfShFlags::new(0x10_0000_0000);
        assert_eq!(header.validate(), Err(ElfErrors::InvalidSectionHeaderFlags));

        header.sh_flags = ElfShFlags::default();
        header.sh_addralign = 3;
        assert_eq!(
            header.validate(),
            Err(ElfErrors::InvalidSectionHeaderAlignment)
        );
    }
}
