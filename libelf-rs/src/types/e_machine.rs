// @author: Amrutansu Garanaik

use crate::elf_errors::ElfErrors;

/// ELF machine architecture identifier (`e_machine`).
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u16)]
pub enum ElfIsa {
    /// No machine specified.
    ElfIsaNone = 0,
    /// AT&T WE 32100.
    ElfIsaM32 = 1,
    /// SPARC.
    ElfIsaSparc = 2,
    /// Intel 80386.
    ElfIsaI386 = 3,
    /// Motorola 68000.
    ElfIsaM68k = 4,
    /// Motorola 88000.
    ElfIsaM88k = 5,
    /// Intel MCU.
    ElfIsaIamcu = 6,
    /// Intel 80860.
    ElfIsaI860 = 7,
    /// MIPS.
    ElfIsaMips = 8,
    /// IBM System/370.
    ElfIsaS370 = 9,
    /// MIPS RS3000 little-endian.
    ElfIsaMipsRs3Le = 10,
    /// Hewlett-Packard PA-RISC.
    ElfIsaPaRisc = 15,
    /// SPARC with enhanced instruction set.
    ElfIsaSparc32Plus = 18,
    /// Intel 80960.
    ElfIsaI960 = 19,
    /// PowerPC.
    ElfIsaPowerPc = 20,
    /// 64-bit PowerPC.
    ElfIsaPowerPc64 = 21,
    /// IBM System/390.
    ElfIsaS390 = 22,
    /// ARM.
    ElfIsaArm = 40,
    /// Renesas / Hitachi SuperH.
    ElfIsaSuperH = 42,
    /// 64-bit SPARC.
    ElfIsaSparcV9 = 43,
    /// Intel Itanium.
    ElfIsaIa64 = 50,
    /// AMD x86-64.
    ElfIsaX86_64 = 62,
    /// 64-bit ARM.
    ElfIsaAArch64 = 183,
    /// RISC-V.
    ElfIsaRiscV = 243,
    /// Linux kernel BPF virtual machine.
    ElfIsaBpf = 247,
    /// LoongArch.
    ElfIsaLoongArch = 258,
    /// Unknown or unsupported machine architecture.
    ElfIsaUnknown(u16),
}

impl From<u16> for ElfIsa {
    fn from(value: u16) -> Self {
        match value {
            0 => ElfIsa::ElfIsaNone,
            1 => ElfIsa::ElfIsaM32,
            2 => ElfIsa::ElfIsaSparc,
            3 => ElfIsa::ElfIsaI386,
            4 => ElfIsa::ElfIsaM68k,
            5 => ElfIsa::ElfIsaM88k,
            6 => ElfIsa::ElfIsaIamcu,
            7 => ElfIsa::ElfIsaI860,
            8 => ElfIsa::ElfIsaMips,
            9 => ElfIsa::ElfIsaS370,
            10 => ElfIsa::ElfIsaMipsRs3Le,
            15 => ElfIsa::ElfIsaPaRisc,
            18 => ElfIsa::ElfIsaSparc32Plus,
            19 => ElfIsa::ElfIsaI960,
            20 => ElfIsa::ElfIsaPowerPc,
            21 => ElfIsa::ElfIsaPowerPc64,
            22 => ElfIsa::ElfIsaS390,
            40 => ElfIsa::ElfIsaArm,
            42 => ElfIsa::ElfIsaSuperH,
            43 => ElfIsa::ElfIsaSparcV9,
            50 => ElfIsa::ElfIsaIa64,
            62 => ElfIsa::ElfIsaX86_64,
            183 => ElfIsa::ElfIsaAArch64,
            243 => ElfIsa::ElfIsaRiscV,
            247 => ElfIsa::ElfIsaBpf,
            258 => ElfIsa::ElfIsaLoongArch,
            _ => ElfIsa::ElfIsaUnknown(value),
        }
    }
}

impl From<ElfIsa> for u16 {
    fn from(value: ElfIsa) -> u16 {
        match value {
            ElfIsa::ElfIsaNone => 0,
            ElfIsa::ElfIsaM32 => 1,
            ElfIsa::ElfIsaSparc => 2,
            ElfIsa::ElfIsaI386 => 3,
            ElfIsa::ElfIsaM68k => 4,
            ElfIsa::ElfIsaM88k => 5,
            ElfIsa::ElfIsaIamcu => 6,
            ElfIsa::ElfIsaI860 => 7,
            ElfIsa::ElfIsaMips => 8,
            ElfIsa::ElfIsaS370 => 9,
            ElfIsa::ElfIsaMipsRs3Le => 10,
            ElfIsa::ElfIsaPaRisc => 15,
            ElfIsa::ElfIsaSparc32Plus => 18,
            ElfIsa::ElfIsaI960 => 19,
            ElfIsa::ElfIsaPowerPc => 20,
            ElfIsa::ElfIsaPowerPc64 => 21,
            ElfIsa::ElfIsaS390 => 22,
            ElfIsa::ElfIsaArm => 40,
            ElfIsa::ElfIsaSuperH => 42,
            ElfIsa::ElfIsaSparcV9 => 43,
            ElfIsa::ElfIsaIa64 => 50,
            ElfIsa::ElfIsaX86_64 => 62,
            ElfIsa::ElfIsaAArch64 => 183,
            ElfIsa::ElfIsaRiscV => 243,
            ElfIsa::ElfIsaBpf => 247,
            ElfIsa::ElfIsaLoongArch => 258,
            ElfIsa::ElfIsaUnknown(value) => value,
        }
    }
}

impl ElfIsa {
    /// Create an `ElfIsa` from its ELF machine identifier.
    pub fn new(value: u16) -> Self {
        Self::from(value)
    }

    /// Check whether this is a recognized ELF machine architecture.
    pub fn validate(&self) -> Result<(), ElfErrors> {
        match self {
            ElfIsa::ElfIsaM32
            | ElfIsa::ElfIsaSparc
            | ElfIsa::ElfIsaI386
            | ElfIsa::ElfIsaM68k
            | ElfIsa::ElfIsaM88k
            | ElfIsa::ElfIsaIamcu
            | ElfIsa::ElfIsaI860
            | ElfIsa::ElfIsaMips
            | ElfIsa::ElfIsaS370
            | ElfIsa::ElfIsaMipsRs3Le
            | ElfIsa::ElfIsaPaRisc
            | ElfIsa::ElfIsaSparc32Plus
            | ElfIsa::ElfIsaI960
            | ElfIsa::ElfIsaPowerPc
            | ElfIsa::ElfIsaPowerPc64
            | ElfIsa::ElfIsaS390
            | ElfIsa::ElfIsaArm
            | ElfIsa::ElfIsaSuperH
            | ElfIsa::ElfIsaSparcV9
            | ElfIsa::ElfIsaIa64
            | ElfIsa::ElfIsaX86_64
            | ElfIsa::ElfIsaAArch64
            | ElfIsa::ElfIsaRiscV
            | ElfIsa::ElfIsaBpf
            | ElfIsa::ElfIsaLoongArch => Ok(()),
            ElfIsa::ElfIsaNone | ElfIsa::ElfIsaUnknown(_) => Err(ElfErrors::InvalidElfIsa),
        }
    }

    /// Convert this architecture into a readable string.
    pub fn as_str(&self) -> &'static str {
        match self {
            ElfIsa::ElfIsaNone => "None",
            ElfIsa::ElfIsaM32 => "AT&T WE 32100",
            ElfIsa::ElfIsaSparc => "SPARC",
            ElfIsa::ElfIsaI386 => "Intel 80386",
            ElfIsa::ElfIsaM68k => "Motorola 68000",
            ElfIsa::ElfIsaM88k => "Motorola 88000",
            ElfIsa::ElfIsaIamcu => "Intel MCU",
            ElfIsa::ElfIsaI860 => "Intel 80860",
            ElfIsa::ElfIsaMips => "MIPS",
            ElfIsa::ElfIsaS370 => "IBM System/370",
            ElfIsa::ElfIsaMipsRs3Le => "MIPS RS3000 little-endian",
            ElfIsa::ElfIsaPaRisc => "Hewlett-Packard PA-RISC",
            ElfIsa::ElfIsaSparc32Plus => "SPARC 32+",
            ElfIsa::ElfIsaI960 => "Intel 80960",
            ElfIsa::ElfIsaPowerPc => "PowerPC",
            ElfIsa::ElfIsaPowerPc64 => "PowerPC 64-bit",
            ElfIsa::ElfIsaS390 => "IBM System/390",
            ElfIsa::ElfIsaArm => "ARM",
            ElfIsa::ElfIsaSuperH => "SuperH",
            ElfIsa::ElfIsaSparcV9 => "SPARC V9",
            ElfIsa::ElfIsaIa64 => "Intel Itanium",
            ElfIsa::ElfIsaX86_64 => "AMD x86-64",
            ElfIsa::ElfIsaAArch64 => "AArch64",
            ElfIsa::ElfIsaRiscV => "RISC-V",
            ElfIsa::ElfIsaBpf => "Linux BPF",
            ElfIsa::ElfIsaLoongArch => "LoongArch",
            ElfIsa::ElfIsaUnknown(_) => "Unknown",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const KNOWN_ISAS: &[(u16, ElfIsa, &str)] = &[
        (0, ElfIsa::ElfIsaNone, "None"),
        (1, ElfIsa::ElfIsaM32, "AT&T WE 32100"),
        (2, ElfIsa::ElfIsaSparc, "SPARC"),
        (3, ElfIsa::ElfIsaI386, "Intel 80386"),
        (4, ElfIsa::ElfIsaM68k, "Motorola 68000"),
        (5, ElfIsa::ElfIsaM88k, "Motorola 88000"),
        (6, ElfIsa::ElfIsaIamcu, "Intel MCU"),
        (7, ElfIsa::ElfIsaI860, "Intel 80860"),
        (8, ElfIsa::ElfIsaMips, "MIPS"),
        (9, ElfIsa::ElfIsaS370, "IBM System/370"),
        (10, ElfIsa::ElfIsaMipsRs3Le, "MIPS RS3000 little-endian"),
        (15, ElfIsa::ElfIsaPaRisc, "Hewlett-Packard PA-RISC"),
        (18, ElfIsa::ElfIsaSparc32Plus, "SPARC 32+"),
        (19, ElfIsa::ElfIsaI960, "Intel 80960"),
        (20, ElfIsa::ElfIsaPowerPc, "PowerPC"),
        (21, ElfIsa::ElfIsaPowerPc64, "PowerPC 64-bit"),
        (22, ElfIsa::ElfIsaS390, "IBM System/390"),
        (40, ElfIsa::ElfIsaArm, "ARM"),
        (42, ElfIsa::ElfIsaSuperH, "SuperH"),
        (43, ElfIsa::ElfIsaSparcV9, "SPARC V9"),
        (50, ElfIsa::ElfIsaIa64, "Intel Itanium"),
        (62, ElfIsa::ElfIsaX86_64, "AMD x86-64"),
        (183, ElfIsa::ElfIsaAArch64, "AArch64"),
        (243, ElfIsa::ElfIsaRiscV, "RISC-V"),
        (247, ElfIsa::ElfIsaBpf, "Linux BPF"),
        (258, ElfIsa::ElfIsaLoongArch, "LoongArch"),
    ];

    #[test]
    fn test_elf_isa_as_string() {
        for (_, isa, name) in KNOWN_ISAS {
            assert_eq!(isa.as_str(), *name);
        }
        assert_eq!(ElfIsa::ElfIsaUnknown(0xffff).as_str(), "Unknown");
    }

    #[test]
    fn test_elf_isa_conversions() {
        for (value, isa, _) in KNOWN_ISAS {
            assert_eq!(ElfIsa::from(*value), *isa);
            assert_eq!(u16::from(*isa), *value);
            assert_eq!(ElfIsa::new(*value), *isa);
        }
        for value in [11, 255, u16::MAX] {
            let isa = ElfIsa::ElfIsaUnknown(value);
            assert_eq!(ElfIsa::from(value), isa);
            assert_eq!(u16::from(isa), value);
            assert_eq!(ElfIsa::new(value), isa);
        }
    }

    #[test]
    fn test_elf_isa_validity() {
        for (_, isa, _) in KNOWN_ISAS {
            if *isa == ElfIsa::ElfIsaNone {
                assert_eq!(isa.validate(), Err(ElfErrors::InvalidElfIsa));
            } else {
                assert_eq!(isa.validate(), Ok(()));
            }
        }
        assert_eq!(
            ElfIsa::ElfIsaUnknown(11).validate(),
            Err(ElfErrors::InvalidElfIsa)
        );
    }
}
