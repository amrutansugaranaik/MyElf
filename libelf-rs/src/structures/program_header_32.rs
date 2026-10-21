// @author: Amrutansu Garanaik

use crate::elf_errors::ElfErrors;
use crate::structures::elf32::ELF32_PROGRAM_HEADER_SIZE;
use crate::types::p_flags::ElfPhFlags;
use crate::types::p_type::ElfPhType;

/// ELF32 program header (`Phdr`).
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ElfProgramHeader32 {
    /// Segment type (`p_type`).
    pub p_type: ElfPhType,
    /// Segment file offset (`p_offset`).
    pub p_offset: u32,
    /// Segment virtual address (`p_vaddr`).
    pub p_vaddr: u32,
    /// Segment physical address (`p_paddr`).
    pub p_paddr: u32,
    /// Number of bytes in the file image (`p_filesz`).
    pub p_filesz: u32,
    /// Number of bytes in the memory image (`p_memsz`).
    pub p_memsz: u32,
    /// Segment flags (`p_flags`).
    pub p_flags: ElfPhFlags,
    /// Segment alignment (`p_align`).
    pub p_align: u32,
}

impl ElfProgramHeader32 {
    /// Create a program header with the given type and zeroed remaining fields.
    pub fn new(p_type: ElfPhType) -> Self {
        Self {
            p_type,
            p_offset: 0,
            p_vaddr: 0,
            p_paddr: 0,
            p_filesz: 0,
            p_memsz: 0,
            p_flags: ElfPhFlags::default(),
            p_align: 0,
        }
    }

    /// Validate the type, flags, loadable-segment sizes, and alignment.
    pub fn validate(&self) -> Result<(), ElfErrors> {
        self.p_type.validate()?;
        self.p_flags.validate()?;
        if self.p_type == ElfPhType::ElfPhTypeLoad && self.p_filesz > self.p_memsz {
            return Err(ElfErrors::InvalidProgramHeaderSizes);
        }
        if self.p_align > 1 && !self.p_align.is_power_of_two() {
            return Err(ElfErrors::InvalidProgramHeaderAlignment);
        }
        Ok(())
    }
}

impl From<[u8; ELF32_PROGRAM_HEADER_SIZE as usize]> for ElfProgramHeader32 {
    /// Read an ELF32 program header from its little-endian byte representation.
    fn from(bytes: [u8; ELF32_PROGRAM_HEADER_SIZE as usize]) -> Self {
        Self {
            p_type: ElfPhType::from(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])),
            p_offset: u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            p_vaddr: u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            p_paddr: u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
            p_filesz: u32::from_le_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]),
            p_memsz: u32::from_le_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]),
            p_flags: ElfPhFlags::from(u32::from_le_bytes([
                bytes[24], bytes[25], bytes[26], bytes[27],
            ])),
            p_align: u32::from_le_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]),
        }
    }
}

impl From<ElfProgramHeader32> for [u8; ELF32_PROGRAM_HEADER_SIZE as usize] {
    /// Convert an ELF32 program header to its little-endian byte representation.
    fn from(header: ElfProgramHeader32) -> Self {
        let mut bytes = [0; ELF32_PROGRAM_HEADER_SIZE as usize];
        bytes[0..4].copy_from_slice(&u32::from(header.p_type).to_le_bytes());
        bytes[4..8].copy_from_slice(&header.p_offset.to_le_bytes());
        bytes[8..12].copy_from_slice(&header.p_vaddr.to_le_bytes());
        bytes[12..16].copy_from_slice(&header.p_paddr.to_le_bytes());
        bytes[16..20].copy_from_slice(&header.p_filesz.to_le_bytes());
        bytes[20..24].copy_from_slice(&header.p_memsz.to_le_bytes());
        bytes[24..28].copy_from_slice(&u32::from(header.p_flags).to_le_bytes());
        bytes[28..32].copy_from_slice(&header.p_align.to_le_bytes());
        bytes
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn header() -> ElfProgramHeader32 {
        ElfProgramHeader32::new(ElfPhType::ElfPhTypeLoad)
    }

    #[test]
    fn test_program_header_32_new_validate_and_round_trip() {
        let mut header = header();
        assert_eq!(header.validate(), Ok(()));

        header.p_offset = 0x1122_3344;
        header.p_vaddr = 0x5566_7788;
        header.p_paddr = 0x99aa_bbcc;
        header.p_filesz = 0x100;
        header.p_memsz = 0x200;
        header.p_flags = ElfPhFlags::PF_R | ElfPhFlags::PF_X;
        header.p_align = 0x1000;

        let bytes: [u8; ELF32_PROGRAM_HEADER_SIZE as usize] = header.into();
        assert_eq!(
            &bytes,
            &[
                1, 0, 0, 0, 0x44, 0x33, 0x22, 0x11, 0x88, 0x77, 0x66, 0x55, 0xcc, 0xbb, 0xaa, 0x99,
                0, 1, 0, 0, 0, 2, 0, 0, 5, 0, 0, 0, 0, 0x10, 0, 0,
            ]
        );
        let parsed = ElfProgramHeader32::from(bytes);
        assert_eq!(parsed, header);
        assert_eq!(parsed.validate(), Ok(()));
    }

    #[test]
    fn test_program_header_32_unknown_type_round_trips() {
        let mut bytes = [0; ELF32_PROGRAM_HEADER_SIZE as usize];
        bytes[0..4].copy_from_slice(&0x1234_u32.to_le_bytes());
        let parsed = ElfProgramHeader32::from(bytes);
        assert_eq!(parsed.p_type, ElfPhType::ElfPhTypeUnknown(0x1234));
        let encoded: [u8; ELF32_PROGRAM_HEADER_SIZE as usize] = parsed.into();
        assert_eq!(&encoded[0..4], &0x1234_u32.to_le_bytes());
    }

    #[test]
    fn test_program_header_32_validation_errors() {
        let mut header = header();
        header.p_type = ElfPhType::ElfPhTypeUnknown(0x1234);
        assert_eq!(header.validate(), Err(ElfErrors::InvalidProgramHeaderType));

        header.p_type = ElfPhType::ElfPhTypeLoad;
        header.p_flags = ElfPhFlags::from(0x8);
        assert_eq!(header.validate(), Err(ElfErrors::InvalidProgramHeaderFlags));

        header.p_flags = ElfPhFlags::PF_R;
        header.p_filesz = 2;
        header.p_memsz = 1;
        assert_eq!(header.validate(), Err(ElfErrors::InvalidProgramHeaderSizes));

        header.p_filesz = 1;
        header.p_align = 3;
        assert_eq!(
            header.validate(),
            Err(ElfErrors::InvalidProgramHeaderAlignment)
        );
    }
}
