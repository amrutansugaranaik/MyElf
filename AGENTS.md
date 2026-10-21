# MyElf project guide

## Project purpose

MyElf is a small Rust library and set of command-line tools for reading and
writing ELF files. The library is intended to be the shared foundation for
`readelf`-like inspection tools, `objdump`-like tools, object-file creation,
and eventually an assembler and linker.

The design goal is a minimal, portable implementation that works across the
project owner's devices and can grow to support additional targets:

- RISC-V with the IMA 32-bit profile
- AArch64 (including Apple Silicon)
- 32-bit ARM (including Raspberry Pi)
- x86-64

Do not assume the host machine's architecture is the ELF file's architecture.
ELF class, byte order, and machine are properties of each file and must be
handled explicitly. ELF32/ELF64 describe the ELF object format and its
associated field/address model, not the processor's physical bus width.

## Repository layout

```text
.
|-- Cargo.toml                 # Rust workspace manifest
|-- README.md                  # Project overview and setup notes
|-- AGENTS.md                  # Project conventions and ELF value reference
|-- libelf-rs/
|   |-- Cargo.toml             # Core library crate
|   `-- src/
|       |-- lib.rs             # Public modules and API
|       |-- elf_errors.rs      # Library error definitions
|       `-- types/             # ELF numeric values named after spec fields
`-- tools/
    |-- readelf/               # ELF inspection tool
    |-- objdump/               # Object inspection/disassembly tool
    `-- objcopy/               # Object conversion/copy tool
```

The workspace is configured for Rust 2024. Keep the core crate independent of
host-specific APIs. Avoid adding dependencies unless they materially reduce
maintenance or improve correctness.

## Type and implementation conventions

- Keep ELF constants and typed values under `libelf-rs/src/types/`, name each
  module after its ELF field (for example, `ei_osabi.rs` for `EI_OSABI`), and
  export each public module from `libelf-rs/src/lib.rs`.
- For a scalar ELF field, provide a typed enum, `From<raw integer>`,
  `From<enum> for raw integer`, `new(raw)`, `validate()`, and `as_str()`.
- Preserve unrecognized raw values in an `Unknown(raw)` variant so readers
  never lose information. `validate()` should distinguish values this version
  can safely create from values that are only preserved for reading.
- ELF flags are bit fields, not mutually exclusive enum choices. Model them
  as small typed bitmask structs with named constants and bitwise operations;
  do not reject combinations of known flags.
- Use explicit integer widths matching the ELF field (`u8`, `u16`, `u32`, or
  `u64`). Do not serialize Rust enum memory layouts directly.
- `From<integer>` handles an already decoded integer. It is not a substitute
  for decoding bytes from a slice; file parsing must use the file's declared
  byte order and checked bounds.
- Add unit tests for every public type: recognized mappings, reverse
  conversions, unknown-value round trips, strings, validation, and flag
  combinations where applicable.
- Prefer structured errors over panics or silent fallback. Do not use
  `unsafe` for ordinary field decoding.
- Keep documentation and tests updated when adding or changing ELF values.

## Current coverage and next implementation layers

The core library currently has typed values for ELF class, byte order, file
type, OS ABI, machine, program header type/flags, section header type/flags,
and ELF version. Modules use field-based names such as `ei_class`, `e_type`,
`p_type`, and `sh_flags`. It does not yet parse or write complete ELF files.

Recommended order after typed constants:

1. Define separate ELF32 and ELF64 layouts or class-dependent field types,
   then add checked byte-slice readers and writers with explicit endianness.
2. Parse the ELF identification and fixed header first; validate magic,
   supported version, class, byte order, and header sizes before using offsets.
3. Add program/section table readers with checked arithmetic, bounds checks,
   extended section numbering, and preservation of unknown records.
4. Add symbols, string tables, relocations, and dynamic entries before
   disassembly, object writing, linking, or assembling.
5. Test with small fixtures produced for each target and run cross-target
   builds. Host CPU support and ELF machine support are separate concerns.

Keep host tools thin: they should use the library for decoding and
representation instead of duplicating ELF logic.

## ELF specification

This section records the numeric values currently used by the library and
the principal generic ABI values needed for its early parser/writer. It is a
maintained quick reference, not a claim to contain every processor-specific
ABI, relocation, note, or vendor extension. Values not listed as implemented
must still be preserved as raw unknown values when reading.

### Identification fields

| Field | Name | Value | Meaning |
|---|---|---:|---|
| `EI_CLASS` | `ELFCLASSNONE` | `0` | Invalid/unspecified class |
| `EI_CLASS` | `ELFCLASS32` | `1` | 32-bit ELF |
| `EI_CLASS` | `ELFCLASS64` | `2` | 64-bit ELF |
| `EI_DATA` | `ELFDATANONE` | `0` | Invalid/unspecified encoding |
| `EI_DATA` | `ELFDATA2LSB` | `1` | Little-endian |
| `EI_DATA` | `ELFDATA2MSB` | `2` | Big-endian |
| `EI_VERSION` / `e_version` | `EV_NONE` | `0` | Invalid/unspecified version |
| `EI_VERSION` / `e_version` | `EV_CURRENT` | `1` | Current ELF version |

`EI_CLASS`, `EI_DATA`, and `EI_VERSION` are bytes in `e_ident`. `e_version` in
the ELF header is a 32-bit field; the value meanings are the same.
The library maps the zero sentinel values to `Unknown(0)` and rejects them
when validating data for output; it preserves their numeric value when
round-tripping.

### ELF file type (`e_type`)

| Name | Value | Meaning |
|---|---:|---|
| `ET_NONE` | `0` | No file type |
| `ET_REL` | `1` | Relocatable object |
| `ET_EXEC` | `2` | Executable |
| `ET_DYN` | `3` | Shared object or position-independent executable |
| `ET_CORE` | `4` | Core file |
| `ET_LOOS`–`ET_HIOS` | `0xfe00`–`0xfeff` | OS-specific |
| `ET_LOPROC`–`ET_HIPROC` | `0xff00`–`0xffff` | Processor-specific |

### OS ABI (`EI_OSABI`)

| Name | Value | Name | Value |
|---|---:|---|---:|
| `ELFOSABI_NONE` / System V | `0` | `ELFOSABI_HPUX` | `1` |
| `ELFOSABI_NETBSD` | `2` | `ELFOSABI_LINUX` | `3` |
| `ELFOSABI_HURD` | `4` | `ELFOSABI_SOLARIS` | `6` |
| `ELFOSABI_AIX` | `7` | `ELFOSABI_IRIX` | `8` |
| `ELFOSABI_FREEBSD` | `9` | `ELFOSABI_TRU64` | `10` |
| `ELFOSABI_MODESTO` | `11` | `ELFOSABI_OPENBSD` | `12` |
| `ELFOSABI_OPENVMS` | `13` | `ELFOSABI_NSK` | `14` |
| `ELFOSABI_AROS` | `15` | `ELFOSABI_FENIXOS` | `16` |
| `ELFOSABI_CLOUDABI` | `17` | `ELFOSABI_OPENVOS` | `18` |

`5` is unassigned. Values `64`–`255` are reserved for processor-specific OS
ABI use. Unknown values should be retained.

### Machine (`e_machine`)

The machine field is 16 bits. The following values are represented by the
library's `ElfIsa` type.

| Name | Value | Name | Value |
|---|---:|---|---:|
| `EM_NONE` | `0` | `EM_M32` | `1` |
| `EM_SPARC` | `2` | `EM_386` | `3` |
| `EM_68K` | `4` | `EM_88K` | `5` |
| `EM_IAMCU` | `6` | `EM_860` | `7` |
| `EM_MIPS` | `8` | `EM_S370` | `9` |
| `EM_MIPS_RS3_LE` | `10` | `EM_PARISC` | `15` |
| `EM_SPARC32PLUS` | `18` | `EM_960` | `19` |
| `EM_PPC` | `20` | `EM_PPC64` | `21` |
| `EM_S390` | `22` | `EM_ARM` | `40` |
| `EM_SH` | `42` | `EM_SPARCV9` | `43` |
| `EM_IA_64` | `50` | `EM_X86_64` | `62` |
| `EM_AARCH64` | `183` | `EM_RISCV` | `243` |
| `EM_BPF` | `247` | `EM_LOONGARCH` | `258` |

The project's current target set includes `EM_ARM` (32-bit ARM), `EM_AARCH64`,
`EM_RISCV`, and `EM_X86_64`. `EM_RISCV` does not encode the RISC-V
XLEN/extension profile: validate those through RISC-V ELF flags and attributes
when that layer is implemented. `EM_NONE` is a defined sentinel, not a usable
machine for a normal object.

### Program header type (`p_type`)

| Name | Value | Meaning |
|---|---:|---|
| `PT_NULL` | `0` | Unused entry |
| `PT_LOAD` | `1` | Loadable segment |
| `PT_DYNAMIC` | `2` | Dynamic linking information |
| `PT_INTERP` | `3` | Interpreter path |
| `PT_NOTE` | `4` | Note records |
| `PT_SHLIB` | `5` | Reserved |
| `PT_PHDR` | `6` | Program header table |
| `PT_TLS` | `7` | Thread-local storage |
| `PT_GNU_EH_FRAME` | `0x6474e550` | GNU exception frame header |
| `PT_GNU_STACK` | `0x6474e551` | GNU stack permissions |
| `PT_GNU_RELRO` | `0x6474e552` | GNU read-only-after-relocation segment |
| `PT_GNU_PROPERTY` | `0x6474e553` | GNU property notes |
| `PT_LOOS`–`PT_HIOS` | `0x60000000`–`0x6fffffff` | OS-specific |
| `PT_LOPROC`–`PT_HIPROC` | `0x70000000`–`0x7fffffff` | Processor-specific |

### Program header flags (`p_flags`)

These are combinable bits in a 32-bit word.

| Name | Value | Meaning |
|---|---:|---|
| `PF_X` | `0x1` | Executable |
| `PF_W` | `0x2` | Writable |
| `PF_R` | `0x4` | Readable |
| `PF_MASKOS` | `0x0ff00000` | OS-specific flag bits |
| `PF_MASKPROC` | `0xf0000000` | Processor-specific flag bits |

### Section header type (`sh_type`)

| Name | Value | Meaning |
|---|---:|---|
| `SHT_NULL` | `0` | Inactive header |
| `SHT_PROGBITS` | `1` | Program-defined data |
| `SHT_SYMTAB` | `2` | Symbol table |
| `SHT_STRTAB` | `3` | String table |
| `SHT_RELA` | `4` | Relocations with addends |
| `SHT_HASH` | `5` | Symbol hash table |
| `SHT_DYNAMIC` | `6` | Dynamic linking information |
| `SHT_NOTE` | `7` | Note records |
| `SHT_NOBITS` | `8` | No file data (for example, BSS) |
| `SHT_REL` | `9` | Relocations without addends |
| `SHT_SHLIB` | `10` | Reserved |
| `SHT_DYNSYM` | `11` | Dynamic symbol table |
| `SHT_INIT_ARRAY` | `14` | Initialization function pointers |
| `SHT_FINI_ARRAY` | `15` | Finalization function pointers |
| `SHT_PREINIT_ARRAY` | `16` | Pre-initialization function pointers |
| `SHT_GROUP` | `17` | Section group |
| `SHT_SYMTAB_SHNDX` | `18` | Extended symbol section indexes |
| `SHT_RELR` | `19` | Packed relative relocations |
| `SHT_GNU_ATTRIBUTES` | `0x6ffffff5` | GNU object attributes |
| `SHT_GNU_HASH` | `0x6ffffff6` | GNU hash table |
| `SHT_GNU_LIBLIST` | `0x6ffffff7` | GNU library list |
| `SHT_CHECKSUM` | `0x6ffffff8` | Checksum |
| `SHT_GNU_VERDEF` | `0x6ffffffd` | GNU version definitions |
| `SHT_GNU_VERNEED` | `0x6ffffffe` | GNU version requirements |
| `SHT_GNU_VERSYM` | `0x6fffffff` | GNU version symbols |
| `SHT_LOOS`–`SHT_HIOS` | `0x60000000`–`0x6fffffff` | OS-specific |
| `SHT_LOPROC`–`SHT_HIPROC` | `0x70000000`–`0x7fffffff` | Processor-specific |
| `SHT_LOUSER`–`SHT_HIUSER` | `0x80000000`–`0xffffffff` | Application-specific |

### Section header flags (`sh_flags`)

These are combinable bits in an ELF word (32 bits for ELF32 and 64 bits for
ELF64). The type wrapper uses `u64` to preserve ELF64 flags.

| Name | Value | Meaning |
|---|---:|---|
| `SHF_WRITE` | `0x1` | Writable |
| `SHF_ALLOC` | `0x2` | Occupies memory during execution |
| `SHF_EXECINSTR` | `0x4` | Executable instructions |
| `SHF_MERGE` | `0x10` | Mergeable entries |
| `SHF_STRINGS` | `0x20` | Null-terminated strings |
| `SHF_INFO_LINK` | `0x40` | `sh_info` contains a section index |
| `SHF_LINK_ORDER` | `0x80` | Preserve link order dependency |
| `SHF_OS_NONCONFORMING` | `0x100` | OS-specific handling required |
| `SHF_GROUP` | `0x200` | Member of a section group |
| `SHF_TLS` | `0x400` | Thread-local storage |
| `SHF_COMPRESSED` | `0x800` | Compressed section |
| `SHF_GNU_RETAIN` | `0x200000` | Do not discard during linking |
| `SHF_EXCLUDE` | `0x80000000` | Exclude from linking when appropriate |
| `SHF_MASKOS` | `0x0ff00000` | OS-specific flag bits |
| `SHF_MASKPROC` | `0xf0000000` | Processor-specific flag bits |

ELF extensions and processor ABIs may define additional meanings in reserved
ranges. Preserve such bits and values even before the library interprets them.
