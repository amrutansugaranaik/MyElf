// @author: Amrutansu Garanaik

use crate::elf_errors::ElfErrors;

/// ELF identification OS ABI (`EI_OSABI`).
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

#[cfg(test)]
mod test {
    use super::*;

    const KNOWN: &[(u8, ElfOsAbi, &str)] = &[
        (0x00, ElfOsAbi::SystemV, "Unix System - V"),
        (0x01, ElfOsAbi::HpUx, "HP-UX"),
        (0x02, ElfOsAbi::NetBsd, "Net BSD"),
        (0x03, ElfOsAbi::Linux, "Linux"),
        (0x04, ElfOsAbi::GnuHurd, "GNU HURD"),
        (0x06, ElfOsAbi::Solaris, "Solaris"),
        (0x07, ElfOsAbi::Aix, "Monterey AIX"),
        (0x08, ElfOsAbi::Irix, "Irix"),
        (0x09, ElfOsAbi::FreeBsd, "FreeBSD"),
        (0x0a, ElfOsAbi::Tru64, "Tru64 Unix"),
        (0x0b, ElfOsAbi::NovellModesto, "Novell Modesto"),
        (0x0c, ElfOsAbi::OpenBsd, "Open BSD"),
        (0x0d, ElfOsAbi::OpenVms, "Open VMS"),
        (0x0e, ElfOsAbi::NonStop, "NonStop Kernel"),
        (0x0f, ElfOsAbi::Aros, "AROS research operating system"),
        (0x10, ElfOsAbi::FenixOs, "Fenix OS"),
        (0x11, ElfOsAbi::Nuxi, "Nuxi"),
        (0x12, ElfOsAbi::OpenVos, "Stratus Technologies OpenVOS"),
    ];

    #[test]
    fn test_os_abi_conversions_and_strings() {
        for (raw, abi, name) in KNOWN {
            assert_eq!(ElfOsAbi::from(*raw), *abi);
            assert_eq!(u8::from(*abi), *raw);
            assert_eq!(ElfOsAbi::new(*raw), *abi);
            assert_eq!(abi.as_str(), *name);
        }

        for raw in [0x05, 0x60, u8::MAX] {
            let unknown = ElfOsAbi::OsAbiUnknown(raw);
            assert_eq!(ElfOsAbi::from(raw), unknown);
            assert_eq!(u8::from(unknown), raw);
            assert_eq!(ElfOsAbi::new(raw), unknown);
            assert_eq!(unknown.as_str(), "Unknown ABI");
        }
    }

    #[test]
    fn test_os_abi_validity() {
        for (_, abi, _) in KNOWN {
            assert_eq!(abi.validate(), Ok(()));
        }
        assert_eq!(
            ElfOsAbi::OsAbiUnknown(0xff).validate(),
            Err(ElfErrors::InvalidOsAbi)
        );
    }
}
