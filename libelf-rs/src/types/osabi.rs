// @author: Amrutansu Garanaik

use crate::elf_errors::ElfErrors;

/// ElfOsAbi enum
///
/// Each entry indicates one of the possible ABIs supported,
/// OsAbiUnknown(u8) is used to indicate any unsupported/invalid ABI.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum ElfOsAbi {
    /// Unix System-V
    SystemV = 0x00,
    /// HP-UX
    HpUx = 0x01,
    /// NetBSD ABI
    NetBsd = 0x02,
    /// Linux ABI
    Linux = 0x03,
    /// GNU Hurd Kernel ABI
    GnuHurd = 0x04,
    /// Solaris ABI
    Solaris = 0x06,
    /// AIX (Monterey)
    Aix = 0x07,
    /// IRIX ABI
    Irix = 0x08,
    /// FreeBSD
    FreeBsd = 0x09,
    /// Tru64
    Tru64 = 0x0A,
    /// Novell Modesto
    NovellModesto = 0x0B,
    /// Open BSD
    OpenBsd = 0x0C,
    /// Open VMS
    OpenVms = 0x0D,
    /// Non-stop kernel
    NonStop = 0x0E,
    /// Aros
    Aros = 0x0F,
    /// Fenix OS
    FenixOs = 0x10,
    /// Nuxi CloudABI
    Nuxi = 0x11,
    /// Stratus Technologies OpenVOS
    OpenVos = 0x12,
    /// Unknown or Unsupported type
    OsAbiUnknown(u8),
}

impl From<u8> for ElfOsAbi {
    /// Converts a raw `u8` ABI value into an [`ElfOsAbi`].
    fn from(value: u8) -> Self {
        match value {
            0x00 => ElfOsAbi::SystemV,
            0x01 => ElfOsAbi::HpUx,
            0x02 => ElfOsAbi::NetBsd,
            0x03 => ElfOsAbi::Linux,
            0x04 => ElfOsAbi::GnuHurd,
            0x06 => ElfOsAbi::Solaris,
            0x07 => ElfOsAbi::Aix,
            0x08 => ElfOsAbi::Irix,
            0x09 => ElfOsAbi::FreeBsd,
            0x0A => ElfOsAbi::Tru64,
            0x0B => ElfOsAbi::NovellModesto,
            0x0C => ElfOsAbi::OpenBsd,
            0x0D => ElfOsAbi::OpenVms,
            0x0E => ElfOsAbi::NonStop,
            0x0F => ElfOsAbi::Aros,
            0x10 => ElfOsAbi::FenixOs,
            0x11 => ElfOsAbi::Nuxi,
            0x12 => ElfOsAbi::OpenVos,
            _ => ElfOsAbi::OsAbiUnknown(value),
        }
    }
}

impl From<ElfOsAbi> for u8 {
    /// Converts an `ElfOsAbi` enum into raw u8 value.
    fn from(value: ElfOsAbi) -> u8 {
        match value {
            ElfOsAbi::SystemV => 0x00,
            ElfOsAbi::HpUx => 0x01,
            ElfOsAbi::NetBsd => 0x02,
            ElfOsAbi::Linux => 0x03,
            ElfOsAbi::GnuHurd => 0x04,
            ElfOsAbi::Solaris => 0x06,
            ElfOsAbi::Aix => 0x07,
            ElfOsAbi::Irix => 0x08,
            ElfOsAbi::FreeBsd => 0x09,
            ElfOsAbi::Tru64 => 0x0A,
            ElfOsAbi::NovellModesto => 0x0B,
            ElfOsAbi::OpenBsd => 0x0C,
            ElfOsAbi::OpenVms => 0x0D,
            ElfOsAbi::NonStop => 0x0E,
            ElfOsAbi::Aros => 0x0F,
            ElfOsAbi::FenixOs => 0x10,
            ElfOsAbi::Nuxi => 0x11,
            ElfOsAbi::OpenVos => 0x12,
            ElfOsAbi::OsAbiUnknown(val) => val,
        }
    }
}

impl ElfOsAbi {
    /// Create a new instance of `ElfOsAbi` from a raw u8 value.
    pub fn new(value: u8) -> Self {
        return ElfOsAbi::from(value);
    }

    /// Checks if the value is a valid OS ABI type.
    pub fn validate(&self) -> Result<(), ElfErrors> {
        match self {
            ElfOsAbi::SystemV
            | ElfOsAbi::HpUx
            | ElfOsAbi::NetBsd
            | ElfOsAbi::Linux
            | ElfOsAbi::GnuHurd
            | ElfOsAbi::Solaris
            | ElfOsAbi::Aix
            | ElfOsAbi::Irix
            | ElfOsAbi::FreeBsd
            | ElfOsAbi::Tru64
            | ElfOsAbi::NovellModesto
            | ElfOsAbi::OpenBsd
            | ElfOsAbi::OpenVms
            | ElfOsAbi::NonStop
            | ElfOsAbi::Aros
            | ElfOsAbi::FenixOs
            | ElfOsAbi::Nuxi
            | ElfOsAbi::OpenVos => {
                return Ok(());
            }
            _ => {
                return Err(ElfErrors::InvalidOsAbi);
            }
        }
    }

    /// Converts an `ElfOsAbi` enum into a readable string.
    pub fn as_str(&self) -> &'static str {
        match self {
            ElfOsAbi::SystemV => "Unix System - V",
            ElfOsAbi::HpUx => "HP-UX",
            ElfOsAbi::NetBsd => "Net BSD",
            ElfOsAbi::Linux => "Linux",
            ElfOsAbi::GnuHurd => "GNU HURD",
            ElfOsAbi::Solaris => "Solaris",
            ElfOsAbi::Aix => "Monterey AIX",
            ElfOsAbi::Irix => "Irix",
            ElfOsAbi::FreeBsd => "FreeBSD",
            ElfOsAbi::Tru64 => "Tru64 Unix",
            ElfOsAbi::NovellModesto => "Novell Modesto",
            ElfOsAbi::OpenBsd => "Open BSD",
            ElfOsAbi::OpenVms => "Open VMS",
            ElfOsAbi::NonStop => "NonStop Kernel",
            ElfOsAbi::Aros => "AROS research operating system",
            ElfOsAbi::FenixOs => "Fenix OS",
            ElfOsAbi::Nuxi => "Nuxi",
            ElfOsAbi::OpenVos => "Stratus Technologies OpenVOS",
            ElfOsAbi::OsAbiUnknown(_) => "Unknown ABI",
        }
    }
}

mod test {
    use super::*;

    // Check the string values of each type.
    #[test]
    fn test_os_abi_as_string() {
        assert_eq!(ElfOsAbi::SystemV.as_str(), "Unix System - V");
        assert_eq!(ElfOsAbi::HpUx.as_str(), "HP-UX");
        assert_eq!(ElfOsAbi::NetBsd.as_str(), "Net BSD");
        assert_eq!(ElfOsAbi::Linux.as_str(), "Linux");
        assert_eq!(ElfOsAbi::GnuHurd.as_str(), "GNU HURD");
        assert_eq!(ElfOsAbi::Solaris.as_str(), "Solaris");
        assert_eq!(ElfOsAbi::Aix.as_str(), "Monterey AIX");
        assert_eq!(ElfOsAbi::Irix.as_str(), "Irix");
        assert_eq!(ElfOsAbi::FreeBsd.as_str(), "FreeBSD");
        assert_eq!(ElfOsAbi::Tru64.as_str(), "Tru64 Unix");
        assert_eq!(ElfOsAbi::NovellModesto.as_str(), "Novell Modesto");
        assert_eq!(ElfOsAbi::OpenBsd.as_str(), "Open BSD");
        assert_eq!(ElfOsAbi::OpenVms.as_str(), "Open VMS");
        assert_eq!(ElfOsAbi::NonStop.as_str(), "NonStop Kernel");
        assert_eq!(ElfOsAbi::Aros.as_str(), "AROS research operating system");
        assert_eq!(ElfOsAbi::FenixOs.as_str(), "Fenix OS");
        assert_eq!(ElfOsAbi::Nuxi.as_str(), "Nuxi");
        assert_eq!(ElfOsAbi::OpenVos.as_str(), "Stratus Technologies OpenVOS");
        assert_eq!(ElfOsAbi::OsAbiUnknown(0).as_str(), "Unknown ABI");
    }

    // Check ABI to u8 conversion.
    #[test]
    fn test_os_abi_as_u8() {
        assert_eq!(u8::from(ElfOsAbi::SystemV), 0x00);
        assert_eq!(u8::from(ElfOsAbi::HpUx), 0x01);
        assert_eq!(u8::from(ElfOsAbi::NetBsd), 0x02);
        assert_eq!(u8::from(ElfOsAbi::Linux), 0x03);
        assert_eq!(u8::from(ElfOsAbi::GnuHurd), 0x04);
        assert_eq!(u8::from(ElfOsAbi::Solaris), 0x06);
        assert_eq!(u8::from(ElfOsAbi::Aix), 0x07);
        assert_eq!(u8::from(ElfOsAbi::Irix), 0x08);
        assert_eq!(u8::from(ElfOsAbi::FreeBsd), 0x09);
        assert_eq!(u8::from(ElfOsAbi::Tru64), 0x0A);
        assert_eq!(u8::from(ElfOsAbi::NovellModesto), 0x0B);
        assert_eq!(u8::from(ElfOsAbi::OpenBsd), 0x0C);
        assert_eq!(u8::from(ElfOsAbi::OpenVms), 0x0D);
        assert_eq!(u8::from(ElfOsAbi::NonStop), 0x0E);
        assert_eq!(u8::from(ElfOsAbi::Aros), 0x0F);
        assert_eq!(u8::from(ElfOsAbi::FenixOs), 0x10);
        assert_eq!(u8::from(ElfOsAbi::Nuxi), 0x11);
        assert_eq!(u8::from(ElfOsAbi::OpenVos), 0x12);
        // Any unknown values
        assert_eq!(u8::from(ElfOsAbi::OsAbiUnknown(0x12)), 0x12);
    }

    // Check u8 to ABI conversion.
    #[test]
    fn test_os_abi_from_u8() {
        assert_eq!(ElfOsAbi::from(0x00), ElfOsAbi::SystemV);
        assert_eq!(ElfOsAbi::from(0x01), ElfOsAbi::HpUx);
        assert_eq!(ElfOsAbi::from(0x02), ElfOsAbi::NetBsd);
        assert_eq!(ElfOsAbi::from(0x03), ElfOsAbi::Linux);
        assert_eq!(ElfOsAbi::from(0x04), ElfOsAbi::GnuHurd);
        assert_eq!(ElfOsAbi::from(0x06), ElfOsAbi::Solaris);
        assert_eq!(ElfOsAbi::from(0x07), ElfOsAbi::Aix);
        assert_eq!(ElfOsAbi::from(0x08), ElfOsAbi::Irix);
        assert_eq!(ElfOsAbi::from(0x09), ElfOsAbi::FreeBsd);
        assert_eq!(ElfOsAbi::from(0x0A), ElfOsAbi::Tru64);
        assert_eq!(ElfOsAbi::from(0x0B), ElfOsAbi::NovellModesto);
        assert_eq!(ElfOsAbi::from(0x0C), ElfOsAbi::OpenBsd);
        assert_eq!(ElfOsAbi::from(0x0D), ElfOsAbi::OpenVms);
        assert_eq!(ElfOsAbi::from(0x0E), ElfOsAbi::NonStop);
        assert_eq!(ElfOsAbi::from(0x0F), ElfOsAbi::Aros);
        assert_eq!(ElfOsAbi::from(0x10), ElfOsAbi::FenixOs);
        assert_eq!(ElfOsAbi::from(0x11), ElfOsAbi::Nuxi);
        assert_eq!(ElfOsAbi::from(0x12), ElfOsAbi::OpenVos);
        // Any unsupported value will store as UnknownType with that value.
        assert_eq!(ElfOsAbi::from(0x05), ElfOsAbi::OsAbiUnknown(0x05)); // 5 is not allocated
        assert_eq!(ElfOsAbi::from(0x60), ElfOsAbi::OsAbiUnknown(0x60)); // A large value which is not supported
    }

    // Check instance creation with new()
    #[test]
    fn test_os_abi_new() {
        let my_elf_type = ElfOsAbi::new(0);
        assert_eq!(my_elf_type, ElfOsAbi::SystemV);
        let my_elf_type = ElfOsAbi::new(1);
        assert_eq!(my_elf_type, ElfOsAbi::HpUx);
        let my_elf_type = ElfOsAbi::new(2);
        assert_eq!(my_elf_type, ElfOsAbi::NetBsd);
        let my_elf_type = ElfOsAbi::new(3);
        assert_eq!(my_elf_type, ElfOsAbi::Linux);
        let my_elf_type = ElfOsAbi::new(4);
        assert_eq!(my_elf_type, ElfOsAbi::GnuHurd);
        let my_elf_type = ElfOsAbi::new(5);
        assert_eq!(my_elf_type, ElfOsAbi::OsAbiUnknown(5)); // 5 is not supported.
        let my_elf_type = ElfOsAbi::new(6);
        assert_eq!(my_elf_type, ElfOsAbi::Solaris);
        let my_elf_type = ElfOsAbi::new(7);
        assert_eq!(my_elf_type, ElfOsAbi::Aix);
        let my_elf_type = ElfOsAbi::new(8);
        assert_eq!(my_elf_type, ElfOsAbi::Irix);
        let my_elf_type = ElfOsAbi::new(9);
        assert_eq!(my_elf_type, ElfOsAbi::FreeBsd);
        let my_elf_type = ElfOsAbi::new(10);
        assert_eq!(my_elf_type, ElfOsAbi::Tru64);
        let my_elf_type = ElfOsAbi::new(11);
        assert_eq!(my_elf_type, ElfOsAbi::NovellModesto);
        let my_elf_type = ElfOsAbi::new(12);
        assert_eq!(my_elf_type, ElfOsAbi::OpenBsd);
        let my_elf_type = ElfOsAbi::new(13);
        assert_eq!(my_elf_type, ElfOsAbi::OpenVms);
        let my_elf_type = ElfOsAbi::new(14);
        assert_eq!(my_elf_type, ElfOsAbi::NonStop);
        let my_elf_type = ElfOsAbi::new(15);
        assert_eq!(my_elf_type, ElfOsAbi::Aros);
        let my_elf_type = ElfOsAbi::new(16);
        assert_eq!(my_elf_type, ElfOsAbi::FenixOs);
        let my_elf_type = ElfOsAbi::new(17);
        assert_eq!(my_elf_type, ElfOsAbi::Nuxi);
        let my_elf_type = ElfOsAbi::new(18);
        assert_eq!(my_elf_type, ElfOsAbi::OpenVos);
        let my_elf_type = ElfOsAbi::new(20);
        assert_eq!(my_elf_type, ElfOsAbi::OsAbiUnknown(20));
        let my_elf_type = ElfOsAbi::new(0xff);
        assert_eq!(my_elf_type, ElfOsAbi::OsAbiUnknown(255));
    }

    // Check validity of OS ABI type
    #[test]
    fn test_os_abi_validity() {
        assert_eq!(ElfOsAbi::SystemV.validate(), Ok(()));
        assert_eq!(ElfOsAbi::HpUx.validate(), Ok(()));
        assert_eq!(ElfOsAbi::NetBsd.validate(), Ok(()));
        assert_eq!(ElfOsAbi::Linux.validate(), Ok(()));
        assert_eq!(ElfOsAbi::GnuHurd.validate(), Ok(()));
        assert_eq!(ElfOsAbi::Solaris.validate(), Ok(()));
        assert_eq!(ElfOsAbi::Aix.validate(), Ok(()));
        assert_eq!(ElfOsAbi::Irix.validate(), Ok(()));
        assert_eq!(ElfOsAbi::FreeBsd.validate(), Ok(()));
        assert_eq!(ElfOsAbi::Tru64.validate(), Ok(()));
        assert_eq!(ElfOsAbi::NovellModesto.validate(), Ok(()));
        assert_eq!(ElfOsAbi::OpenBsd.validate(), Ok(()));
        assert_eq!(ElfOsAbi::OpenVms.validate(), Ok(()));
        assert_eq!(ElfOsAbi::NonStop.validate(), Ok(()));
        assert_eq!(ElfOsAbi::Aros.validate(), Ok(()));
        assert_eq!(ElfOsAbi::FenixOs.validate(), Ok(()));
        assert_eq!(ElfOsAbi::Nuxi.validate(), Ok(()));
        assert_eq!(ElfOsAbi::OpenVos.validate(), Ok(()));
        assert_eq!(
            ElfOsAbi::OsAbiUnknown(1).validate(),
            Err(ElfErrors::InvalidOsAbi)
        );
        assert_eq!(
            ElfOsAbi::OsAbiUnknown(0xff).validate(),
            Err(ElfErrors::InvalidOsAbi)
        );
    }
}
