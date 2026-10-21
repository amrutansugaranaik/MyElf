// @author: Amrutansu Garanaik

/// This file contains the elf common identifier which is present in
/// both 32-bit and 64-bit elf files. It is used to identify the file as an elf file.
use crate::elf_errors::ElfErrors;
use crate::types::ei_class::ElfClass;
use crate::types::ei_data::ElfEndian;
use crate::types::ei_osabi::ElfOsAbi;
use crate::types::ei_version::ElfVersion;

/// Common ELF identifier structure, present in both 32-bit and 64-bit ELF files.
#[derive(Clone, Debug, PartialEq)]
pub struct ElfIdentifier {
    /// Magic number, always `0x7F 'E' 'L' 'F'`.
    pub magic: [u8; 4],
    /// ELF class, indicating 32-bit or 64-bit format.
    pub class: ElfClass,
    /// Data encoding, indicating endianness.
    pub data: ElfEndian,
    /// ELF version, always `1` for the original version.
    pub version: ElfVersion,
    /// Operating system and ABI identification.
    pub os_abi: ElfOsAbi,
    /// ABI version (required in very niche cases). Usually ignored.
    pub abi_version: u8,
    /// Padding.
    pub pad: [u8; 7],
}

impl From<[u8; 16]> for ElfIdentifier {
    /// Create an `ElfIdentifier` from an array of 16 bytes, as found in the ELF header.
    /// It is assumed that the data is correct in the slice, and the caller is responsible for validate().
    fn from(value: [u8; 16]) -> Self {
        Self {
            magic: [value[0], value[1], value[2], value[3]],
            class: ElfClass::from(value[4]),
            data: ElfEndian::from(value[5]),
            version: ElfVersion::from(value[6]),
            os_abi: ElfOsAbi::from(value[7]),
            abi_version: value[8],
            pad: [
                value[9], value[10], value[11], value[12], value[13], value[14], value[15],
            ],
        }
    }
}

impl From<ElfIdentifier> for [u8; 16] {
    /// Convert an ElfIdentifier into an array of 16 bytes.
    /// This can be used to write the identifier back to the header or file.
    fn from(identifier: ElfIdentifier) -> [u8; 16] {
        let mut result = [0u8; 16];
        result[0] = identifier.magic[0];
        result[1] = identifier.magic[1];
        result[2] = identifier.magic[2];
        result[3] = identifier.magic[3];
        result[4] = identifier.class.into();
        result[5] = identifier.data.into();
        result[6] = identifier.version.into();
        result[7] = identifier.os_abi.into();
        result[8] = identifier.abi_version;
        result[9] = identifier.pad[0];
        result[10] = identifier.pad[1];
        result[11] = identifier.pad[2];
        result[12] = identifier.pad[3];
        result[13] = identifier.pad[4];
        result[14] = identifier.pad[5];
        result[15] = identifier.pad[6];
        result
    }
}

impl ElfIdentifier {
    /// Create a new identifier with given class (32-bit or 64-bit), data endianess and OS-ABI.
    /// The version is always set to Current and abi_version and paddings are set to 0.
    /// The ELF magic is also hard-coded.
    pub fn new(class: ElfClass, data: ElfEndian, os_abi: ElfOsAbi) -> Self {
        Self {
            magic: [0x7F, b'E', b'L', b'F'],
            class,
            data,
            version: ElfVersion::ElfVersionCurrent,
            os_abi,
            abi_version: 0,
            pad: [0; 7],
        }
    }
    /// Validate the ELF identifier fields.
    pub fn validate(&self) -> Result<(), ElfErrors> {
        if self.magic != [0x7F, b'E', b'L', b'F'] {
            return Err(ElfErrors::InvalidMagicNumber);
        }
        // Shadow the validate_common variable to store the result and validate.
        // Validate all the fields and if any error is found, return that error.
        let validate_common = self.class.validate();
        if let Err(e) = validate_common {
            return Err(e);
        }
        let validate_common = self.data.validate();
        if let Err(e) = validate_common {
            return Err(e);
        }
        let validate_common = self.version.validate();
        if let Err(e) = validate_common {
            return Err(e);
        }
        let validate_common = self.os_abi.validate();
        if let Err(e) = validate_common {
            return Err(e);
        }

        // Ignore ABI version and padding as they are not required.

        // Reaching here means no error was found.
        Ok(())
    }

    /* Setter APIs for class, data and osabi. The version, abi_version and padding
     * do not have any setters as their values are fixed.
     */
    /// Update the class of the ELF identifier.
    pub fn set_class(&mut self, class: ElfClass) {
        self.class = class;
    }
    /// Update the endianess of the ELF identifier.
    pub fn set_data(&mut self, data: ElfEndian) {
        self.data = data;
    }
    /// Update the OS-ABI of the ELF identifier.
    pub fn set_osabi(&mut self, os_abi: ElfOsAbi) {
        self.os_abi = os_abi;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Identifier array for ELF64, LE for Linux.
    const G_IDENTIFIER_BYTES: [u8; 16] =
        [0x7F, b'E', b'L', b'F', 2, 1, 1, 3, 0, 0, 0, 0, 0, 0, 0, 0];

    #[test]
    // A single test for both new() and setters. This is because after new(), the setters
    // can update them and we can check the corresponding values.
    fn test_new_and_setters() {
        let mut identifier = ElfIdentifier::new(
            ElfClass::ElfClass64,
            ElfEndian::ElfEndianLittle,
            ElfOsAbi::Linux,
        );

        // Check default values after new().
        assert_eq!(identifier.magic, [0x7F, b'E', b'L', b'F']);
        assert_eq!(identifier.class, ElfClass::ElfClass64);
        assert_eq!(identifier.data, ElfEndian::ElfEndianLittle);
        assert_eq!(identifier.version, ElfVersion::ElfVersionCurrent);
        assert_eq!(identifier.os_abi, ElfOsAbi::Linux);
        assert_eq!(identifier.abi_version, 0);
        assert_eq!(identifier.pad, [0; 7]);

        // Update class. The remaining fields should not change.
        identifier.set_class(ElfClass::ElfClass32);
        assert_eq!(identifier.magic, [0x7F, b'E', b'L', b'F']);
        assert_ne!(identifier.class, ElfClass::ElfClass64);
        assert_eq!(identifier.class, ElfClass::ElfClass32);
        assert_eq!(identifier.data, ElfEndian::ElfEndianLittle);
        assert_eq!(identifier.version, ElfVersion::ElfVersionCurrent);
        assert_eq!(identifier.os_abi, ElfOsAbi::Linux);
        assert_eq!(identifier.abi_version, 0);
        assert_eq!(identifier.pad, [0; 7]);

        // Class is now 32-bit. Update endianess.
        identifier.set_data(ElfEndian::ElfEndianBig);
        assert_eq!(identifier.magic, [0x7F, b'E', b'L', b'F']);
        assert_eq!(identifier.class, ElfClass::ElfClass32);
        assert_ne!(identifier.data, ElfEndian::ElfEndianLittle);
        assert_eq!(identifier.data, ElfEndian::ElfEndianBig);
        assert_eq!(identifier.version, ElfVersion::ElfVersionCurrent);
        assert_eq!(identifier.os_abi, ElfOsAbi::Linux);
        assert_eq!(identifier.abi_version, 0);
        assert_eq!(identifier.pad, [0; 7]);

        // Class is now 32-bit and endianess is LE. Update ABI..
        identifier.set_osabi(ElfOsAbi::Solaris);
        assert_eq!(identifier.magic, [0x7F, b'E', b'L', b'F']);
        assert_eq!(identifier.class, ElfClass::ElfClass32);
        assert_eq!(identifier.data, ElfEndian::ElfEndianBig);
        assert_eq!(identifier.version, ElfVersion::ElfVersionCurrent);
        assert_ne!(identifier.os_abi, ElfOsAbi::Linux);
        assert_eq!(identifier.os_abi, ElfOsAbi::Solaris);
        assert_eq!(identifier.abi_version, 0);
        assert_eq!(identifier.pad, [0; 7]);
    }

    #[test]
    fn test_from_array() {
        let identifier = ElfIdentifier::from(G_IDENTIFIER_BYTES);
        assert_eq!(identifier.magic, [0x7F, b'E', b'L', b'F']);
        assert_eq!(identifier.class, ElfClass::ElfClass64);
        assert_eq!(identifier.data, ElfEndian::ElfEndianLittle);
        assert_eq!(identifier.version, ElfVersion::ElfVersionCurrent);
        assert_eq!(identifier.os_abi, ElfOsAbi::Linux);
        assert_eq!(identifier.abi_version, 0);
        assert_eq!(identifier.pad, [0; 7]);
    }

    #[test]
    // Convert data into array. Just create an identifier from new (64-bit LE Linux) and convert to array.
    // Compare it with the global array which is the expected array.
    fn test_into_array() {
        let identifier = ElfIdentifier::new(
            ElfClass::ElfClass64,
            ElfEndian::ElfEndianLittle,
            ElfOsAbi::Linux,
        );

        let array: [u8; 16] = identifier.into();
        assert_eq!(array, G_IDENTIFIER_BYTES);
    }

    #[test]
    fn test_validate() {
        // This array will be modified into variaour test values and then the identifier
        // will be created from that. This is because new() and setters don't allow individual
        // field setting for all fields.
        let mut identifier_array = [0x7F, b'E', b'L', b'F', 2, 1, 1, 3, 0, 0, 0, 0, 0, 0, 0, 0];

        // test1 : Check a valid identifier. This should pass.
        let identifier = ElfIdentifier::from(identifier_array);
        assert_eq!(identifier.validate(), Ok(()));

        // test2: Update marker to invalid value.
        identifier_array[0] = 0xFF;
        let identifier = ElfIdentifier::from(identifier_array);
        assert_eq!(identifier.validate(), Err(ElfErrors::InvalidMagicNumber));
        identifier_array[0] = 0x7F; // reset to valid value for the next test.

        // test3: Update class to invalid value.
        identifier_array[4] = 0xFF;
        let identifier = ElfIdentifier::from(identifier_array);
        assert_eq!(identifier.validate(), Err(ElfErrors::InvalidElfClass));
        identifier_array[4] = 2; // reset to valid value for the next test.

        // test4: Update Endianess to invalid value.
        identifier_array[5] = 0xFF;
        let identifier = ElfIdentifier::from(identifier_array);
        assert_eq!(identifier.validate(), Err(ElfErrors::InvalidElfEndian));
        identifier_array[5] = 1; // reset to valid value for the next test.

        // test5: Update version to invalid value.
        identifier_array[6] = 0xFF;
        let identifier = ElfIdentifier::from(identifier_array);
        assert_eq!(identifier.validate(), Err(ElfErrors::InvalidElfVersion));
        identifier_array[6] = 1; // reset to valid value for the next test.

        // test6: Update ABI to invalid value.
        identifier_array[7] = 0xFF;
        let identifier = ElfIdentifier::from(identifier_array);
        assert_eq!(identifier.validate(), Err(ElfErrors::InvalidOsAbi));
        identifier_array[7] = 3; // reset to valid value for the next test.
    }
}
