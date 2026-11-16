// @author: Amrutansu Garanaik

use crate::elf_errors::ElfErrors;
use crate::types::st_type::{SymTabInfo, SymTabVisibility};
use crate::structures::elf32::ELF32_SYMBOL_TABLE_SIZE;

#[derive(Copy, Clone, Debug)]
/// Each symbol table entry
pub struct ElfSymbolTable32 {
    /// Offset of the symbol name string table.
    pub st_name : usize,
    /// symbol value
    pub st_value : usize,
    /// symbol size
    pub st_size : usize,
    /// Symbol binding and type
    pub st_info : SymTabInfo,
    /// Symbol visibility
    pub st_other : SymTabVisibility,
    /// Section index
    pub st_shndx : usize,
}

impl ElfSymbolTable32 {
    pub fn new() -> Self {
        Self {
            st_name : 0,
            st_value : 0,
            st_size : 0,
            st_info : SymTabInfo::from(&0_u8),
            st_other : SymTabVisibility::StvDefault,
            st_shndx : 0,
        }
    }
    pub fn validate(&self) -> Result<(), ElfErrors> {
        // Only two values are required here as rest are integers.
        // and always valid.
        if self.st_info.validate().is_err() {
            Err(ElfErrors::InvalidSymbolInfo)
        } else if self.st_other.validate().is_err() {
            Err(ElfErrors::InvalidSymTabVisibility)
        } else {
            Ok(())
        }
    }
}

impl From<&[u8;ELF32_SYMBOL_TABLE_SIZE]> for ElfSymbolTable32 {
    fn from (data: &[u8;ELF32_SYMBOL_TABLE_SIZE]) -> Self {
        Self {
            st_name : u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize,
            st_value : u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize,
            st_size : u32::from_le_bytes([data[8], data[9], data[10], data[11]]) as usize,
            st_info : SymTabInfo::from(&data[12]),
            st_other : SymTabVisibility::from(&data[13]),
            st_shndx : u16::from_le_bytes([data[14], data[15]]) as usize,
        }
    }
}

impl From<&ElfSymbolTable32> for [u8;ELF32_SYMBOL_TABLE_SIZE] {
    fn from (data: &ElfSymbolTable32) -> [u8;ELF32_SYMBOL_TABLE_SIZE]  {
        let mut result = [0; ELF32_SYMBOL_TABLE_SIZE];
        result[0..4].copy_from_slice(&(data.st_name as u32).to_le_bytes());
        result[4..8].copy_from_slice(&(data.st_value as u32).to_le_bytes());
        result[8..12].copy_from_slice(&(data.st_size as u32).to_le_bytes());
        result[12] = u8::from(&data.st_info);
        result[13] = data.st_other as u8;
        result[14..16].copy_from_slice(&(data.st_shndx as u16).to_le_bytes());

        return result;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::st_type::{SymTabInfoBind, SymTabInfoType};

    #[test]
    fn test_symbol_table_32_new_validate_and_round_trip() {
        let mut symbol = ElfSymbolTable32::new();
        assert_eq!(symbol.validate(), Ok(()));

        symbol.st_name = 0x1122_3344;
        symbol.st_info = SymTabInfo::new(0x1);
        symbol.st_other = SymTabVisibility::StvProtected;
        symbol.st_shndx = 0x8899;
        symbol.st_value = 0x5566_7788;
        symbol.st_size = 0x1122_3344;

        let bytes: [u8; ELF32_SYMBOL_TABLE_SIZE] = (&symbol).into();
        println! ("Bytes: {:?}", bytes);
        assert_eq!(
            &bytes,
            &[
                // st_name: 0x11223344
                0x44, 0x33, 0x22, 0x11,
                // st_value: 0x55667788
                0x88, 0x77, 0x66, 0x55,
                // st_size: 0x11223344
                0x44, 0x33, 0x22, 0x11,
                // st_info: 0x1
                0x1,
                // st_other: 0x3
                0x3,
                // st_shndx: 0x8899
                0x99, 0x88,
            ]
        );

        let parsed = ElfSymbolTable32::from(&bytes);
        assert_eq!(parsed.st_name, symbol.st_name);
        assert_eq!(parsed.st_info.st_type, SymTabInfoType::SttObject);
        assert_eq!(parsed.st_info.st_bind, SymTabInfoBind::StbLocal);
        assert_eq!(parsed.st_other, SymTabVisibility::StvProtected);
        assert_eq!(parsed.st_shndx, symbol.st_shndx);
        assert_eq!(parsed.st_value, symbol.st_value);
        assert_eq!(parsed.st_size, symbol.st_size);
        assert_eq!(parsed.validate(), Ok(()));
    }

    #[test]
    fn test_symbol_table_32_validation_errors() {
        let mut symbol = ElfSymbolTable32::new();
        symbol.st_info = SymTabInfo::new(0x30);
        assert_eq!(symbol.validate(), Err(ElfErrors::InvalidSymbolInfo));

        symbol.st_info = SymTabInfo::new(0);
        symbol.st_other = SymTabVisibility::StvInvalid;
        assert_eq!(
            symbol.validate(),
            Err(ElfErrors::InvalidSymTabVisibility)
        );
    }
}