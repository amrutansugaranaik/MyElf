// @author: Amrutansu Garanaik

use crate::elf_errors::ElfErrors;
use crate::structures::elf_identifier::ElfIdentifier;
use crate::types::e_machine::ElfIsa;
use crate::types::e_type::ElfFileType;
use crate::types::ei_class::ElfClass;
use crate::types::ei_data::ElfEndian;

/// Size of the ELF64 header in bytes.
pub const ELF64_HEADER_SIZE: u16 = 64;
/// Size of an ELF64 program header in bytes.
pub const ELF64_PROGRAM_HEADER_SIZE: u16 = 56;
/// Size of an ELF64 section header in bytes.
pub const ELF64_SECTION_HEADER_SIZE: u16 = 64;

/// ELF64 file header.
#[derive(Clone, Debug, PartialEq)]
pub struct Elf64 {
    /// Common ELF identifier.
    pub identifier: ElfIdentifier,
    /// Object file type (`e_type`).
    pub file_type: ElfFileType,
    /// Target architecture (`e_machine`).
    pub machine: ElfIsa,
    /// ELF header version (`e_version`).
    pub version: u32,
    /// Virtual entry address (`e_entry`).
    pub entry: u64,
    /// Program header table file offset (`e_phoff`).
    pub phoff: u64,
    /// Section header table file offset (`e_shoff`).
    pub shoff: u64,
    /// Processor-specific flags (`e_flags`).
    pub flags: u32,
    /// ELF header size (`e_ehsize`).
    pub ehsize: u16,
    /// Program header entry size (`e_phentsize`).
    pub phentsize: u16,
    /// Program header count (`e_phnum`).
    pub phnum: u16,
    /// Section header entry size (`e_shentsize`).
    pub shentsize: u16,
    /// Section header count (`e_shnum`).
    pub shnum: u16,
    /// Section name string table index (`e_shstrndx`).
    pub shstrndx: u16,
}

impl Elf64 {
    /// Create an ELF64 header with standard entry sizes and zeroed offsets.
    pub fn new(identifier: ElfIdentifier, file_type: ElfFileType, machine: ElfIsa) -> Self {
        Self {
            identifier,
            file_type,
            machine,
            version: 1,
            entry: 0,
            phoff: 0,
            shoff: 0,
            flags: 0,
            ehsize: ELF64_HEADER_SIZE,
            phentsize: ELF64_PROGRAM_HEADER_SIZE,
            phnum: 0,
            shentsize: ELF64_SECTION_HEADER_SIZE,
            shnum: 0,
            shstrndx: 0,
        }
    }

    /// Validate the fields that identify a writable ELF64 file header.
    pub fn validate(&self) -> Result<(), ElfErrors> {
        self.identifier.validate()?;
        if self.identifier.class != ElfClass::ElfClass64 {
            return Err(ElfErrors::InvalidElfClass);
        }
        self.file_type.validate()?;
        self.machine.validate()?;
        if self.version != 1 {
            return Err(ElfErrors::InvalidElfVersion);
        }
        if self.ehsize != ELF64_HEADER_SIZE {
            return Err(ElfErrors::InvalidElfHeaderSize);
        }
        Ok(())
    }
}

impl From<[u8; ELF64_HEADER_SIZE as usize]> for Elf64 {
    /// Read an ELF64 header from its fixed-size byte representation.
    ///
    /// Unsupported data encodings are normalized to `Unknown(0xFF)`. Since
    /// their byte order is not defined, the remaining fields use little-endian
    /// decoding as a deterministic placeholder; validate before using them.
    fn from(bytes: [u8; ELF64_HEADER_SIZE as usize]) -> Self {
        let mut identifier_bytes = [0; 16];
        identifier_bytes.copy_from_slice(&bytes[..16]);
        if !matches!(identifier_bytes[5], 1 | 2) {
            identifier_bytes[5] = u8::MAX;
        }
        let identifier = ElfIdentifier::from(identifier_bytes);

        Self {
            identifier,
            file_type: ElfFileType::from(u16::from_le_bytes([bytes[16], bytes[17]])),
            machine: ElfIsa::from(u16::from_le_bytes([bytes[18], bytes[19]])),
            version: u32::from_le_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]),
            entry: u64::from_le_bytes([
                bytes[24], bytes[25], bytes[26], bytes[27], bytes[28], bytes[29], bytes[30],
                bytes[31],
            ]),
            phoff: u64::from_le_bytes([
                bytes[32], bytes[33], bytes[34], bytes[35], bytes[36], bytes[37], bytes[38],
                bytes[39],
            ]),
            shoff: u64::from_le_bytes([
                bytes[40], bytes[41], bytes[42], bytes[43], bytes[44], bytes[45], bytes[46],
                bytes[47],
            ]),
            flags: u32::from_le_bytes([bytes[48], bytes[49], bytes[50], bytes[51]]),
            ehsize: u16::from_le_bytes([bytes[52], bytes[53]]),
            phentsize: u16::from_le_bytes([bytes[54], bytes[55]]),
            phnum: u16::from_le_bytes([bytes[56], bytes[57]]),
            shentsize: u16::from_le_bytes([bytes[58], bytes[59]]),
            shnum: u16::from_le_bytes([bytes[60], bytes[61]]),
            shstrndx: u16::from_le_bytes([bytes[62], bytes[63]]),
        }
    }
}

impl From<Elf64> for [u8; ELF64_HEADER_SIZE as usize] {
    /// Convert an ELF64 header into its fixed-size byte representation.
    ///
    /// An unsupported data encoding is written as `0xFF`; the remaining fields
    /// use little-endian encoding as a deterministic placeholder.
    fn from(header: Elf64) -> Self {
        let mut bytes = [0; ELF64_HEADER_SIZE as usize];
        let data = header.identifier.data;
        bytes[..16].copy_from_slice(&<[u8; 16]>::from(header.identifier));
        if matches!(data, ElfEndian::ElfEndianUnknown(_)) {
            bytes[5] = u8::MAX;
        }
        bytes[16..18].copy_from_slice(&u16::from(header.file_type).to_le_bytes());
        bytes[18..20].copy_from_slice(&u16::from(header.machine).to_le_bytes());
        bytes[20..24].copy_from_slice(&header.version.to_le_bytes());
        bytes[24..32].copy_from_slice(&header.entry.to_le_bytes());
        bytes[32..40].copy_from_slice(&header.phoff.to_le_bytes());
        bytes[40..48].copy_from_slice(&header.shoff.to_le_bytes());
        bytes[48..52].copy_from_slice(&header.flags.to_le_bytes());
        bytes[52..54].copy_from_slice(&header.ehsize.to_le_bytes());
        bytes[54..56].copy_from_slice(&header.phentsize.to_le_bytes());
        bytes[56..58].copy_from_slice(&header.phnum.to_le_bytes());
        bytes[58..60].copy_from_slice(&header.shentsize.to_le_bytes());
        bytes[60..62].copy_from_slice(&header.shnum.to_le_bytes());
        bytes[62..64].copy_from_slice(&header.shstrndx.to_le_bytes());
        bytes
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::ei_data::ElfEndian;
    use crate::types::ei_osabi::ElfOsAbi;

    fn header(data: ElfEndian) -> Elf64 {
        Elf64::new(
            ElfIdentifier::new(ElfClass::ElfClass64, data, ElfOsAbi::Linux),
            ElfFileType::ElfTypeExec,
            ElfIsa::ElfIsaAArch64,
        )
    }

    #[test]
    fn test_elf64_new_and_validate() {
        let header = header(ElfEndian::ElfEndianLittle);
        assert_eq!(header.version, 1);
        assert_eq!(header.ehsize, 64);
        assert_eq!(header.phentsize, 56);
        assert_eq!(header.shentsize, 64);
        assert_eq!(header.validate(), Ok(()));
    }

    #[test]
    fn test_elf64_little_endian_round_trip() {
        let mut header = header(ElfEndian::ElfEndianLittle);
        header.entry = 0x1122_3344_5566_7788;
        header.phoff = 64;
        header.shoff = 0x200;
        header.flags = 0xaabb_ccdd;
        header.phnum = 2;
        header.shnum = 3;
        header.shstrndx = 1;

        let bytes: [u8; ELF64_HEADER_SIZE as usize] = header.clone().into();
        assert_eq!(
            &bytes[16..32],
            &[
                2, 0, 183, 0, 1, 0, 0, 0, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11
            ]
        );
        let parsed = Elf64::from(bytes);
        assert_eq!(parsed, header);
        assert_eq!(parsed.validate(), Ok(()));
    }

    #[test]
    fn test_elf64_big_endian_is_rejected_and_fields_are_decoded_little_endian() {
        let mut header = header(ElfEndian::ElfEndianBig);
        header.entry = 0x1122_3344_5566_7788;
        header.phoff = 64;
        header.shnum = 1;

        let bytes: [u8; ELF64_HEADER_SIZE as usize] = header.clone().into();
        assert_eq!(
            &bytes[16..32],
            &[
                2, 0, 183, 0, 1, 0, 0, 0, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11
            ]
        );
        let parsed = Elf64::from(bytes);
        assert_eq!(parsed, header);
        assert_eq!(parsed.validate(), Err(ElfErrors::InvalidElfEndian));
    }

    #[test]
    fn test_elf64_unknown_endian_uses_placeholder() {
        for raw_data in [0, 0x7f, u8::MAX] {
            let mut bytes: [u8; ELF64_HEADER_SIZE as usize] =
                header(ElfEndian::ElfEndianLittle).into();
            bytes[5] = raw_data;
            let parsed = Elf64::from(bytes);
            assert_eq!(parsed.identifier.data, ElfEndian::ElfEndianUnknown(u8::MAX));
            assert_eq!(parsed.version, 1);
            assert_eq!(parsed.validate(), Err(ElfErrors::InvalidElfEndian));

            let mut header = header(ElfEndian::ElfEndianLittle);
            header.identifier.data = ElfEndian::ElfEndianUnknown(raw_data);
            let bytes: [u8; ELF64_HEADER_SIZE as usize] = header.into();
            assert_eq!(bytes[5], u8::MAX);
            assert_eq!(
                u32::from_le_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]),
                1
            );
        }
    }

    #[test]
    fn test_elf64_validation_rejects_mismatched_fields() {
        let mut header = header(ElfEndian::ElfEndianLittle);
        header.identifier.class = ElfClass::ElfClass32;
        assert_eq!(header.validate(), Err(ElfErrors::InvalidElfClass));
        header.identifier.class = ElfClass::ElfClass64;
        header.file_type = ElfFileType::ElfTypeNone;
        assert_eq!(header.validate(), Err(ElfErrors::InvalidElfFileType));
        header.file_type = ElfFileType::ElfTypeExec;
        header.machine = ElfIsa::ElfIsaNone;
        assert_eq!(header.validate(), Err(ElfErrors::InvalidElfIsa));
        header.machine = ElfIsa::ElfIsaAArch64;
        header.version = 2;
        assert_eq!(header.validate(), Err(ElfErrors::InvalidElfVersion));
        header.version = 1;
        header.ehsize = 52;
        assert_eq!(header.validate(), Err(ElfErrors::InvalidElfHeaderSize));
    }
}
