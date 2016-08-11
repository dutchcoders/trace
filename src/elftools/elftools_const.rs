// AUTO-GENERATED FILE, DO NOT EDIT [elftools_const.rs]


extern crate libc;


/* This file defines standard ELF types, structures, and macros.
   Copyright (C) 1995-2012 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <http://www.gnu.org/licenses/>.  */






pub const _ELF_H: u32 = 1;










/* Standard ELF types.  */








/* Type for a 16-bit quantity.  */


#[allow(non_camel_case_types)]
pub type Elf32_Half = u16;


#[allow(non_camel_case_types)]
pub type Elf64_Half = u16;




/* Types for signed and unsigned 32-bit quantities.  */


#[allow(non_camel_case_types)]
pub type Elf32_Word = u32;


#[allow(non_camel_case_types)]
pub type Elf32_Sword = i32;


#[allow(non_camel_case_types)]
pub type Elf64_Word = u32;


#[allow(non_camel_case_types)]
pub type Elf64_Sword = i32;




/* Types for signed and unsigned 64-bit quantities.  */


#[allow(non_camel_case_types)]
pub type Elf32_Xword = u64;


#[allow(non_camel_case_types)]
pub type Elf32_Sxword = i64;


#[allow(non_camel_case_types)]
pub type Elf64_Xword = u64;


#[allow(non_camel_case_types)]
pub type Elf64_Sxword = i64;




/* Type of addresses.  */


#[allow(non_camel_case_types)]
pub type Elf32_Addr = u32;


#[allow(non_camel_case_types)]
pub type Elf64_Addr = u64;




/* Type of file offsets.  */


#[allow(non_camel_case_types)]
pub type Elf32_Off = u32;


#[allow(non_camel_case_types)]
pub type Elf64_Off = u64;




/* Type for section indices, which are 16-bit quantities.  */


#[allow(non_camel_case_types)]
pub type Elf32_Section = u16;


#[allow(non_camel_case_types)]
pub type Elf64_Section = u16;




/* Type for version symbol information.  */


#[allow(non_camel_case_types)]
pub type Elf32_Versym = Elf32_Half;


#[allow(non_camel_case_types)]
pub type Elf64_Versym = Elf64_Half;






/* The ELF file header.  This appears at the start of every ELF file.  */
















































































/* Fields in the e_ident array.  The EI_* macros are indices into the
   array.  The macros under each EI_* macro are the values the byte
   may have.  */




pub const EI_MAG0: u32 = 0;
pub const ELFMAG0: u32 = 0x7f;


pub const EI_MAG1: u32 = 1;




pub const EI_MAG2: u32 = 2;




pub const EI_MAG3: u32 = 3;




/* Conglomeration of the identification bytes, for easy testing as a word.  */




pub const SELFMAG: u32 = 4;


pub const EI_CLASS: u32 = 4;
pub const ELFCLASSNONE: u32 = 0;
pub const ELFCLASS32: u32 = 1;
pub const ELFCLASS64: u32 = 2;
pub const ELFCLASSNUM: u32 = 3;


pub const EI_DATA: u32 = 5;
pub const ELFDATANONE: u32 = 0;
pub const ELFDATA2LSB: u32 = 1;
pub const ELFDATA2MSB: u32 = 2;
pub const ELFDATANUM: u32 = 3;


pub const EI_VERSION: u32 = 6;




pub const EI_OSABI: u32 = 7;
pub const ELFOSABI_NONE: u32 = 0;
pub const ELFOSABI_SYSV: u32 = 0;
pub const ELFOSABI_HPUX: u32 = 1;
pub const ELFOSABI_NETBSD: u32 = 2;
pub const ELFOSABI_GNU: u32 = 3;
pub const ELFOSABI_LINUX: u32 = ELFOSABI_GNU;
pub const ELFOSABI_SOLARIS: u32 = 6;
pub const ELFOSABI_AIX: u32 = 7;
pub const ELFOSABI_IRIX: u32 = 8;
pub const ELFOSABI_FREEBSD: u32 = 9;
pub const ELFOSABI_TRU64: u32 = 10;
pub const ELFOSABI_MODESTO: u32 = 11;
pub const ELFOSABI_OPENBSD: u32 = 12;
pub const ELFOSABI_ARM_AEABI: u32 = 64;
pub const ELFOSABI_ARM: u32 = 97;
pub const ELFOSABI_STANDALONE: u32 = 255;


pub const EI_ABIVERSION: u32 = 8;


pub const EI_PAD: u32 = 9;


/* Legal values for e_type (object file type).  */




pub const ET_NONE: u32 = 0;
pub const ET_REL: u32 = 1;
pub const ET_EXEC: u32 = 2;
pub const ET_DYN: u32 = 3;
pub const ET_CORE: u32 = 4;
pub const ET_NUM: u32 = 5;
pub const ET_LOOS: u32 = 0xfe00;
pub const ET_HIOS: u32 = 0xfeff;
pub const ET_LOPROC: u32 = 0xff00;
pub const ET_HIPROC: u32 = 0xffff;


/* Legal values for e_machine (architecture).  */




pub const EM_NONE: u32 = 0;
pub const EM_M32: u32 = 1;
pub const EM_SPARC: u32 = 2;
pub const EM_386: u32 = 3;
pub const EM_68K: u32 = 4;
pub const EM_88K: u32 = 5;
pub const EM_860: u32 = 7;
pub const EM_MIPS: u32 = 8;
pub const EM_S370: u32 = 9;
pub const EM_MIPS_RS3_LE: u32 = 10;


pub const EM_PARISC: u32 = 15;
pub const EM_VPP500: u32 = 17;
pub const EM_SPARC32PLUS: u32 = 18;
pub const EM_960: u32 = 19;
pub const EM_PPC: u32 = 20;
pub const EM_PPC64: u32 = 21;
pub const EM_S390: u32 = 22;


pub const EM_V800: u32 = 36;
pub const EM_FR20: u32 = 37;
pub const EM_RH32: u32 = 38;
pub const EM_RCE: u32 = 39;
pub const EM_ARM: u32 = 40;
pub const EM_FAKE_ALPHA: u32 = 41;
pub const EM_SH: u32 = 42;
pub const EM_SPARCV9: u32 = 43;
pub const EM_TRICORE: u32 = 44;
pub const EM_ARC: u32 = 45;
pub const EM_H8_300: u32 = 46;
pub const EM_H8_300H: u32 = 47;
pub const EM_H8S: u32 = 48;
pub const EM_H8_500: u32 = 49;
pub const EM_IA_64: u32 = 50;
pub const EM_MIPS_X: u32 = 51;
pub const EM_COLDFIRE: u32 = 52;
pub const EM_68HC12: u32 = 53;
pub const EM_MMA: u32 = 54;
pub const EM_PCP: u32 = 55;
pub const EM_NCPU: u32 = 56;
pub const EM_NDR1: u32 = 57;
pub const EM_STARCORE: u32 = 58;
pub const EM_ME16: u32 = 59;
pub const EM_ST100: u32 = 60;
pub const EM_TINYJ: u32 = 61;
pub const EM_X86_64: u32 = 62;
pub const EM_PDSP: u32 = 63;


pub const EM_FX66: u32 = 66;
pub const EM_ST9PLUS: u32 = 67;
pub const EM_ST7: u32 = 68;
pub const EM_68HC16: u32 = 69;
pub const EM_68HC11: u32 = 70;
pub const EM_68HC08: u32 = 71;
pub const EM_68HC05: u32 = 72;
pub const EM_SVX: u32 = 73;
pub const EM_ST19: u32 = 74;
pub const EM_VAX: u32 = 75;
pub const EM_CRIS: u32 = 76;
pub const EM_JAVELIN: u32 = 77;
pub const EM_FIREPATH: u32 = 78;
pub const EM_ZSP: u32 = 79;
pub const EM_MMIX: u32 = 80;
pub const EM_HUANY: u32 = 81;
pub const EM_PRISM: u32 = 82;
pub const EM_AVR: u32 = 83;
pub const EM_FR30: u32 = 84;
pub const EM_D10V: u32 = 85;
pub const EM_D30V: u32 = 86;
pub const EM_V850: u32 = 87;
pub const EM_M32R: u32 = 88;
pub const EM_MN10300: u32 = 89;
pub const EM_MN10200: u32 = 90;
pub const EM_PJ: u32 = 91;
pub const EM_OPENRISC: u32 = 92;
pub const EM_ARC_A5: u32 = 93;
pub const EM_XTENSA: u32 = 94;
pub const EM_TILEPRO: u32 = 188;
pub const EM_TILEGX: u32 = 191;
pub const EM_NUM: u32 = 192;


/* If it is necessary to assign new unofficial EM_* values, please
   pick large random numbers (0x8523, 0xa7f2, etc.) to minimize the
   chances of collision with official or non-GNU unofficial values.  */




pub const EM_ALPHA: u32 = 0x9026;


/* Legal values for e_version (version).  */




pub const EV_NONE: u32 = 0;
pub const EV_CURRENT: u32 = 1;
pub const EV_NUM: u32 = 2;


/* Section header.  */




























































/* Special section indices.  */




pub const SHN_UNDEF: u32 = 0;
pub const SHN_LORESERVE: u32 = 0xff00;
pub const SHN_LOPROC: u32 = 0xff00;








pub const SHN_HIPROC: u32 = 0xff1f;
pub const SHN_LOOS: u32 = 0xff20;
pub const SHN_HIOS: u32 = 0xff3f;
pub const SHN_ABS: u32 = 0xfff1;
pub const SHN_COMMON: u32 = 0xfff2;
pub const SHN_XINDEX: u32 = 0xffff;
pub const SHN_HIRESERVE: u32 = 0xffff;


/* Legal values for sh_type (section type).  */




pub const SHT_NULL: u32 = 0;
pub const SHT_PROGBITS: u32 = 1;
pub const SHT_SYMTAB: u32 = 2;
pub const SHT_STRTAB: u32 = 3;
pub const SHT_RELA: u32 = 4;
pub const SHT_HASH: u32 = 5;
pub const SHT_DYNAMIC: u32 = 6;
pub const SHT_NOTE: u32 = 7;
pub const SHT_NOBITS: u32 = 8;
pub const SHT_REL: u32 = 9;
pub const SHT_SHLIB: u32 = 10;
pub const SHT_DYNSYM: u32 = 11;
pub const SHT_INIT_ARRAY: u32 = 14;
pub const SHT_FINI_ARRAY: u32 = 15;
pub const SHT_PREINIT_ARRAY: u32 = 16;
pub const SHT_GROUP: u32 = 17;
pub const SHT_SYMTAB_SHNDX: u32 = 18;
pub const SHT_NUM: u32 = 19;
pub const SHT_LOOS: u32 = 0x60000000;
pub const SHT_GNU_ATTRIBUTES: u32 = 0x6ffffff5;
pub const SHT_GNU_HASH: u32 = 0x6ffffff6;
pub const SHT_GNU_LIBLIST: u32 = 0x6ffffff7;
pub const SHT_CHECKSUM: u32 = 0x6ffffff8;
pub const SHT_LOSUNW: u32 = 0x6ffffffa;
pub const SHT_SUNW_move: u32 = 0x6ffffffa;
pub const SHT_SUNW_COMDAT: u32 = 0x6ffffffb;
pub const SHT_SUNW_syminfo: u32 = 0x6ffffffc;
pub const SHT_GNU_verdef: u32 = 0x6ffffffd;
pub const SHT_GNU_verneed: u32 = 0x6ffffffe;
pub const SHT_GNU_versym: u32 = 0x6fffffff;
pub const SHT_HISUNW: u32 = 0x6fffffff;
pub const SHT_HIOS: u32 = 0x6fffffff;
pub const SHT_LOPROC: u32 = 0x70000000;
pub const SHT_HIPROC: u32 = 0x7fffffff;
pub const SHT_LOUSER: u32 = 0x80000000;
pub const SHT_HIUSER: u32 = 0x8fffffff;


/* Legal values for sh_flags (section flags).  */


























pub const SHF_MASKOS: u32 = 0x0ff00000;
pub const SHF_MASKPROC: u32 = 0xf0000000;










/* Section group handling.  */


pub const GRP_COMDAT: u32 = 0x1;


/* Symbol table entry.  */












































/* The syminfo section if available contains additional information about
   every dynamic symbol.  */




























/* Possible values for si_boundto.  */


pub const SYMINFO_BT_SELF: u32 = 0xffff;
pub const SYMINFO_BT_PARENT: u32 = 0xfffe;
pub const SYMINFO_BT_LOWRESERVE: u32 = 0xff00;


/* Possible bitmasks for si_flags.  */


pub const SYMINFO_FLG_DIRECT: u32 = 0x0001;
pub const SYMINFO_FLG_PASSTHRU: u32 = 0x0002;
pub const SYMINFO_FLG_COPY: u32 = 0x0004;




/* Syminfo version values.  */


pub const SYMINFO_NONE: u32 = 0;
pub const SYMINFO_CURRENT: u32 = 1;
pub const SYMINFO_NUM: u32 = 2;




/* How to extract and insert information held in the st_info field.  */












/* Both Elf32_Sym and Elf64_Sym use the same one-byte st_info field.  */










/* Legal values for ST_BIND subfield of st_info (symbol binding).  */




pub const STB_LOCAL: u32 = 0;
pub const STB_GLOBAL: u32 = 1;
pub const STB_WEAK: u32 = 2;
pub const STB_NUM: u32 = 3;
pub const STB_LOOS: u32 = 10;
pub const STB_GNU_UNIQUE: u32 = 10;
pub const STB_HIOS: u32 = 12;
pub const STB_LOPROC: u32 = 13;
pub const STB_HIPROC: u32 = 15;


/* Legal values for ST_TYPE subfield of st_info (symbol type).  */




pub const STT_NOTYPE: u32 = 0;
pub const STT_OBJECT: u32 = 1;
pub const STT_FUNC: u32 = 2;
pub const STT_SECTION: u32 = 3;
pub const STT_FILE: u32 = 4;
pub const STT_COMMON: u32 = 5;
pub const STT_TLS: u32 = 6;
pub const STT_NUM: u32 = 7;
pub const STT_LOOS: u32 = 10;
pub const STT_GNU_IFUNC: u32 = 10;
pub const STT_HIOS: u32 = 12;
pub const STT_LOPROC: u32 = 13;
pub const STT_HIPROC: u32 = 15;




/* Symbol table indices are found in the hash buckets and chain table
   of a symbol hash table section.  This special index value indicates
   the end of a chain, meaning no further symbols are found in that bucket.  */




pub const STN_UNDEF: u32 = 0;




/* How to extract and insert information held in the st_other field.  */








/* For ELF64 the definitions are the same.  */






/* Symbol visibility specification encoded in the st_other field.  */


pub const STV_DEFAULT: u32 = 0;
pub const STV_INTERNAL: u32 = 1;
pub const STV_HIDDEN: u32 = 2;
pub const STV_PROTECTED: u32 = 3;




/* Relocation table entry without addend (in section of type SHT_REL).  */
















/* I have seen two different definitions of the Elf64_Rel and
   Elf64_Rela structures, so we'll leave them out until Novell (or
   whoever) gets their act together.  */


/* The following, at least, is used on Sparc v9, MIPS, and Alpha.  */
















/* Relocation table entry with addend (in section of type SHT_RELA).  */
































/* How to extract and insert information held in the r_info field.  */




















/* Program segment header.  */




















































/* Special value for e_phnum.  This indicates that the real number of
   program headers is too large to fit into e_phnum.  Instead the real
   value is in the field sh_info of section 0.  */




pub const PN_XNUM: u32 = 0xffff;


/* Legal values for p_type (segment type).  */




pub const PT_NULL: u32 = 0;
pub const PT_LOAD: u32 = 1;
pub const PT_DYNAMIC: u32 = 2;
pub const PT_INTERP: u32 = 3;
pub const PT_NOTE: u32 = 4;
pub const PT_SHLIB: u32 = 5;
pub const PT_PHDR: u32 = 6;
pub const PT_TLS: u32 = 7;
pub const PT_NUM: u32 = 8;
pub const PT_LOOS: u32 = 0x60000000;
pub const PT_GNU_EH_FRAME: u32 = 0x6474e550;
pub const PT_GNU_STACK: u32 = 0x6474e551;
pub const PT_GNU_RELRO: u32 = 0x6474e552;
pub const PT_LOSUNW: u32 = 0x6ffffffa;
pub const PT_SUNWBSS: u32 = 0x6ffffffa;
pub const PT_SUNWSTACK: u32 = 0x6ffffffb;
pub const PT_HISUNW: u32 = 0x6fffffff;
pub const PT_HIOS: u32 = 0x6fffffff;
pub const PT_LOPROC: u32 = 0x70000000;
pub const PT_HIPROC: u32 = 0x7fffffff;


/* Legal values for p_flags (segment flags).  */










pub const PF_MASKOS: u32 = 0x0ff00000;
pub const PF_MASKPROC: u32 = 0xf0000000;


/* Legal values for note segment descriptor types for core files. */




pub const NT_PRSTATUS: u32 = 1;
pub const NT_FPREGSET: u32 = 2;
pub const NT_PRPSINFO: u32 = 3;
pub const NT_PRXREG: u32 = 4;
pub const NT_TASKSTRUCT: u32 = 4;
pub const NT_PLATFORM: u32 = 5;
pub const NT_AUXV: u32 = 6;
pub const NT_GWINDOWS: u32 = 7;
pub const NT_ASRS: u32 = 8;
pub const NT_PSTATUS: u32 = 10;
pub const NT_PSINFO: u32 = 13;
pub const NT_PRCRED: u32 = 14;
pub const NT_UTSNAME: u32 = 15;
pub const NT_LWPSTATUS: u32 = 16;
pub const NT_LWPSINFO: u32 = 17;
pub const NT_PRFPXREG: u32 = 20;
pub const NT_PRXFPREG: u32 = 0x46e62b7f;
pub const NT_PPC_VMX: u32 = 0x100;
pub const NT_PPC_SPE: u32 = 0x101;
pub const NT_PPC_VSX: u32 = 0x102;
pub const NT_386_TLS: u32 = 0x200;
pub const NT_386_IOPERM: u32 = 0x201;
pub const NT_X86_XSTATE: u32 = 0x202;


/* Legal values for the note segment descriptor types for object files.  */




pub const NT_VERSION: u32 = 1;




/* Dynamic section entry.  */












































/* Legal values for d_tag (dynamic entry type).  */




pub const DT_NULL: u32 = 0;
pub const DT_NEEDED: u32 = 1;
pub const DT_PLTRELSZ: u32 = 2;
pub const DT_PLTGOT: u32 = 3;
pub const DT_HASH: u32 = 4;
pub const DT_STRTAB: u32 = 5;
pub const DT_SYMTAB: u32 = 6;
pub const DT_RELA: u32 = 7;
pub const DT_RELASZ: u32 = 8;
pub const DT_RELAENT: u32 = 9;
pub const DT_STRSZ: u32 = 10;
pub const DT_SYMENT: u32 = 11;
pub const DT_INIT: u32 = 12;
pub const DT_FINI: u32 = 13;
pub const DT_SONAME: u32 = 14;
pub const DT_RPATH: u32 = 15;
pub const DT_SYMBOLIC: u32 = 16;
pub const DT_REL: u32 = 17;
pub const DT_RELSZ: u32 = 18;
pub const DT_RELENT: u32 = 19;
pub const DT_PLTREL: u32 = 20;
pub const DT_DEBUG: u32 = 21;
pub const DT_TEXTREL: u32 = 22;
pub const DT_JMPREL: u32 = 23;
pub const DT_BIND_NOW: u32 = 24;
pub const DT_INIT_ARRAY: u32 = 25;
pub const DT_FINI_ARRAY: u32 = 26;
pub const DT_INIT_ARRAYSZ: u32 = 27;
pub const DT_FINI_ARRAYSZ: u32 = 28;
pub const DT_RUNPATH: u32 = 29;
pub const DT_FLAGS: u32 = 30;
pub const DT_ENCODING: u32 = 32;
pub const DT_PREINIT_ARRAY: u32 = 32;
pub const DT_PREINIT_ARRAYSZ: u32 = 33;
pub const DT_NUM: u32 = 34;
pub const DT_LOOS: u32 = 0x6000000d;
pub const DT_HIOS: u32 = 0x6ffff000;
pub const DT_LOPROC: u32 = 0x70000000;
pub const DT_HIPROC: u32 = 0x7fffffff;
pub const DT_PROCNUM: u32 = DT_MIPS_NUM;


/* DT_* entries which fall between DT_VALRNGHI & DT_VALRNGLO use the
   Dyn.d_un.d_val field of the Elf*_Dyn structure.  This follows Sun's
   approach.  */


pub const DT_VALRNGLO: u32 = 0x6ffffd00;
pub const DT_GNU_PRELINKED: u32 = 0x6ffffdf5;
pub const DT_GNU_CONFLICTSZ: u32 = 0x6ffffdf6;
pub const DT_GNU_LIBLISTSZ: u32 = 0x6ffffdf7;
pub const DT_CHECKSUM: u32 = 0x6ffffdf8;
pub const DT_PLTPADSZ: u32 = 0x6ffffdf9;
pub const DT_MOVEENT: u32 = 0x6ffffdfa;
pub const DT_MOVESZ: u32 = 0x6ffffdfb;
pub const DT_FEATURE_1: u32 = 0x6ffffdfc;




pub const DT_SYMINSZ: u32 = 0x6ffffdfe;
pub const DT_SYMINENT: u32 = 0x6ffffdff;
pub const DT_VALRNGHI: u32 = 0x6ffffdff;


pub const DT_VALNUM: u32 = 12;


/* DT_* entries which fall between DT_ADDRRNGHI & DT_ADDRRNGLO use the
   Dyn.d_un.d_ptr field of the Elf*_Dyn structure.

   If any adjustment is made to the ELF object after it has been
   built these entries will need to be adjusted.  */


pub const DT_ADDRRNGLO: u32 = 0x6ffffe00;
pub const DT_GNU_HASH: u32 = 0x6ffffef5;
pub const DT_TLSDESC_PLT: u32 = 0x6ffffef6;
pub const DT_TLSDESC_GOT: u32 = 0x6ffffef7;
pub const DT_GNU_CONFLICT: u32 = 0x6ffffef8;
pub const DT_GNU_LIBLIST: u32 = 0x6ffffef9;
pub const DT_CONFIG: u32 = 0x6ffffefa;
pub const DT_DEPAUDIT: u32 = 0x6ffffefb;
pub const DT_AUDIT: u32 = 0x6ffffefc;
pub const DT_PLTPAD: u32 = 0x6ffffefd;
pub const DT_MOVETAB: u32 = 0x6ffffefe;
pub const DT_SYMINFO: u32 = 0x6ffffeff;
pub const DT_ADDRRNGHI: u32 = 0x6ffffeff;


pub const DT_ADDRNUM: u32 = 11;


/* The versioning entry types.  The next are defined as part of the
   GNU extension.  */


pub const DT_VERSYM: u32 = 0x6ffffff0;


pub const DT_RELACOUNT: u32 = 0x6ffffff9;
pub const DT_RELCOUNT: u32 = 0x6ffffffa;


/* These were chosen by Sun.  */


pub const DT_FLAGS_1: u32 = 0x6ffffffb;




pub const DT_VERDEFNUM: u32 = 0x6ffffffd;




pub const DT_VERNEEDNUM: u32 = 0x6fffffff;


pub const DT_VERSIONTAGNUM: u32 = 16;


/* Sun added these machine-independent extensions in the "processor-specific"
   range.  Be compatible.  */


pub const DT_AUXILIARY: u32 = 0x7ffffffd;
pub const DT_FILTER: u32 = 0x7fffffff;


pub const DT_EXTRANUM: u32 = 3;


/* Values of `d_un.d_val' in the DT_FLAGS entry.  */


pub const DF_ORIGIN: u32 = 0x00000001;
pub const DF_SYMBOLIC: u32 = 0x00000002;
pub const DF_TEXTREL: u32 = 0x00000004;
pub const DF_BIND_NOW: u32 = 0x00000008;
pub const DF_STATIC_TLS: u32 = 0x00000010;


/* State flags selectable in the `d_un.d_val' element of the DT_FLAGS_1
   entry in the dynamic section.  */


pub const DF_1_NOW: u32 = 0x00000001;
pub const DF_1_GLOBAL: u32 = 0x00000002;
pub const DF_1_GROUP: u32 = 0x00000004;
pub const DF_1_NODELETE: u32 = 0x00000008;
pub const DF_1_LOADFLTR: u32 = 0x00000010;
pub const DF_1_INITFIRST: u32 = 0x00000020;
pub const DF_1_NOOPEN: u32 = 0x00000040;
pub const DF_1_ORIGIN: u32 = 0x00000080;
pub const DF_1_DIRECT: u32 = 0x00000100;
pub const DF_1_TRANS: u32 = 0x00000200;
pub const DF_1_INTERPOSE: u32 = 0x00000400;
pub const DF_1_NODEFLIB: u32 = 0x00000800;
pub const DF_1_NODUMP: u32 = 0x00001000;
pub const DF_1_CONFALT: u32 = 0x00002000;
pub const DF_1_ENDFILTEE: u32 = 0x00004000;
pub const DF_1_DISPRELDNE: u32 = 0x00008000;
pub const DF_1_DISPRELPND: u32 = 0x00010000;


/* Flags for the feature selection in DT_FEATURE_1.  */


pub const DTF_1_PARINIT: u32 = 0x00000001;
pub const DTF_1_CONFEXP: u32 = 0x00000002;


/* Flags in the DT_POSFLAG_1 entry effecting only the next DT_* entry.  */


pub const DF_P1_LAZYLOAD: u32 = 0x00000001;






/* Version definition sections.  */






















































/* Legal values for vd_version (version revision).  */


pub const VER_DEF_NONE: u32 = 0;
pub const VER_DEF_CURRENT: u32 = 1;
pub const VER_DEF_NUM: u32 = 2;


/* Legal values for vd_flags (version information flags).  */


pub const VER_FLG_BASE: u32 = 0x1;
pub const VER_FLG_WEAK: u32 = 0x2;


/* Versym symbol index values.  */


pub const VER_NDX_LOCAL: u32 = 0;
pub const VER_NDX_GLOBAL: u32 = 1;
pub const VER_NDX_LORESERVE: u32 = 0xff00;
pub const VER_NDX_ELIMINATE: u32 = 0xff01;


/* Auxialiary version information.  */


































/* Version dependency section.  */


















































/* Legal values for vn_version (version revision).  */


pub const VER_NEED_NONE: u32 = 0;
pub const VER_NEED_CURRENT: u32 = 1;
pub const VER_NEED_NUM: u32 = 2;


/* Auxiliary needed version information.  */














































/* Legal values for vna_flags.  */


pub const VER_FLG_WEAK: u32 = 0x2;




/* Auxiliary vector.  */




/* This vector is normally only used by the program interpreter.  The
   usual definition in an ABI supplement uses the name auxv_t.  The
   vector is not usually defined in a standard <elf.h> file, but it
   can't hurt.  We rename it to avoid conflicts.  The sizes of these
   types are an arrangement between the exec server and the program
   interpreter, so we don't fully specify them here.  */




















































/* Legal values for a_type (entry type).  */




pub const AT_NULL: u32 = 0;
pub const AT_IGNORE: u32 = 1;
pub const AT_EXECFD: u32 = 2;
pub const AT_PHDR: u32 = 3;
pub const AT_PHENT: u32 = 4;
pub const AT_PHNUM: u32 = 5;
pub const AT_PAGESZ: u32 = 6;
pub const AT_BASE: u32 = 7;
pub const AT_FLAGS: u32 = 8;
pub const AT_ENTRY: u32 = 9;
pub const AT_NOTELF: u32 = 10;
pub const AT_UID: u32 = 11;
pub const AT_EUID: u32 = 12;
pub const AT_GID: u32 = 13;
pub const AT_EGID: u32 = 14;
pub const AT_CLKTCK: u32 = 17;


/* Some more special a_type values describing the hardware.  */


pub const AT_PLATFORM: u32 = 15;






/* This entry gives some information about the FPU initialization
   performed by the kernel.  */


pub const AT_FPUCW: u32 = 18;


/* Cache block sizes.  */


pub const AT_DCACHEBSIZE: u32 = 19;
pub const AT_ICACHEBSIZE: u32 = 20;
pub const AT_UCACHEBSIZE: u32 = 21;


/* A special ignored value for PPC, used by the kernel to control the
   interpretation of the AUXV. Must be > 16.  */


pub const AT_IGNOREPPC: u32 = 22;


pub const AT_SECURE: u32 = 23;


pub const AT_BASE_PLATFORM: u32 = 24;


pub const AT_RANDOM: u32 = 25;


pub const AT_EXECFN: u32 = 31;


/* Pointer to the global system page used for system calls and other
   nice things.  */


pub const AT_SYSINFO: u32 = 32;
pub const AT_SYSINFO_EHDR: u32 = 33;


/* Shapes of the caches.  Bits 0-3 contains associativity; bits 4-7 contains
   log2 of line size; mask those to get cache size.  */


pub const AT_L1I_CACHESHAPE: u32 = 34;
pub const AT_L1D_CACHESHAPE: u32 = 35;
pub const AT_L2_CACHESHAPE: u32 = 36;
pub const AT_L3_CACHESHAPE: u32 = 37;


/* Note section contents.  Each entry in the note section begins with
   a header of a fixed form.  */
































/* Known names of notes.  */




/* Solaris entries in the note section have this name.  */






/* Note entries for GNU systems have this name.  */








/* Defined types of notes for Solaris.  */




/* Value of descriptor (one word) is desired pagesize for the binary.  */


pub const ELF_NOTE_PAGESIZE_HINT: u32 = 1;




/* Defined note types for GNU systems.  */




/* ABI information.  The descriptor consists of words:
   word 0: OS descriptor
   word 1: major version of the ABI
   word 2: minor version of the ABI
   word 3: subminor version of the ABI
*/


pub const NT_GNU_ABI_TAG: u32 = 1;
pub const ELF_NOTE_ABI: u32 = NT_GNU_ABI_TAG;


/* Known OSes.  These values can appear in word 0 of an
   NT_GNU_ABI_TAG note section entry.  */


pub const ELF_NOTE_OS_LINUX: u32 = 0;
pub const ELF_NOTE_OS_GNU: u32 = 1;
pub const ELF_NOTE_OS_SOLARIS2: u32 = 2;
pub const ELF_NOTE_OS_FREEBSD: u32 = 3;


/* Synthetic hwcap information.  The descriptor begins with two words:
   word 0: number of entries
   word 1: bitmask of enabled entries
   Then follow variable-length entries, one byte followed by a
   '\0'-terminated hwcap name string.  The byte gives the bit
   number to test if enabled, (1U << bit) & bitmask.  */


pub const NT_GNU_HWCAP: u32 = 2;


/* Build ID bits as generated by ld --build-id.
   The descriptor consists of any nonzero number of bytes.  */


pub const NT_GNU_BUILD_ID: u32 = 3;


/* Version note generated by GNU gold containing a version string.  */


pub const NT_GNU_GOLD_VERSION: u32 = 4;




/* Move records.  */






































/* Macro to construct move records.  */




















/* Motorola 68k specific definitions.  */




/* Values for Elf32_Ehdr.e_flags.  */


pub const EF_CPU32: u32 = 0x00810000;


/* m68k relocs.  */




pub const R_68K_NONE: u32 = 0;
pub const R_68K_32: u32 = 1;
pub const R_68K_16: u32 = 2;
pub const R_68K_8: u32 = 3;
pub const R_68K_PC32: u32 = 4;
pub const R_68K_PC16: u32 = 5;
pub const R_68K_PC8: u32 = 6;
pub const R_68K_GOT32: u32 = 7;
pub const R_68K_GOT16: u32 = 8;
pub const R_68K_GOT8: u32 = 9;
pub const R_68K_GOT32O: u32 = 10;
pub const R_68K_GOT16O: u32 = 11;
pub const R_68K_GOT8O: u32 = 12;
pub const R_68K_PLT32: u32 = 13;
pub const R_68K_PLT16: u32 = 14;
pub const R_68K_PLT8: u32 = 15;
pub const R_68K_PLT32O: u32 = 16;
pub const R_68K_PLT16O: u32 = 17;
pub const R_68K_PLT8O: u32 = 18;
pub const R_68K_COPY: u32 = 19;
pub const R_68K_GLOB_DAT: u32 = 20;
pub const R_68K_JMP_SLOT: u32 = 21;
pub const R_68K_RELATIVE: u32 = 22;
pub const R_68K_TLS_GD32: u32 = 25;
pub const R_68K_TLS_GD16: u32 = 26;
pub const R_68K_TLS_GD8: u32 = 27;
pub const R_68K_TLS_LDM32: u32 = 28;
pub const R_68K_TLS_LDM16: u32 = 29;
pub const R_68K_TLS_LDM8: u32 = 30;
pub const R_68K_TLS_LDO32: u32 = 31;
pub const R_68K_TLS_LDO16: u32 = 32;
pub const R_68K_TLS_LDO8: u32 = 33;
pub const R_68K_TLS_IE32: u32 = 34;
pub const R_68K_TLS_IE16: u32 = 35;
pub const R_68K_TLS_IE8: u32 = 36;












pub const R_68K_TLS_DTPMOD32: u32 = 40;
pub const R_68K_TLS_DTPREL32: u32 = 41;
pub const R_68K_TLS_TPREL32: u32 = 42;
/* Keep this the last entry.  */


pub const R_68K_NUM: u32 = 43;


/* Intel 80386 specific definitions.  */




/* i386 relocs.  */




pub const R_386_NONE: u32 = 0;
pub const R_386_32: u32 = 1;
pub const R_386_PC32: u32 = 2;
pub const R_386_GOT32: u32 = 3;
pub const R_386_PLT32: u32 = 4;
pub const R_386_COPY: u32 = 5;
pub const R_386_GLOB_DAT: u32 = 6;
pub const R_386_JMP_SLOT: u32 = 7;
pub const R_386_RELATIVE: u32 = 8;
pub const R_386_GOTOFF: u32 = 9;
pub const R_386_GOTPC: u32 = 10;
pub const R_386_32PLT: u32 = 11;
pub const R_386_TLS_TPOFF: u32 = 14;






















pub const R_386_16: u32 = 20;
pub const R_386_PC16: u32 = 21;
pub const R_386_8: u32 = 22;
pub const R_386_PC8: u32 = 23;




pub const R_386_TLS_GD_PUSH: u32 = 25;




pub const R_386_TLS_GD_POP: u32 = 27;




pub const R_386_TLS_LDM_PUSH: u32 = 29;




pub const R_386_TLS_LDM_POP: u32 = 31;
pub const R_386_TLS_LDO_32: u32 = 32;








pub const R_386_TLS_DTPMOD32: u32 = 35;
pub const R_386_TLS_DTPOFF32: u32 = 36;
pub const R_386_TLS_TPOFF32: u32 = 37;
/* 38? */


pub const R_386_TLS_GOTDESC: u32 = 39;














pub const R_386_IRELATIVE: u32 = 42;
/* Keep this the last entry.  */


pub const R_386_NUM: u32 = 43;


/* SUN SPARC specific definitions.  */




/* Legal values for ST_TYPE subfield of st_info (symbol type).  */




pub const STT_SPARC_REGISTER: u32 = 13;


/* Values for Elf64_Ehdr.e_flags.  */




pub const EF_SPARCV9_MM: u32 = 3;
pub const EF_SPARCV9_TSO: u32 = 0;
pub const EF_SPARCV9_PSO: u32 = 1;
pub const EF_SPARCV9_RMO: u32 = 2;
pub const EF_SPARC_LEDATA: u32 = 0x800000;
pub const EF_SPARC_EXT_MASK: u32 = 0xFFFF00;
pub const EF_SPARC_32PLUS: u32 = 0x000100;
pub const EF_SPARC_SUN_US1: u32 = 0x000200;
pub const EF_SPARC_HAL_R1: u32 = 0x000400;
pub const EF_SPARC_SUN_US3: u32 = 0x000800;


/* SPARC relocs.  */




pub const R_SPARC_NONE: u32 = 0;
pub const R_SPARC_8: u32 = 1;
pub const R_SPARC_16: u32 = 2;
pub const R_SPARC_32: u32 = 3;
pub const R_SPARC_DISP8: u32 = 4;
pub const R_SPARC_DISP16: u32 = 5;
pub const R_SPARC_DISP32: u32 = 6;
pub const R_SPARC_WDISP30: u32 = 7;
pub const R_SPARC_WDISP22: u32 = 8;
pub const R_SPARC_HI22: u32 = 9;
pub const R_SPARC_22: u32 = 10;
pub const R_SPARC_13: u32 = 11;
pub const R_SPARC_LO10: u32 = 12;
pub const R_SPARC_GOT10: u32 = 13;
pub const R_SPARC_GOT13: u32 = 14;
pub const R_SPARC_GOT22: u32 = 15;
pub const R_SPARC_PC10: u32 = 16;
pub const R_SPARC_PC22: u32 = 17;
pub const R_SPARC_WPLT30: u32 = 18;
pub const R_SPARC_COPY: u32 = 19;
pub const R_SPARC_GLOB_DAT: u32 = 20;
pub const R_SPARC_JMP_SLOT: u32 = 21;
pub const R_SPARC_RELATIVE: u32 = 22;
pub const R_SPARC_UA32: u32 = 23;


/* Additional Sparc64 relocs.  */




pub const R_SPARC_PLT32: u32 = 24;
pub const R_SPARC_HIPLT22: u32 = 25;
pub const R_SPARC_LOPLT10: u32 = 26;
pub const R_SPARC_PCPLT32: u32 = 27;
pub const R_SPARC_PCPLT22: u32 = 28;
pub const R_SPARC_PCPLT10: u32 = 29;
pub const R_SPARC_10: u32 = 30;
pub const R_SPARC_11: u32 = 31;
pub const R_SPARC_64: u32 = 32;
pub const R_SPARC_OLO10: u32 = 33;
pub const R_SPARC_HH22: u32 = 34;
pub const R_SPARC_HM10: u32 = 35;
pub const R_SPARC_LM22: u32 = 36;
pub const R_SPARC_PC_HH22: u32 = 37;
pub const R_SPARC_PC_HM10: u32 = 38;
pub const R_SPARC_PC_LM22: u32 = 39;
pub const R_SPARC_WDISP16: u32 = 40;
pub const R_SPARC_WDISP19: u32 = 41;
pub const R_SPARC_GLOB_JMP: u32 = 42;
pub const R_SPARC_7: u32 = 43;
pub const R_SPARC_5: u32 = 44;
pub const R_SPARC_6: u32 = 45;
pub const R_SPARC_DISP64: u32 = 46;
pub const R_SPARC_PLT64: u32 = 47;
pub const R_SPARC_HIX22: u32 = 48;
pub const R_SPARC_LOX10: u32 = 49;
pub const R_SPARC_H44: u32 = 50;
pub const R_SPARC_M44: u32 = 51;
pub const R_SPARC_L44: u32 = 52;
pub const R_SPARC_REGISTER: u32 = 53;
pub const R_SPARC_UA64: u32 = 54;
pub const R_SPARC_UA16: u32 = 55;
pub const R_SPARC_TLS_GD_HI22: u32 = 56;
pub const R_SPARC_TLS_GD_LO10: u32 = 57;
pub const R_SPARC_TLS_GD_ADD: u32 = 58;
pub const R_SPARC_TLS_GD_CALL: u32 = 59;
pub const R_SPARC_TLS_LDM_HI22: u32 = 60;
pub const R_SPARC_TLS_LDM_LO10: u32 = 61;
pub const R_SPARC_TLS_LDM_ADD: u32 = 62;
pub const R_SPARC_TLS_LDM_CALL: u32 = 63;
pub const R_SPARC_TLS_LDO_HIX22: u32 = 64;
pub const R_SPARC_TLS_LDO_LOX10: u32 = 65;
pub const R_SPARC_TLS_LDO_ADD: u32 = 66;
pub const R_SPARC_TLS_IE_HI22: u32 = 67;
pub const R_SPARC_TLS_IE_LO10: u32 = 68;
pub const R_SPARC_TLS_IE_LD: u32 = 69;
pub const R_SPARC_TLS_IE_LDX: u32 = 70;
pub const R_SPARC_TLS_IE_ADD: u32 = 71;
pub const R_SPARC_TLS_LE_HIX22: u32 = 72;
pub const R_SPARC_TLS_LE_LOX10: u32 = 73;
pub const R_SPARC_TLS_DTPMOD32: u32 = 74;
pub const R_SPARC_TLS_DTPMOD64: u32 = 75;
pub const R_SPARC_TLS_DTPOFF32: u32 = 76;
pub const R_SPARC_TLS_DTPOFF64: u32 = 77;
pub const R_SPARC_TLS_TPOFF32: u32 = 78;
pub const R_SPARC_TLS_TPOFF64: u32 = 79;
pub const R_SPARC_GOTDATA_HIX22: u32 = 80;
pub const R_SPARC_GOTDATA_LOX10: u32 = 81;
pub const R_SPARC_GOTDATA_OP_HIX22: u32 = 82;
pub const R_SPARC_GOTDATA_OP_LOX10: u32 = 83;
pub const R_SPARC_GOTDATA_OP: u32 = 84;
pub const R_SPARC_H34: u32 = 85;
pub const R_SPARC_SIZE32: u32 = 86;
pub const R_SPARC_SIZE64: u32 = 87;
pub const R_SPARC_WDISP10: u32 = 88;
pub const R_SPARC_JMP_IREL: u32 = 248;
pub const R_SPARC_IRELATIVE: u32 = 249;
pub const R_SPARC_GNU_VTINHERIT: u32 = 250;
pub const R_SPARC_GNU_VTENTRY: u32 = 251;
pub const R_SPARC_REV32: u32 = 252;
/* Keep this the last entry.  */


pub const R_SPARC_NUM: u32 = 253;


/* For Sparc64, legal values for d_tag of Elf64_Dyn.  */




pub const DT_SPARC_REGISTER: u32 = 0x70000001;
pub const DT_SPARC_NUM: u32 = 2;


/* MIPS R3000 specific definitions.  */




/* Legal values for e_flags field of Elf32_Ehdr.  */




pub const EF_MIPS_NOREORDER: u32 = 1;
pub const EF_MIPS_PIC: u32 = 2;
pub const EF_MIPS_CPIC: u32 = 4;
pub const EF_MIPS_XGOT: u32 = 8;
pub const EF_MIPS_64BIT_WHIRL: u32 = 16;
pub const EF_MIPS_ABI2: u32 = 32;
pub const EF_MIPS_ABI_ON32: u32 = 64;
pub const EF_MIPS_ARCH: u32 = 0xf0000000;


/* Legal values for MIPS architecture level.  */




pub const EF_MIPS_ARCH_1: u32 = 0x00000000;
pub const EF_MIPS_ARCH_2: u32 = 0x10000000;
pub const EF_MIPS_ARCH_3: u32 = 0x20000000;
pub const EF_MIPS_ARCH_4: u32 = 0x30000000;
pub const EF_MIPS_ARCH_5: u32 = 0x40000000;
pub const EF_MIPS_ARCH_32: u32 = 0x60000000;
pub const EF_MIPS_ARCH_64: u32 = 0x70000000;


/* The following are non-official names and should not be used.  */




pub const E_MIPS_ARCH_1: u32 = 0x00000000;
pub const E_MIPS_ARCH_2: u32 = 0x10000000;
pub const E_MIPS_ARCH_3: u32 = 0x20000000;
pub const E_MIPS_ARCH_4: u32 = 0x30000000;
pub const E_MIPS_ARCH_5: u32 = 0x40000000;
pub const E_MIPS_ARCH_32: u32 = 0x60000000;
pub const E_MIPS_ARCH_64: u32 = 0x70000000;


/* Special section indices.  */




pub const SHN_MIPS_ACOMMON: u32 = 0xff00;
pub const SHN_MIPS_TEXT: u32 = 0xff01;
pub const SHN_MIPS_DATA: u32 = 0xff02;
pub const SHN_MIPS_SCOMMON: u32 = 0xff03;
pub const SHN_MIPS_SUNDEFINED: u32 = 0xff04;


/* Legal values for sh_type field of Elf32_Shdr.  */




pub const SHT_MIPS_LIBLIST: u32 = 0x70000000;
pub const SHT_MIPS_MSYM: u32 = 0x70000001;
pub const SHT_MIPS_CONFLICT: u32 = 0x70000002;
pub const SHT_MIPS_GPTAB: u32 = 0x70000003;
pub const SHT_MIPS_UCODE: u32 = 0x70000004;
pub const SHT_MIPS_DEBUG: u32 = 0x70000005;
pub const SHT_MIPS_REGINFO: u32 = 0x70000006;
pub const SHT_MIPS_PACKAGE: u32 = 0x70000007;
pub const SHT_MIPS_PACKSYM: u32 = 0x70000008;
pub const SHT_MIPS_RELD: u32 = 0x70000009;
pub const SHT_MIPS_IFACE: u32 = 0x7000000b;
pub const SHT_MIPS_CONTENT: u32 = 0x7000000c;
pub const SHT_MIPS_OPTIONS: u32 = 0x7000000d;
pub const SHT_MIPS_SHDR: u32 = 0x70000010;
pub const SHT_MIPS_FDESC: u32 = 0x70000011;
pub const SHT_MIPS_EXTSYM: u32 = 0x70000012;
pub const SHT_MIPS_DENSE: u32 = 0x70000013;
pub const SHT_MIPS_PDESC: u32 = 0x70000014;
pub const SHT_MIPS_LOCSYM: u32 = 0x70000015;
pub const SHT_MIPS_AUXSYM: u32 = 0x70000016;
pub const SHT_MIPS_OPTSYM: u32 = 0x70000017;
pub const SHT_MIPS_LOCSTR: u32 = 0x70000018;
pub const SHT_MIPS_LINE: u32 = 0x70000019;
pub const SHT_MIPS_RFDESC: u32 = 0x7000001a;
pub const SHT_MIPS_DELTASYM: u32 = 0x7000001b;
pub const SHT_MIPS_DELTAINST: u32 = 0x7000001c;
pub const SHT_MIPS_DELTACLASS: u32 = 0x7000001d;
pub const SHT_MIPS_DWARF: u32 = 0x7000001e;
pub const SHT_MIPS_DELTADECL: u32 = 0x7000001f;
pub const SHT_MIPS_SYMBOL_LIB: u32 = 0x70000020;
pub const SHT_MIPS_EVENTS: u32 = 0x70000021;
pub const SHT_MIPS_TRANSLATE: u32 = 0x70000022;
pub const SHT_MIPS_PIXIE: u32 = 0x70000023;
pub const SHT_MIPS_XLATE: u32 = 0x70000024;
pub const SHT_MIPS_XLATE_DEBUG: u32 = 0x70000025;
pub const SHT_MIPS_WHIRL: u32 = 0x70000026;
pub const SHT_MIPS_EH_REGION: u32 = 0x70000027;
pub const SHT_MIPS_XLATE_OLD: u32 = 0x70000028;
pub const SHT_MIPS_PDR_EXCEPTION: u32 = 0x70000029;


/* Legal values for sh_flags field of Elf32_Shdr.  */




pub const SHF_MIPS_GPREL: u32 = 0x10000000;
pub const SHF_MIPS_MERGE: u32 = 0x20000000;
pub const SHF_MIPS_ADDR: u32 = 0x40000000;
pub const SHF_MIPS_STRINGS: u32 = 0x80000000;
pub const SHF_MIPS_NOSTRIP: u32 = 0x08000000;
pub const SHF_MIPS_LOCAL: u32 = 0x04000000;
pub const SHF_MIPS_NAMES: u32 = 0x02000000;
pub const SHF_MIPS_NODUPE: u32 = 0x01000000;




/* Symbol tables.  */




/* MIPS specific values for `st_other'.  */


pub const STO_MIPS_DEFAULT: u32 = 0x0;
pub const STO_MIPS_INTERNAL: u32 = 0x1;
pub const STO_MIPS_HIDDEN: u32 = 0x2;
pub const STO_MIPS_PROTECTED: u32 = 0x3;
pub const STO_MIPS_PLT: u32 = 0x8;
pub const STO_MIPS_SC_ALIGN_UNUSED: u32 = 0xff;


/* MIPS specific values for `st_info'.  */


pub const STB_MIPS_SPLIT_COMMON: u32 = 13;


/* Entries found in sections of type SHT_MIPS_GPTAB.  */
































/* Entry found in sections of type SHT_MIPS_REGINFO.  */


















/* Entries found in sections of type SHT_MIPS_OPTIONS.  */
























/* Values for `kind' field in Elf_Options.  */




pub const ODK_NULL: u32 = 0;
pub const ODK_REGINFO: u32 = 1;
pub const ODK_EXCEPTIONS: u32 = 2;
pub const ODK_PAD: u32 = 3;
pub const ODK_HWPATCH: u32 = 4;
pub const ODK_FILL: u32 = 5;
pub const ODK_TAGS: u32 = 6;
pub const ODK_HWAND: u32 = 7;
pub const ODK_HWOR: u32 = 8;


/* Values for `info' in Elf_Options for ODK_EXCEPTIONS entries.  */




pub const OEX_FPU_MIN: u32 = 0x1f;
pub const OEX_FPU_MAX: u32 = 0x1f00;
pub const OEX_PAGE0: u32 = 0x10000;
pub const OEX_SMM: u32 = 0x20000;
pub const OEX_FPDBUG: u32 = 0x40000;
pub const OEX_PRECISEFP: u32 = OEX_FPDBUG;
pub const OEX_DISMISS: u32 = 0x80000;


pub const OEX_FPU_INVAL: u32 = 0x10;
pub const OEX_FPU_DIV0: u32 = 0x08;
pub const OEX_FPU_OFLO: u32 = 0x04;
pub const OEX_FPU_UFLO: u32 = 0x02;
pub const OEX_FPU_INEX: u32 = 0x01;


/* Masks for `info' in Elf_Options for an ODK_HWPATCH entry.  */




pub const OHW_R4KEOP: u32 = 0x1;
pub const OHW_R8KPFETCH: u32 = 0x2;
pub const OHW_R5KEOP: u32 = 0x4;
pub const OHW_R5KCVTL: u32 = 0x8;


pub const OPAD_PREFIX: u32 = 0x1;
pub const OPAD_POSTFIX: u32 = 0x2;
pub const OPAD_SYMBOL: u32 = 0x4;


/* Entry found in `.options' section.  */
















/* Masks for `info' in ElfOptions for ODK_HWAND and ODK_HWOR entries.  */




pub const OHWA0_R4KEOP_CHECKED: u32 = 0x00000001;
pub const OHWA1_R4KEOP_CLEAN: u32 = 0x00000002;


/* MIPS relocs.  */




pub const R_MIPS_NONE: u32 = 0;
pub const R_MIPS_16: u32 = 1;
pub const R_MIPS_32: u32 = 2;
pub const R_MIPS_REL32: u32 = 3;
pub const R_MIPS_26: u32 = 4;
pub const R_MIPS_HI16: u32 = 5;
pub const R_MIPS_LO16: u32 = 6;
pub const R_MIPS_GPREL16: u32 = 7;
pub const R_MIPS_LITERAL: u32 = 8;
pub const R_MIPS_GOT16: u32 = 9;
pub const R_MIPS_PC16: u32 = 10;
pub const R_MIPS_CALL16: u32 = 11;
pub const R_MIPS_GPREL32: u32 = 12;


pub const R_MIPS_SHIFT5: u32 = 16;
pub const R_MIPS_SHIFT6: u32 = 17;
pub const R_MIPS_64: u32 = 18;
pub const R_MIPS_GOT_DISP: u32 = 19;
pub const R_MIPS_GOT_PAGE: u32 = 20;
pub const R_MIPS_GOT_OFST: u32 = 21;
pub const R_MIPS_GOT_HI16: u32 = 22;
pub const R_MIPS_GOT_LO16: u32 = 23;
pub const R_MIPS_SUB: u32 = 24;
pub const R_MIPS_INSERT_A: u32 = 25;
pub const R_MIPS_INSERT_B: u32 = 26;
pub const R_MIPS_DELETE: u32 = 27;
pub const R_MIPS_HIGHER: u32 = 28;
pub const R_MIPS_HIGHEST: u32 = 29;
pub const R_MIPS_CALL_HI16: u32 = 30;
pub const R_MIPS_CALL_LO16: u32 = 31;
pub const R_MIPS_SCN_DISP: u32 = 32;
pub const R_MIPS_REL16: u32 = 33;
pub const R_MIPS_ADD_IMMEDIATE: u32 = 34;
pub const R_MIPS_PJUMP: u32 = 35;
pub const R_MIPS_RELGOT: u32 = 36;
pub const R_MIPS_JALR: u32 = 37;
pub const R_MIPS_TLS_DTPMOD32: u32 = 38;
pub const R_MIPS_TLS_DTPREL32: u32 = 39;
pub const R_MIPS_TLS_DTPMOD64: u32 = 40;
pub const R_MIPS_TLS_DTPREL64: u32 = 41;
pub const R_MIPS_TLS_GD: u32 = 42;
pub const R_MIPS_TLS_LDM: u32 = 43;
pub const R_MIPS_TLS_DTPREL_HI16: u32 = 44;
pub const R_MIPS_TLS_DTPREL_LO16: u32 = 45;
pub const R_MIPS_TLS_GOTTPREL: u32 = 46;
pub const R_MIPS_TLS_TPREL32: u32 = 47;
pub const R_MIPS_TLS_TPREL64: u32 = 48;
pub const R_MIPS_TLS_TPREL_HI16: u32 = 49;
pub const R_MIPS_TLS_TPREL_LO16: u32 = 50;
pub const R_MIPS_GLOB_DAT: u32 = 51;
pub const R_MIPS_COPY: u32 = 126;
pub const R_MIPS_JUMP_SLOT: u32 = 127;
/* Keep this the last entry.  */


pub const R_MIPS_NUM: u32 = 128;


/* Legal values for p_type field of Elf32_Phdr.  */




pub const PT_MIPS_REGINFO: u32 = 0x70000000;
pub const PT_MIPS_RTPROC: u32 = 0x70000001;
pub const PT_MIPS_OPTIONS: u32 = 0x70000002;


/* Special program header types.  */




pub const PF_MIPS_LOCAL: u32 = 0x10000000;


/* Legal values for d_tag field of Elf32_Dyn.  */




pub const DT_MIPS_RLD_VERSION: u32 = 0x70000001;
pub const DT_MIPS_TIME_STAMP: u32 = 0x70000002;
pub const DT_MIPS_ICHECKSUM: u32 = 0x70000003;
pub const DT_MIPS_IVERSION: u32 = 0x70000004;
pub const DT_MIPS_FLAGS: u32 = 0x70000005;
pub const DT_MIPS_BASE_ADDRESS: u32 = 0x70000006;
pub const DT_MIPS_MSYM: u32 = 0x70000007;
pub const DT_MIPS_CONFLICT: u32 = 0x70000008;
pub const DT_MIPS_LIBLIST: u32 = 0x70000009;
pub const DT_MIPS_LOCAL_GOTNO: u32 = 0x7000000a;
pub const DT_MIPS_CONFLICTNO: u32 = 0x7000000b;
pub const DT_MIPS_LIBLISTNO: u32 = 0x70000010;
pub const DT_MIPS_SYMTABNO: u32 = 0x70000011;
pub const DT_MIPS_UNREFEXTNO: u32 = 0x70000012;
pub const DT_MIPS_GOTSYM: u32 = 0x70000013;
pub const DT_MIPS_HIPAGENO: u32 = 0x70000014;
pub const DT_MIPS_RLD_MAP: u32 = 0x70000016;
pub const DT_MIPS_DELTA_CLASS: u32 = 0x70000017;




pub const DT_MIPS_DELTA_INSTANCE: u32 = 0x70000019;




pub const DT_MIPS_DELTA_RELOC: u32 = 0x7000001b;




















pub const DT_MIPS_CXX_FLAGS: u32 = 0x70000022;
pub const DT_MIPS_PIXIE_INIT: u32 = 0x70000023;
pub const DT_MIPS_SYMBOL_LIB: u32 = 0x70000024;
pub const DT_MIPS_LOCALPAGE_GOTIDX: u32 = 0x70000025;
pub const DT_MIPS_LOCAL_GOTIDX: u32 = 0x70000026;
pub const DT_MIPS_HIDDEN_GOTIDX: u32 = 0x70000027;
pub const DT_MIPS_PROTECTED_GOTIDX: u32 = 0x70000028;
pub const DT_MIPS_OPTIONS: u32 = 0x70000029;
pub const DT_MIPS_INTERFACE: u32 = 0x7000002a;
pub const DT_MIPS_DYNSTR_ALIGN: u32 = 0x7000002b;
pub const DT_MIPS_INTERFACE_SIZE: u32 = 0x7000002c;








pub const DT_MIPS_COMPACT_SIZE: u32 = 0x7000002f;
pub const DT_MIPS_GP_VALUE: u32 = 0x70000030;
pub const DT_MIPS_AUX_DYNAMIC: u32 = 0x70000031;
/* The address of .got.plt in an executable using the new non-PIC ABI.  */


pub const DT_MIPS_PLTGOT: u32 = 0x70000032;
/* The base of the PLT in an executable using the new non-PIC ABI if that
   PLT is writable.  For a non-writable PLT, this is omitted or has a zero
   value.  */


pub const DT_MIPS_RWPLT: u32 = 0x70000034;
pub const DT_MIPS_NUM: u32 = 0x35;


/* Legal values for DT_MIPS_FLAGS Elf32_Dyn entry.  */




pub const RHF_NONE: u32 = 0;
































/* Entries found in sections of type SHT_MIPS_LIBLIST.  */










































/* Legal values for l_flags.  */




pub const LL_NONE: u32 = 0;














/* Entries found in sections of type SHT_MIPS_CONFLICT.  */




#[allow(non_camel_case_types)]
pub type Elf32_Conflict = Elf32_Addr;






/* HPPA specific definitions.  */




/* Legal values for e_flags field of Elf32_Ehdr.  */




pub const EF_PARISC_TRAPNIL: u32 = 0x00010000;
pub const EF_PARISC_EXT: u32 = 0x00020000;
pub const EF_PARISC_LSB: u32 = 0x00040000;
pub const EF_PARISC_WIDE: u32 = 0x00080000;




pub const EF_PARISC_LAZYSWAP: u32 = 0x00400000;
pub const EF_PARISC_ARCH: u32 = 0x0000ffff;


/* Defined values for `e_flags & EF_PARISC_ARCH' are:  */




pub const EFA_PARISC_1_0: u32 = 0x020b;
pub const EFA_PARISC_1_1: u32 = 0x0210;
pub const EFA_PARISC_2_0: u32 = 0x0214;


/* Additional section indeces.  */








pub const SHN_PARISC_HUGE_COMMON: u32 = 0xff01;


/* Legal values for sh_type field of Elf32_Shdr.  */




pub const SHT_PARISC_EXT: u32 = 0x70000000;
pub const SHT_PARISC_UNWIND: u32 = 0x70000001;
pub const SHT_PARISC_DOC: u32 = 0x70000002;


/* Legal values for sh_flags field of Elf32_Shdr.  */




pub const SHF_PARISC_SHORT: u32 = 0x20000000;
pub const SHF_PARISC_HUGE: u32 = 0x40000000;
pub const SHF_PARISC_SBP: u32 = 0x80000000;


/* Legal values for ST_TYPE subfield of st_info (symbol type).  */




pub const STT_PARISC_MILLICODE: u32 = 13;








/* HPPA relocs.  */




pub const R_PARISC_NONE: u32 = 0;
pub const R_PARISC_DIR32: u32 = 1;
pub const R_PARISC_DIR21L: u32 = 2;
pub const R_PARISC_DIR17R: u32 = 3;
pub const R_PARISC_DIR17F: u32 = 4;
pub const R_PARISC_DIR14R: u32 = 6;
pub const R_PARISC_PCREL32: u32 = 9;
pub const R_PARISC_PCREL21L: u32 = 10;
pub const R_PARISC_PCREL17R: u32 = 11;
pub const R_PARISC_PCREL17F: u32 = 12;
pub const R_PARISC_PCREL14R: u32 = 14;
pub const R_PARISC_DPREL21L: u32 = 18;
pub const R_PARISC_DPREL14R: u32 = 22;
pub const R_PARISC_GPREL21L: u32 = 26;
pub const R_PARISC_GPREL14R: u32 = 30;
pub const R_PARISC_LTOFF21L: u32 = 34;
pub const R_PARISC_LTOFF14R: u32 = 38;
pub const R_PARISC_SECREL32: u32 = 41;
pub const R_PARISC_SEGBASE: u32 = 48;
pub const R_PARISC_SEGREL32: u32 = 49;
pub const R_PARISC_PLTOFF21L: u32 = 50;
pub const R_PARISC_PLTOFF14R: u32 = 54;
pub const R_PARISC_LTOFF_FPTR32: u32 = 57;
pub const R_PARISC_LTOFF_FPTR21L: u32 = 58;
pub const R_PARISC_LTOFF_FPTR14R: u32 = 62;
pub const R_PARISC_FPTR64: u32 = 64;
pub const R_PARISC_PLABEL32: u32 = 65;
pub const R_PARISC_PLABEL21L: u32 = 66;
pub const R_PARISC_PLABEL14R: u32 = 70;
pub const R_PARISC_PCREL64: u32 = 72;
pub const R_PARISC_PCREL22F: u32 = 74;
pub const R_PARISC_PCREL14WR: u32 = 75;
pub const R_PARISC_PCREL14DR: u32 = 76;
pub const R_PARISC_PCREL16F: u32 = 77;
pub const R_PARISC_PCREL16WF: u32 = 78;
pub const R_PARISC_PCREL16DF: u32 = 79;
pub const R_PARISC_DIR64: u32 = 80;
pub const R_PARISC_DIR14WR: u32 = 83;
pub const R_PARISC_DIR14DR: u32 = 84;
pub const R_PARISC_DIR16F: u32 = 85;
pub const R_PARISC_DIR16WF: u32 = 86;
pub const R_PARISC_DIR16DF: u32 = 87;
pub const R_PARISC_GPREL64: u32 = 88;
pub const R_PARISC_GPREL14WR: u32 = 91;
pub const R_PARISC_GPREL14DR: u32 = 92;
pub const R_PARISC_GPREL16F: u32 = 93;
pub const R_PARISC_GPREL16WF: u32 = 94;
pub const R_PARISC_GPREL16DF: u32 = 95;
pub const R_PARISC_LTOFF64: u32 = 96;
pub const R_PARISC_LTOFF14WR: u32 = 99;
pub const R_PARISC_LTOFF14DR: u32 = 100;
pub const R_PARISC_LTOFF16F: u32 = 101;
pub const R_PARISC_LTOFF16WF: u32 = 102;
pub const R_PARISC_LTOFF16DF: u32 = 103;
pub const R_PARISC_SECREL64: u32 = 104;
pub const R_PARISC_SEGREL64: u32 = 112;
pub const R_PARISC_PLTOFF14WR: u32 = 115;
pub const R_PARISC_PLTOFF14DR: u32 = 116;
pub const R_PARISC_PLTOFF16F: u32 = 117;
pub const R_PARISC_PLTOFF16WF: u32 = 118;
pub const R_PARISC_PLTOFF16DF: u32 = 119;
pub const R_PARISC_LTOFF_FPTR64: u32 = 120;
pub const R_PARISC_LTOFF_FPTR14WR: u32 = 123;
pub const R_PARISC_LTOFF_FPTR14DR: u32 = 124;
pub const R_PARISC_LTOFF_FPTR16F: u32 = 125;
pub const R_PARISC_LTOFF_FPTR16WF: u32 = 126;
pub const R_PARISC_LTOFF_FPTR16DF: u32 = 127;
pub const R_PARISC_LORESERVE: u32 = 128;
pub const R_PARISC_COPY: u32 = 128;
pub const R_PARISC_IPLT: u32 = 129;
pub const R_PARISC_EPLT: u32 = 130;
pub const R_PARISC_TPREL32: u32 = 153;
pub const R_PARISC_TPREL21L: u32 = 154;
pub const R_PARISC_TPREL14R: u32 = 158;
pub const R_PARISC_LTOFF_TP21L: u32 = 162;
pub const R_PARISC_LTOFF_TP14R: u32 = 166;
pub const R_PARISC_LTOFF_TP14F: u32 = 167;
pub const R_PARISC_TPREL64: u32 = 216;
pub const R_PARISC_TPREL14WR: u32 = 219;
pub const R_PARISC_TPREL14DR: u32 = 220;
pub const R_PARISC_TPREL16F: u32 = 221;
pub const R_PARISC_TPREL16WF: u32 = 222;
pub const R_PARISC_TPREL16DF: u32 = 223;
pub const R_PARISC_LTOFF_TP64: u32 = 224;
pub const R_PARISC_LTOFF_TP14WR: u32 = 227;
pub const R_PARISC_LTOFF_TP14DR: u32 = 228;
pub const R_PARISC_LTOFF_TP16F: u32 = 229;
pub const R_PARISC_LTOFF_TP16WF: u32 = 230;
pub const R_PARISC_LTOFF_TP16DF: u32 = 231;
pub const R_PARISC_GNU_VTENTRY: u32 = 232;
pub const R_PARISC_GNU_VTINHERIT: u32 = 233;
pub const R_PARISC_TLS_GD21L: u32 = 234;
pub const R_PARISC_TLS_GD14R: u32 = 235;
pub const R_PARISC_TLS_GDCALL: u32 = 236;
pub const R_PARISC_TLS_LDM21L: u32 = 237;
pub const R_PARISC_TLS_LDM14R: u32 = 238;
pub const R_PARISC_TLS_LDMCALL: u32 = 239;
pub const R_PARISC_TLS_LDO21L: u32 = 240;
pub const R_PARISC_TLS_LDO14R: u32 = 241;
pub const R_PARISC_TLS_DTPMOD32: u32 = 242;
pub const R_PARISC_TLS_DTPMOD64: u32 = 243;
pub const R_PARISC_TLS_DTPOFF32: u32 = 244;
pub const R_PARISC_TLS_DTPOFF64: u32 = 245;
pub const R_PARISC_TLS_LE21L: u32 = R_PARISC_TPREL21L;
pub const R_PARISC_TLS_LE14R: u32 = R_PARISC_TPREL14R;
pub const R_PARISC_TLS_IE21L: u32 = R_PARISC_LTOFF_TP21L;
pub const R_PARISC_TLS_IE14R: u32 = R_PARISC_LTOFF_TP14R;
pub const R_PARISC_TLS_TPREL32: u32 = R_PARISC_TPREL32;
pub const R_PARISC_TLS_TPREL64: u32 = R_PARISC_TPREL64;
pub const R_PARISC_HIRESERVE: u32 = 255;


/* Legal values for p_type field of Elf32_Phdr/Elf64_Phdr.  */




































pub const PT_PARISC_ARCHEXT: u32 = 0x70000000;
pub const PT_PARISC_UNWIND: u32 = 0x70000001;


/* Legal values for p_flags field of Elf32_Phdr/Elf64_Phdr.  */




pub const PF_PARISC_SBP: u32 = 0x08000000;


pub const PF_HP_PAGE_SIZE: u32 = 0x00100000;
pub const PF_HP_FAR_SHARED: u32 = 0x00200000;
pub const PF_HP_NEAR_SHARED: u32 = 0x00400000;
pub const PF_HP_CODE: u32 = 0x01000000;
pub const PF_HP_MODIFY: u32 = 0x02000000;
pub const PF_HP_LAZYSWAP: u32 = 0x04000000;
pub const PF_HP_SBP: u32 = 0x08000000;




/* Alpha specific definitions.  */




/* Legal values for e_flags field of Elf64_Ehdr.  */




pub const EF_ALPHA_32BIT: u32 = 1;
pub const EF_ALPHA_CANRELAX: u32 = 2;


/* Legal values for sh_type field of Elf64_Shdr.  */




/* These two are primerily concerned with ECOFF debugging info.  */


pub const SHT_ALPHA_DEBUG: u32 = 0x70000001;
pub const SHT_ALPHA_REGINFO: u32 = 0x70000002;


/* Legal values for sh_flags field of Elf64_Shdr.  */




pub const SHF_ALPHA_GPREL: u32 = 0x10000000;


/* Legal values for st_other field of Elf64_Sym.  */


pub const STO_ALPHA_NOPV: u32 = 0x80;
pub const STO_ALPHA_STD_GPLOAD: u32 = 0x88;


/* Alpha relocs.  */




pub const R_ALPHA_NONE: u32 = 0;
pub const R_ALPHA_REFLONG: u32 = 1;
pub const R_ALPHA_REFQUAD: u32 = 2;
pub const R_ALPHA_GPREL32: u32 = 3;
pub const R_ALPHA_LITERAL: u32 = 4;
pub const R_ALPHA_LITUSE: u32 = 5;
pub const R_ALPHA_GPDISP: u32 = 6;
pub const R_ALPHA_BRADDR: u32 = 7;
pub const R_ALPHA_HINT: u32 = 8;
pub const R_ALPHA_SREL16: u32 = 9;
pub const R_ALPHA_SREL32: u32 = 10;
pub const R_ALPHA_SREL64: u32 = 11;
pub const R_ALPHA_GPRELHIGH: u32 = 17;
pub const R_ALPHA_GPRELLOW: u32 = 18;
pub const R_ALPHA_GPREL16: u32 = 19;
pub const R_ALPHA_COPY: u32 = 24;
pub const R_ALPHA_GLOB_DAT: u32 = 25;
pub const R_ALPHA_JMP_SLOT: u32 = 26;
pub const R_ALPHA_RELATIVE: u32 = 27;
pub const R_ALPHA_TLS_GD_HI: u32 = 28;
pub const R_ALPHA_TLSGD: u32 = 29;
pub const R_ALPHA_TLS_LDM: u32 = 30;
pub const R_ALPHA_DTPMOD64: u32 = 31;
pub const R_ALPHA_GOTDTPREL: u32 = 32;
pub const R_ALPHA_DTPREL64: u32 = 33;
pub const R_ALPHA_DTPRELHI: u32 = 34;
pub const R_ALPHA_DTPRELLO: u32 = 35;
pub const R_ALPHA_DTPREL16: u32 = 36;
pub const R_ALPHA_GOTTPREL: u32 = 37;
pub const R_ALPHA_TPREL64: u32 = 38;
pub const R_ALPHA_TPRELHI: u32 = 39;
pub const R_ALPHA_TPRELLO: u32 = 40;
pub const R_ALPHA_TPREL16: u32 = 41;
/* Keep this the last entry.  */


pub const R_ALPHA_NUM: u32 = 46;


/* Magic values of the LITUSE relocation addend.  */


pub const LITUSE_ALPHA_ADDR: u32 = 0;
pub const LITUSE_ALPHA_BASE: u32 = 1;
pub const LITUSE_ALPHA_BYTOFF: u32 = 2;
pub const LITUSE_ALPHA_JSR: u32 = 3;
pub const LITUSE_ALPHA_TLS_GD: u32 = 4;
pub const LITUSE_ALPHA_TLS_LDM: u32 = 5;


/* Legal values for d_tag of Elf64_Dyn.  */




pub const DT_ALPHA_NUM: u32 = 1;


/* PowerPC specific declarations */




/* Values for Elf32/64_Ehdr.e_flags.  */


pub const EF_PPC_EMB: u32 = 0x80000000;


/* Cygnus local bits below */


pub const EF_PPC_RELOCATABLE: u32 = 0x00010000;






/* PowerPC relocations defined by the ABIs */


pub const R_PPC_NONE: u32 = 0;
pub const R_PPC_ADDR32: u32 = 1;
pub const R_PPC_ADDR24: u32 = 2;
pub const R_PPC_ADDR16: u32 = 3;
pub const R_PPC_ADDR16_LO: u32 = 4;
pub const R_PPC_ADDR16_HI: u32 = 5;
pub const R_PPC_ADDR16_HA: u32 = 6;
pub const R_PPC_ADDR14: u32 = 7;
pub const R_PPC_ADDR14_BRTAKEN: u32 = 8;
pub const R_PPC_ADDR14_BRNTAKEN: u32 = 9;
pub const R_PPC_REL24: u32 = 10;
pub const R_PPC_REL14: u32 = 11;
pub const R_PPC_REL14_BRTAKEN: u32 = 12;
pub const R_PPC_REL14_BRNTAKEN: u32 = 13;
pub const R_PPC_GOT16: u32 = 14;
pub const R_PPC_GOT16_LO: u32 = 15;
pub const R_PPC_GOT16_HI: u32 = 16;
pub const R_PPC_GOT16_HA: u32 = 17;
pub const R_PPC_PLTREL24: u32 = 18;
pub const R_PPC_COPY: u32 = 19;
pub const R_PPC_GLOB_DAT: u32 = 20;
pub const R_PPC_JMP_SLOT: u32 = 21;
pub const R_PPC_RELATIVE: u32 = 22;
pub const R_PPC_LOCAL24PC: u32 = 23;
pub const R_PPC_UADDR32: u32 = 24;
pub const R_PPC_UADDR16: u32 = 25;
pub const R_PPC_REL32: u32 = 26;
pub const R_PPC_PLT32: u32 = 27;
pub const R_PPC_PLTREL32: u32 = 28;
pub const R_PPC_PLT16_LO: u32 = 29;
pub const R_PPC_PLT16_HI: u32 = 30;
pub const R_PPC_PLT16_HA: u32 = 31;
pub const R_PPC_SDAREL16: u32 = 32;
pub const R_PPC_SECTOFF: u32 = 33;
pub const R_PPC_SECTOFF_LO: u32 = 34;
pub const R_PPC_SECTOFF_HI: u32 = 35;
pub const R_PPC_SECTOFF_HA: u32 = 36;


/* PowerPC relocations defined for the TLS access ABI.  */


pub const R_PPC_TLS: u32 = 67;
pub const R_PPC_DTPMOD32: u32 = 68;
pub const R_PPC_TPREL16: u32 = 69;
pub const R_PPC_TPREL16_LO: u32 = 70;
pub const R_PPC_TPREL16_HI: u32 = 71;
pub const R_PPC_TPREL16_HA: u32 = 72;
pub const R_PPC_TPREL32: u32 = 73;
pub const R_PPC_DTPREL16: u32 = 74;
pub const R_PPC_DTPREL16_LO: u32 = 75;
pub const R_PPC_DTPREL16_HI: u32 = 76;
pub const R_PPC_DTPREL16_HA: u32 = 77;
pub const R_PPC_DTPREL32: u32 = 78;
pub const R_PPC_GOT_TLSGD16: u32 = 79;
pub const R_PPC_GOT_TLSGD16_LO: u32 = 80;
pub const R_PPC_GOT_TLSGD16_HI: u32 = 81;
pub const R_PPC_GOT_TLSGD16_HA: u32 = 82;
pub const R_PPC_GOT_TLSLD16: u32 = 83;
pub const R_PPC_GOT_TLSLD16_LO: u32 = 84;
pub const R_PPC_GOT_TLSLD16_HI: u32 = 85;
pub const R_PPC_GOT_TLSLD16_HA: u32 = 86;
pub const R_PPC_GOT_TPREL16: u32 = 87;
pub const R_PPC_GOT_TPREL16_LO: u32 = 88;
pub const R_PPC_GOT_TPREL16_HI: u32 = 89;
pub const R_PPC_GOT_TPREL16_HA: u32 = 90;
pub const R_PPC_GOT_DTPREL16: u32 = 91;
pub const R_PPC_GOT_DTPREL16_LO: u32 = 92;
pub const R_PPC_GOT_DTPREL16_HI: u32 = 93;
pub const R_PPC_GOT_DTPREL16_HA: u32 = 94;


/* The remaining relocs are from the Embedded ELF ABI, and are not
   in the SVR4 ELF ABI.  */


pub const R_PPC_EMB_NADDR32: u32 = 101;
pub const R_PPC_EMB_NADDR16: u32 = 102;
pub const R_PPC_EMB_NADDR16_LO: u32 = 103;
pub const R_PPC_EMB_NADDR16_HI: u32 = 104;
pub const R_PPC_EMB_NADDR16_HA: u32 = 105;
pub const R_PPC_EMB_SDAI16: u32 = 106;
pub const R_PPC_EMB_SDA2I16: u32 = 107;
pub const R_PPC_EMB_SDA2REL: u32 = 108;
pub const R_PPC_EMB_SDA21: u32 = 109;
pub const R_PPC_EMB_MRKREF: u32 = 110;
pub const R_PPC_EMB_RELSEC16: u32 = 111;
pub const R_PPC_EMB_RELST_LO: u32 = 112;
pub const R_PPC_EMB_RELST_HI: u32 = 113;
pub const R_PPC_EMB_RELST_HA: u32 = 114;
pub const R_PPC_EMB_BIT_FLD: u32 = 115;
pub const R_PPC_EMB_RELSDA: u32 = 116;


/* Diab tool relocations.  */


pub const R_PPC_DIAB_SDA21_LO: u32 = 180;
pub const R_PPC_DIAB_SDA21_HI: u32 = 181;
pub const R_PPC_DIAB_SDA21_HA: u32 = 182;
pub const R_PPC_DIAB_RELSDA_LO: u32 = 183;
pub const R_PPC_DIAB_RELSDA_HI: u32 = 184;
pub const R_PPC_DIAB_RELSDA_HA: u32 = 185;


/* GNU extension to support local ifunc.  */


pub const R_PPC_IRELATIVE: u32 = 248;


/* GNU relocs used in PIC code sequences.  */


pub const R_PPC_REL16: u32 = 249;
pub const R_PPC_REL16_LO: u32 = 250;
pub const R_PPC_REL16_HI: u32 = 251;
pub const R_PPC_REL16_HA: u32 = 252;


/* This is a phony reloc to handle any old fashioned TOC16 references
   that may still be in object files.  */


pub const R_PPC_TOC16: u32 = 255;


/* PowerPC specific values for the Dyn d_tag field.  */




pub const DT_PPC_NUM: u32 = 1;


/* PowerPC64 relocations defined by the ABIs */


pub const R_PPC64_NONE: u32 = R_PPC_NONE;
pub const R_PPC64_ADDR32: u32 = R_PPC_ADDR32;
pub const R_PPC64_ADDR24: u32 = R_PPC_ADDR24;
pub const R_PPC64_ADDR16: u32 = R_PPC_ADDR16;
pub const R_PPC64_ADDR16_LO: u32 = R_PPC_ADDR16_LO;
pub const R_PPC64_ADDR16_HI: u32 = R_PPC_ADDR16_HI;
pub const R_PPC64_ADDR16_HA: u32 = R_PPC_ADDR16_HA;
pub const R_PPC64_ADDR14: u32 = R_PPC_ADDR14;
pub const R_PPC64_ADDR14_BRTAKEN: u32 = R_PPC_ADDR14_BRTAKEN;
pub const R_PPC64_ADDR14_BRNTAKEN: u32 = R_PPC_ADDR14_BRNTAKEN;
pub const R_PPC64_REL24: u32 = R_PPC_REL24;
pub const R_PPC64_REL14: u32 = R_PPC_REL14;
pub const R_PPC64_REL14_BRTAKEN: u32 = R_PPC_REL14_BRTAKEN;
pub const R_PPC64_REL14_BRNTAKEN: u32 = R_PPC_REL14_BRNTAKEN;
pub const R_PPC64_GOT16: u32 = R_PPC_GOT16;
pub const R_PPC64_GOT16_LO: u32 = R_PPC_GOT16_LO;
pub const R_PPC64_GOT16_HI: u32 = R_PPC_GOT16_HI;
pub const R_PPC64_GOT16_HA: u32 = R_PPC_GOT16_HA;


pub const R_PPC64_COPY: u32 = R_PPC_COPY;
pub const R_PPC64_GLOB_DAT: u32 = R_PPC_GLOB_DAT;
pub const R_PPC64_JMP_SLOT: u32 = R_PPC_JMP_SLOT;
pub const R_PPC64_RELATIVE: u32 = R_PPC_RELATIVE;


pub const R_PPC64_UADDR32: u32 = R_PPC_UADDR32;
pub const R_PPC64_UADDR16: u32 = R_PPC_UADDR16;
pub const R_PPC64_REL32: u32 = R_PPC_REL32;
pub const R_PPC64_PLT32: u32 = R_PPC_PLT32;
pub const R_PPC64_PLTREL32: u32 = R_PPC_PLTREL32;
pub const R_PPC64_PLT16_LO: u32 = R_PPC_PLT16_LO;
pub const R_PPC64_PLT16_HI: u32 = R_PPC_PLT16_HI;
pub const R_PPC64_PLT16_HA: u32 = R_PPC_PLT16_HA;


pub const R_PPC64_SECTOFF: u32 = R_PPC_SECTOFF;
pub const R_PPC64_SECTOFF_LO: u32 = R_PPC_SECTOFF_LO;
pub const R_PPC64_SECTOFF_HI: u32 = R_PPC_SECTOFF_HI;
pub const R_PPC64_SECTOFF_HA: u32 = R_PPC_SECTOFF_HA;
pub const R_PPC64_ADDR30: u32 = 37;
pub const R_PPC64_ADDR64: u32 = 38;
pub const R_PPC64_ADDR16_HIGHER: u32 = 39;
pub const R_PPC64_ADDR16_HIGHERA: u32 = 40;
pub const R_PPC64_ADDR16_HIGHEST: u32 = 41;
pub const R_PPC64_ADDR16_HIGHESTA: u32 = 42;
pub const R_PPC64_UADDR64: u32 = 43;
pub const R_PPC64_REL64: u32 = 44;
pub const R_PPC64_PLT64: u32 = 45;
pub const R_PPC64_PLTREL64: u32 = 46;
pub const R_PPC64_TOC16: u32 = 47;
pub const R_PPC64_TOC16_LO: u32 = 48;
pub const R_PPC64_TOC16_HI: u32 = 49;
pub const R_PPC64_TOC16_HA: u32 = 50;
pub const R_PPC64_TOC: u32 = 51;
pub const R_PPC64_PLTGOT16: u32 = 52;
pub const R_PPC64_PLTGOT16_LO: u32 = 53;
pub const R_PPC64_PLTGOT16_HI: u32 = 54;
pub const R_PPC64_PLTGOT16_HA: u32 = 55;


pub const R_PPC64_ADDR16_DS: u32 = 56;
pub const R_PPC64_ADDR16_LO_DS: u32 = 57;
pub const R_PPC64_GOT16_DS: u32 = 58;
pub const R_PPC64_GOT16_LO_DS: u32 = 59;
pub const R_PPC64_PLT16_LO_DS: u32 = 60;
pub const R_PPC64_SECTOFF_DS: u32 = 61;
pub const R_PPC64_SECTOFF_LO_DS: u32 = 62;
pub const R_PPC64_TOC16_DS: u32 = 63;
pub const R_PPC64_TOC16_LO_DS: u32 = 64;
pub const R_PPC64_PLTGOT16_DS: u32 = 65;
pub const R_PPC64_PLTGOT16_LO_DS: u32 = 66;


/* PowerPC64 relocations defined for the TLS access ABI.  */


pub const R_PPC64_TLS: u32 = 67;
pub const R_PPC64_DTPMOD64: u32 = 68;
pub const R_PPC64_TPREL16: u32 = 69;
pub const R_PPC64_TPREL16_LO: u32 = 70;
pub const R_PPC64_TPREL16_HI: u32 = 71;
pub const R_PPC64_TPREL16_HA: u32 = 72;
pub const R_PPC64_TPREL64: u32 = 73;
pub const R_PPC64_DTPREL16: u32 = 74;
pub const R_PPC64_DTPREL16_LO: u32 = 75;
pub const R_PPC64_DTPREL16_HI: u32 = 76;
pub const R_PPC64_DTPREL16_HA: u32 = 77;
pub const R_PPC64_DTPREL64: u32 = 78;
pub const R_PPC64_GOT_TLSGD16: u32 = 79;
pub const R_PPC64_GOT_TLSGD16_LO: u32 = 80;
pub const R_PPC64_GOT_TLSGD16_HI: u32 = 81;
pub const R_PPC64_GOT_TLSGD16_HA: u32 = 82;
pub const R_PPC64_GOT_TLSLD16: u32 = 83;
pub const R_PPC64_GOT_TLSLD16_LO: u32 = 84;
pub const R_PPC64_GOT_TLSLD16_HI: u32 = 85;
pub const R_PPC64_GOT_TLSLD16_HA: u32 = 86;
pub const R_PPC64_GOT_TPREL16_DS: u32 = 87;
pub const R_PPC64_GOT_TPREL16_LO_DS: u32 = 88;
pub const R_PPC64_GOT_TPREL16_HI: u32 = 89;
pub const R_PPC64_GOT_TPREL16_HA: u32 = 90;
pub const R_PPC64_GOT_DTPREL16_DS: u32 = 91;
pub const R_PPC64_GOT_DTPREL16_LO_DS: u32 = 92;
pub const R_PPC64_GOT_DTPREL16_HI: u32 = 93;
pub const R_PPC64_GOT_DTPREL16_HA: u32 = 94;
pub const R_PPC64_TPREL16_DS: u32 = 95;
pub const R_PPC64_TPREL16_LO_DS: u32 = 96;
pub const R_PPC64_TPREL16_HIGHER: u32 = 97;
pub const R_PPC64_TPREL16_HIGHERA: u32 = 98;
pub const R_PPC64_TPREL16_HIGHEST: u32 = 99;
pub const R_PPC64_TPREL16_HIGHESTA: u32 = 100;
pub const R_PPC64_DTPREL16_DS: u32 = 101;
pub const R_PPC64_DTPREL16_LO_DS: u32 = 102;
pub const R_PPC64_DTPREL16_HIGHER: u32 = 103;
pub const R_PPC64_DTPREL16_HIGHERA: u32 = 104;
pub const R_PPC64_DTPREL16_HIGHEST: u32 = 105;
pub const R_PPC64_DTPREL16_HIGHESTA: u32 = 106;


/* GNU extension to support local ifunc.  */


pub const R_PPC64_JMP_IREL: u32 = 247;
pub const R_PPC64_IRELATIVE: u32 = 248;
pub const R_PPC64_REL16: u32 = 249;
pub const R_PPC64_REL16_LO: u32 = 250;
pub const R_PPC64_REL16_HI: u32 = 251;
pub const R_PPC64_REL16_HA: u32 = 252;


/* PowerPC64 specific values for the Dyn d_tag field.  */








pub const DT_PPC64_NUM: u32 = 3;




/* ARM specific declarations */




/* Processor specific flags for the ELF header e_flags field.  */


pub const EF_ARM_RELEXEC: u32 = 0x01;
pub const EF_ARM_HASENTRY: u32 = 0x02;
pub const EF_ARM_INTERWORK: u32 = 0x04;
pub const EF_ARM_APCS_26: u32 = 0x08;
pub const EF_ARM_APCS_FLOAT: u32 = 0x10;
pub const EF_ARM_PIC: u32 = 0x20;
pub const EF_ARM_ALIGN8: u32 = 0x40;
pub const EF_ARM_NEW_ABI: u32 = 0x80;
pub const EF_ARM_OLD_ABI: u32 = 0x100;
pub const EF_ARM_SOFT_FLOAT: u32 = 0x200;
pub const EF_ARM_VFP_FLOAT: u32 = 0x400;
pub const EF_ARM_MAVERICK_FLOAT: u32 = 0x800;




/* Other constants defined in the ARM ELF spec. version B-01.  */


/* NB. These conflict with values defined above.  */


pub const EF_ARM_SYMSARESORTED: u32 = 0x04;
pub const EF_ARM_DYNSYMSUSESEGIDX: u32 = 0x08;
pub const EF_ARM_MAPSYMSFIRST: u32 = 0x10;
pub const EF_ARM_EABIMASK: u32 = 0XFF000000;


/* Constants defined in AAELF.  */


pub const EF_ARM_BE8: u32 = 0x00800000;
pub const EF_ARM_LE8: u32 = 0x00400000;




pub const EF_ARM_EABI_UNKNOWN: u32 = 0x00000000;
pub const EF_ARM_EABI_VER1: u32 = 0x01000000;
pub const EF_ARM_EABI_VER2: u32 = 0x02000000;
pub const EF_ARM_EABI_VER3: u32 = 0x03000000;
pub const EF_ARM_EABI_VER4: u32 = 0x04000000;
pub const EF_ARM_EABI_VER5: u32 = 0x05000000;


/* Additional symbol types for Thumb.  */


pub const STT_ARM_TFUNC: u32 = STT_LOPROC;
pub const STT_ARM_16BIT: u32 = STT_HIPROC;


/* ARM-specific values for sh_flags */


pub const SHF_ARM_ENTRYSECT: u32 = 0x10000000;






/* ARM-specific program header flags */






pub const PF_ARM_PI: u32 = 0x20000000;
pub const PF_ARM_ABS: u32 = 0x40000000;


/* Processor specific values for the Phdr p_type field.  */






/* Processor specific values for the Shdr sh_type field.  */












/* ARM relocs.  */




pub const R_ARM_NONE: u32 = 0;
pub const R_ARM_PC24: u32 = 1;
pub const R_ARM_ABS32: u32 = 2;
pub const R_ARM_REL32: u32 = 3;
pub const R_ARM_PC13: u32 = 4;
pub const R_ARM_ABS16: u32 = 5;
pub const R_ARM_ABS12: u32 = 6;
pub const R_ARM_THM_ABS5: u32 = 7;
pub const R_ARM_ABS8: u32 = 8;
pub const R_ARM_SBREL32: u32 = 9;
pub const R_ARM_THM_PC22: u32 = 10;
pub const R_ARM_THM_PC8: u32 = 11;
pub const R_ARM_AMP_VCALL9: u32 = 12;
pub const R_ARM_SWI24: u32 = 13;
pub const R_ARM_TLS_DESC: u32 = 13;
pub const R_ARM_THM_SWI8: u32 = 14;
pub const R_ARM_XPC25: u32 = 15;
pub const R_ARM_THM_XPC22: u32 = 16;
pub const R_ARM_TLS_DTPMOD32: u32 = 17;
pub const R_ARM_TLS_DTPOFF32: u32 = 18;
pub const R_ARM_TLS_TPOFF32: u32 = 19;
pub const R_ARM_COPY: u32 = 20;
pub const R_ARM_GLOB_DAT: u32 = 21;
pub const R_ARM_JUMP_SLOT: u32 = 22;
pub const R_ARM_RELATIVE: u32 = 23;
pub const R_ARM_GOTOFF: u32 = 24;
pub const R_ARM_GOTPC: u32 = 25;
pub const R_ARM_GOT32: u32 = 26;
pub const R_ARM_PLT32: u32 = 27;
pub const R_ARM_ALU_PCREL_7_0: u32 = 32;
pub const R_ARM_ALU_PCREL_15_8: u32 = 33;
pub const R_ARM_ALU_PCREL_23_15: u32 = 34;
pub const R_ARM_LDR_SBREL_11_0: u32 = 35;
pub const R_ARM_ALU_SBREL_19_12: u32 = 36;
pub const R_ARM_ALU_SBREL_27_20: u32 = 37;
pub const R_ARM_TLS_GOTDESC: u32 = 90;
pub const R_ARM_TLS_CALL: u32 = 91;
pub const R_ARM_TLS_DESCSEQ: u32 = 92;
pub const R_ARM_THM_TLS_CALL: u32 = 93;
pub const R_ARM_GNU_VTENTRY: u32 = 100;
pub const R_ARM_GNU_VTINHERIT: u32 = 101;
pub const R_ARM_THM_PC11: u32 = 102;
pub const R_ARM_THM_PC9: u32 = 103;




















pub const R_ARM_THM_TLS_DESCSEQ: u32 = 129;
pub const R_ARM_IRELATIVE: u32 = 160;
pub const R_ARM_RXPC25: u32 = 249;
pub const R_ARM_RSBREL32: u32 = 250;
pub const R_ARM_THM_RPC22: u32 = 251;
pub const R_ARM_RREL32: u32 = 252;
pub const R_ARM_RABS22: u32 = 253;
pub const R_ARM_RPC24: u32 = 254;
pub const R_ARM_RBASE: u32 = 255;
/* Keep this the last entry.  */


pub const R_ARM_NUM: u32 = 256;


/* IA-64 specific declarations.  */




/* Processor specific flags for the Ehdr e_flags field.  */


pub const EF_IA_64_MASKOS: u32 = 0x0000000f;
pub const EF_IA_64_ABI64: u32 = 0x00000010;
pub const EF_IA_64_ARCH: u32 = 0xff000000;


/* Processor specific values for the Phdr p_type field.  */














/* Processor specific flags for the Phdr p_flags field.  */


pub const PF_IA_64_NORECOV: u32 = 0x80000000;


/* Processor specific values for the Shdr sh_type field.  */








/* Processor specific flags for the Shdr sh_flags field.  */


pub const SHF_IA_64_SHORT: u32 = 0x10000000;
pub const SHF_IA_64_NORECOV: u32 = 0x20000000;


/* Processor specific values for the Dyn d_tag field.  */




pub const DT_IA_64_NUM: u32 = 1;


/* IA-64 relocations.  */


pub const R_IA64_NONE: u32 = 0x00;
pub const R_IA64_IMM14: u32 = 0x21;
pub const R_IA64_IMM22: u32 = 0x22;
pub const R_IA64_IMM64: u32 = 0x23;
pub const R_IA64_DIR32MSB: u32 = 0x24;
pub const R_IA64_DIR32LSB: u32 = 0x25;
pub const R_IA64_DIR64MSB: u32 = 0x26;
pub const R_IA64_DIR64LSB: u32 = 0x27;
pub const R_IA64_GPREL22: u32 = 0x2a;
pub const R_IA64_GPREL64I: u32 = 0x2b;
pub const R_IA64_GPREL32MSB: u32 = 0x2c;
pub const R_IA64_GPREL32LSB: u32 = 0x2d;
pub const R_IA64_GPREL64MSB: u32 = 0x2e;
pub const R_IA64_GPREL64LSB: u32 = 0x2f;
pub const R_IA64_LTOFF22: u32 = 0x32;
pub const R_IA64_LTOFF64I: u32 = 0x33;
pub const R_IA64_PLTOFF22: u32 = 0x3a;
pub const R_IA64_PLTOFF64I: u32 = 0x3b;
pub const R_IA64_PLTOFF64MSB: u32 = 0x3e;
pub const R_IA64_PLTOFF64LSB: u32 = 0x3f;
pub const R_IA64_FPTR64I: u32 = 0x43;
pub const R_IA64_FPTR32MSB: u32 = 0x44;
pub const R_IA64_FPTR32LSB: u32 = 0x45;
pub const R_IA64_FPTR64MSB: u32 = 0x46;
pub const R_IA64_FPTR64LSB: u32 = 0x47;
pub const R_IA64_PCREL60B: u32 = 0x48;
pub const R_IA64_PCREL21B: u32 = 0x49;
pub const R_IA64_PCREL21M: u32 = 0x4a;
pub const R_IA64_PCREL21F: u32 = 0x4b;
pub const R_IA64_PCREL32MSB: u32 = 0x4c;
pub const R_IA64_PCREL32LSB: u32 = 0x4d;
pub const R_IA64_PCREL64MSB: u32 = 0x4e;
pub const R_IA64_PCREL64LSB: u32 = 0x4f;
pub const R_IA64_LTOFF_FPTR22: u32 = 0x52;
pub const R_IA64_LTOFF_FPTR64I: u32 = 0x53;
pub const R_IA64_LTOFF_FPTR32MSB: u32 = 0x54;
pub const R_IA64_LTOFF_FPTR32LSB: u32 = 0x55;
pub const R_IA64_LTOFF_FPTR64MSB: u32 = 0x56;
pub const R_IA64_LTOFF_FPTR64LSB: u32 = 0x57;
pub const R_IA64_SEGREL32MSB: u32 = 0x5c;
pub const R_IA64_SEGREL32LSB: u32 = 0x5d;
pub const R_IA64_SEGREL64MSB: u32 = 0x5e;
pub const R_IA64_SEGREL64LSB: u32 = 0x5f;
pub const R_IA64_SECREL32MSB: u32 = 0x64;
pub const R_IA64_SECREL32LSB: u32 = 0x65;
pub const R_IA64_SECREL64MSB: u32 = 0x66;
pub const R_IA64_SECREL64LSB: u32 = 0x67;
pub const R_IA64_REL32MSB: u32 = 0x6c;
pub const R_IA64_REL32LSB: u32 = 0x6d;
pub const R_IA64_REL64MSB: u32 = 0x6e;
pub const R_IA64_REL64LSB: u32 = 0x6f;
pub const R_IA64_LTV32MSB: u32 = 0x74;
pub const R_IA64_LTV32LSB: u32 = 0x75;
pub const R_IA64_LTV64MSB: u32 = 0x76;
pub const R_IA64_LTV64LSB: u32 = 0x77;
pub const R_IA64_PCREL21BI: u32 = 0x79;
pub const R_IA64_PCREL22: u32 = 0x7a;
pub const R_IA64_PCREL64I: u32 = 0x7b;
pub const R_IA64_IPLTMSB: u32 = 0x80;
pub const R_IA64_IPLTLSB: u32 = 0x81;
pub const R_IA64_COPY: u32 = 0x84;
pub const R_IA64_SUB: u32 = 0x85;
pub const R_IA64_LTOFF22X: u32 = 0x86;
pub const R_IA64_LDXMOV: u32 = 0x87;
pub const R_IA64_TPREL14: u32 = 0x91;
pub const R_IA64_TPREL22: u32 = 0x92;
pub const R_IA64_TPREL64I: u32 = 0x93;
pub const R_IA64_TPREL64MSB: u32 = 0x96;
pub const R_IA64_TPREL64LSB: u32 = 0x97;
pub const R_IA64_LTOFF_TPREL22: u32 = 0x9a;
pub const R_IA64_DTPMOD64MSB: u32 = 0xa6;
pub const R_IA64_DTPMOD64LSB: u32 = 0xa7;
pub const R_IA64_LTOFF_DTPMOD22: u32 = 0xaa;
pub const R_IA64_DTPREL14: u32 = 0xb1;
pub const R_IA64_DTPREL22: u32 = 0xb2;
pub const R_IA64_DTPREL64I: u32 = 0xb3;
pub const R_IA64_DTPREL32MSB: u32 = 0xb4;
pub const R_IA64_DTPREL32LSB: u32 = 0xb5;
pub const R_IA64_DTPREL64MSB: u32 = 0xb6;
pub const R_IA64_DTPREL64LSB: u32 = 0xb7;
pub const R_IA64_LTOFF_DTPREL22: u32 = 0xba;


/* SH specific declarations */




/* Processor specific flags for the ELF header e_flags field.  */


pub const EF_SH_MACH_MASK: u32 = 0x1f;
pub const EF_SH_UNKNOWN: u32 = 0x0;
pub const EF_SH1: u32 = 0x1;
pub const EF_SH2: u32 = 0x2;
pub const EF_SH3: u32 = 0x3;
pub const EF_SH_DSP: u32 = 0x4;
pub const EF_SH3_DSP: u32 = 0x5;
pub const EF_SH4AL_DSP: u32 = 0x6;
pub const EF_SH3E: u32 = 0x8;
pub const EF_SH4: u32 = 0x9;
pub const EF_SH2E: u32 = 0xb;
pub const EF_SH4A: u32 = 0xc;
pub const EF_SH2A: u32 = 0xd;
pub const EF_SH4_NOFPU: u32 = 0x10;
pub const EF_SH4A_NOFPU: u32 = 0x11;
pub const EF_SH4_NOMMU_NOFPU: u32 = 0x12;
pub const EF_SH2A_NOFPU: u32 = 0x13;
pub const EF_SH3_NOMMU: u32 = 0x14;
pub const EF_SH2A_SH4_NOFPU: u32 = 0x15;
pub const EF_SH2A_SH3_NOFPU: u32 = 0x16;
pub const EF_SH2A_SH4: u32 = 0x17;
pub const EF_SH2A_SH3E: u32 = 0x18;


/* SH relocs.  */


pub const R_SH_NONE: u32 = 0;
pub const R_SH_DIR32: u32 = 1;
pub const R_SH_REL32: u32 = 2;
pub const R_SH_DIR8WPN: u32 = 3;
pub const R_SH_IND12W: u32 = 4;
pub const R_SH_DIR8WPL: u32 = 5;
pub const R_SH_DIR8WPZ: u32 = 6;
pub const R_SH_DIR8BP: u32 = 7;
pub const R_SH_DIR8W: u32 = 8;
pub const R_SH_DIR8L: u32 = 9;
pub const R_SH_SWITCH16: u32 = 25;
pub const R_SH_SWITCH32: u32 = 26;
pub const R_SH_USES: u32 = 27;
pub const R_SH_COUNT: u32 = 28;
pub const R_SH_ALIGN: u32 = 29;
pub const R_SH_CODE: u32 = 30;
pub const R_SH_DATA: u32 = 31;
pub const R_SH_LABEL: u32 = 32;
pub const R_SH_SWITCH8: u32 = 33;
pub const R_SH_GNU_VTINHERIT: u32 = 34;
pub const R_SH_GNU_VTENTRY: u32 = 35;
pub const R_SH_TLS_GD_32: u32 = 144;
pub const R_SH_TLS_LD_32: u32 = 145;
pub const R_SH_TLS_LDO_32: u32 = 146;
pub const R_SH_TLS_IE_32: u32 = 147;
pub const R_SH_TLS_LE_32: u32 = 148;
pub const R_SH_TLS_DTPMOD32: u32 = 149;
pub const R_SH_TLS_DTPOFF32: u32 = 150;
pub const R_SH_TLS_TPOFF32: u32 = 151;
pub const R_SH_GOT32: u32 = 160;
pub const R_SH_PLT32: u32 = 161;
pub const R_SH_COPY: u32 = 162;
pub const R_SH_GLOB_DAT: u32 = 163;
pub const R_SH_JMP_SLOT: u32 = 164;
pub const R_SH_RELATIVE: u32 = 165;
pub const R_SH_GOTOFF: u32 = 166;
pub const R_SH_GOTPC: u32 = 167;
/* Keep this the last entry.  */


pub const R_SH_NUM: u32 = 256;


/* S/390 specific definitions.  */




/* Valid values for the e_flags field.  */




pub const EF_S390_HIGH_GPRS: u32 = 0x00000001;


/* Additional s390 relocs */




pub const R_390_NONE: u32 = 0;
pub const R_390_8: u32 = 1;
pub const R_390_12: u32 = 2;
pub const R_390_16: u32 = 3;
pub const R_390_32: u32 = 4;
pub const R_390_PC32: u32 = 5;
pub const R_390_GOT12: u32 = 6;
pub const R_390_GOT32: u32 = 7;
pub const R_390_PLT32: u32 = 8;
pub const R_390_COPY: u32 = 9;
pub const R_390_GLOB_DAT: u32 = 10;
pub const R_390_JMP_SLOT: u32 = 11;
pub const R_390_RELATIVE: u32 = 12;
pub const R_390_GOTOFF32: u32 = 13;
pub const R_390_GOTPC: u32 = 14;
pub const R_390_GOT16: u32 = 15;
pub const R_390_PC16: u32 = 16;
pub const R_390_PC16DBL: u32 = 17;
pub const R_390_PLT16DBL: u32 = 18;
pub const R_390_PC32DBL: u32 = 19;
pub const R_390_PLT32DBL: u32 = 20;
pub const R_390_GOTPCDBL: u32 = 21;
pub const R_390_64: u32 = 22;
pub const R_390_PC64: u32 = 23;
pub const R_390_GOT64: u32 = 24;
pub const R_390_PLT64: u32 = 25;
pub const R_390_GOTENT: u32 = 26;
pub const R_390_GOTOFF16: u32 = 27;
pub const R_390_GOTOFF64: u32 = 28;
pub const R_390_GOTPLT12: u32 = 29;
pub const R_390_GOTPLT16: u32 = 30;
pub const R_390_GOTPLT32: u32 = 31;
pub const R_390_GOTPLT64: u32 = 32;
pub const R_390_GOTPLTENT: u32 = 33;
pub const R_390_PLTOFF16: u32 = 34;
pub const R_390_PLTOFF32: u32 = 35;
pub const R_390_PLTOFF64: u32 = 36;
pub const R_390_TLS_LOAD: u32 = 37;
































































pub const R_390_TLS_DTPMOD: u32 = 54;
pub const R_390_TLS_DTPOFF: u32 = 55;




pub const R_390_20: u32 = 57;
pub const R_390_GOT20: u32 = 58;
pub const R_390_GOTPLT20: u32 = 59;




/* Keep this the last entry.  */


pub const R_390_NUM: u32 = 61;




/* CRIS relocations.  */


pub const R_CRIS_NONE: u32 = 0;
pub const R_CRIS_8: u32 = 1;
pub const R_CRIS_16: u32 = 2;
pub const R_CRIS_32: u32 = 3;
pub const R_CRIS_8_PCREL: u32 = 4;
pub const R_CRIS_16_PCREL: u32 = 5;
pub const R_CRIS_32_PCREL: u32 = 6;
pub const R_CRIS_GNU_VTINHERIT: u32 = 7;
pub const R_CRIS_GNU_VTENTRY: u32 = 8;
pub const R_CRIS_COPY: u32 = 9;
pub const R_CRIS_GLOB_DAT: u32 = 10;
pub const R_CRIS_JUMP_SLOT: u32 = 11;
pub const R_CRIS_RELATIVE: u32 = 12;
pub const R_CRIS_16_GOT: u32 = 13;
pub const R_CRIS_32_GOT: u32 = 14;
pub const R_CRIS_16_GOTPLT: u32 = 15;
pub const R_CRIS_32_GOTPLT: u32 = 16;
pub const R_CRIS_32_GOTREL: u32 = 17;
pub const R_CRIS_32_PLT_GOTREL: u32 = 18;
pub const R_CRIS_32_PLT_PCREL: u32 = 19;


pub const R_CRIS_NUM: u32 = 20;




/* AMD x86-64 relocations.  */


pub const R_X86_64_NONE: u32 = 0;
pub const R_X86_64_64: u32 = 1;
pub const R_X86_64_PC32: u32 = 2;
pub const R_X86_64_GOT32: u32 = 3;
pub const R_X86_64_PLT32: u32 = 4;
pub const R_X86_64_COPY: u32 = 5;
pub const R_X86_64_GLOB_DAT: u32 = 6;
pub const R_X86_64_JUMP_SLOT: u32 = 7;
pub const R_X86_64_RELATIVE: u32 = 8;




pub const R_X86_64_32: u32 = 10;
pub const R_X86_64_32S: u32 = 11;
pub const R_X86_64_16: u32 = 12;
pub const R_X86_64_PC16: u32 = 13;
pub const R_X86_64_8: u32 = 14;
pub const R_X86_64_PC8: u32 = 15;
pub const R_X86_64_DTPMOD64: u32 = 16;
pub const R_X86_64_DTPOFF64: u32 = 17;
pub const R_X86_64_TPOFF64: u32 = 18;








pub const R_X86_64_DTPOFF32: u32 = 21;




pub const R_X86_64_TPOFF32: u32 = 23;
pub const R_X86_64_PC64: u32 = 24;
pub const R_X86_64_GOTOFF64: u32 = 25;




pub const R_X86_64_GOT64: u32 = 27;




pub const R_X86_64_GOTPC64: u32 = 29;
pub const R_X86_64_GOTPLT64: u32 = 30;




pub const R_X86_64_SIZE32: u32 = 32;
pub const R_X86_64_SIZE64: u32 = 33;
pub const R_X86_64_GOTPC32_TLSDESC: u32 = 34;




pub const R_X86_64_TLSDESC: u32 = 36;
pub const R_X86_64_IRELATIVE: u32 = 37;
pub const R_X86_64_RELATIVE64: u32 = 38;


pub const R_X86_64_NUM: u32 = 39;




/* AM33 relocations.  */


pub const R_MN10300_NONE: u32 = 0;
pub const R_MN10300_32: u32 = 1;
pub const R_MN10300_16: u32 = 2;
pub const R_MN10300_8: u32 = 3;
pub const R_MN10300_PCREL32: u32 = 4;
pub const R_MN10300_PCREL16: u32 = 5;
pub const R_MN10300_PCREL8: u32 = 6;
pub const R_MN10300_GNU_VTINHERIT: u32 = 7;
pub const R_MN10300_GNU_VTENTRY: u32 = 8;
pub const R_MN10300_24: u32 = 9;
pub const R_MN10300_GOTPC32: u32 = 10;
pub const R_MN10300_GOTPC16: u32 = 11;
pub const R_MN10300_GOTOFF32: u32 = 12;
pub const R_MN10300_GOTOFF24: u32 = 13;
pub const R_MN10300_GOTOFF16: u32 = 14;
pub const R_MN10300_PLT32: u32 = 15;
pub const R_MN10300_PLT16: u32 = 16;
pub const R_MN10300_GOT32: u32 = 17;
pub const R_MN10300_GOT24: u32 = 18;
pub const R_MN10300_GOT16: u32 = 19;
pub const R_MN10300_COPY: u32 = 20;
pub const R_MN10300_GLOB_DAT: u32 = 21;
pub const R_MN10300_JMP_SLOT: u32 = 22;
pub const R_MN10300_RELATIVE: u32 = 23;


pub const R_MN10300_NUM: u32 = 24;




/* M32R relocs.  */


pub const R_M32R_NONE: u32 = 0;
pub const R_M32R_16: u32 = 1;
pub const R_M32R_32: u32 = 2;
pub const R_M32R_24: u32 = 3;
pub const R_M32R_10_PCREL: u32 = 4;
pub const R_M32R_18_PCREL: u32 = 5;
pub const R_M32R_26_PCREL: u32 = 6;
pub const R_M32R_HI16_ULO: u32 = 7;
pub const R_M32R_HI16_SLO: u32 = 8;
pub const R_M32R_LO16: u32 = 9;
pub const R_M32R_SDA16: u32 = 10;
pub const R_M32R_GNU_VTINHERIT: u32 = 11;
pub const R_M32R_GNU_VTENTRY: u32 = 12;
/* M32R relocs use SHT_RELA.  */


pub const R_M32R_16_RELA: u32 = 33;
pub const R_M32R_32_RELA: u32 = 34;
pub const R_M32R_24_RELA: u32 = 35;
pub const R_M32R_10_PCREL_RELA: u32 = 36;
pub const R_M32R_18_PCREL_RELA: u32 = 37;
pub const R_M32R_26_PCREL_RELA: u32 = 38;
pub const R_M32R_HI16_ULO_RELA: u32 = 39;
pub const R_M32R_HI16_SLO_RELA: u32 = 40;
pub const R_M32R_LO16_RELA: u32 = 41;
pub const R_M32R_SDA16_RELA: u32 = 42;
pub const R_M32R_RELA_GNU_VTINHERIT: u32 = 43;
pub const R_M32R_RELA_GNU_VTENTRY: u32 = 44;
pub const R_M32R_REL32: u32 = 45;


pub const R_M32R_GOT24: u32 = 48;
pub const R_M32R_26_PLTREL: u32 = 49;
pub const R_M32R_COPY: u32 = 50;
pub const R_M32R_GLOB_DAT: u32 = 51;
pub const R_M32R_JMP_SLOT: u32 = 52;
pub const R_M32R_RELATIVE: u32 = 53;
pub const R_M32R_GOTOFF: u32 = 54;
pub const R_M32R_GOTPC24: u32 = 55;








pub const R_M32R_GOT16_LO: u32 = 58;




















pub const R_M32R_GOTOFF_LO: u32 = 64;
pub const R_M32R_NUM: u32 = 256;




/* TILEPro relocations.  */


pub const R_TILEPRO_NONE: u32 = 0;
pub const R_TILEPRO_32: u32 = 1;
pub const R_TILEPRO_16: u32 = 2;
pub const R_TILEPRO_8: u32 = 3;
pub const R_TILEPRO_32_PCREL: u32 = 4;
pub const R_TILEPRO_16_PCREL: u32 = 5;
pub const R_TILEPRO_8_PCREL: u32 = 6;
pub const R_TILEPRO_LO16: u32 = 7;
pub const R_TILEPRO_HI16: u32 = 8;
pub const R_TILEPRO_HA16: u32 = 9;
pub const R_TILEPRO_COPY: u32 = 10;
pub const R_TILEPRO_GLOB_DAT: u32 = 11;
pub const R_TILEPRO_JMP_SLOT: u32 = 12;
pub const R_TILEPRO_RELATIVE: u32 = 13;
pub const R_TILEPRO_BROFF_X1: u32 = 14;
pub const R_TILEPRO_JOFFLONG_X1: u32 = 15;
pub const R_TILEPRO_JOFFLONG_X1_PLT: u32 = 16;
pub const R_TILEPRO_IMM8_X0: u32 = 17;
pub const R_TILEPRO_IMM8_Y0: u32 = 18;
pub const R_TILEPRO_IMM8_X1: u32 = 19;
pub const R_TILEPRO_IMM8_Y1: u32 = 20;
pub const R_TILEPRO_MT_IMM15_X1: u32 = 21;
pub const R_TILEPRO_MF_IMM15_X1: u32 = 22;
pub const R_TILEPRO_IMM16_X0: u32 = 23;
pub const R_TILEPRO_IMM16_X1: u32 = 24;
pub const R_TILEPRO_IMM16_X0_LO: u32 = 25;
pub const R_TILEPRO_IMM16_X1_LO: u32 = 26;
pub const R_TILEPRO_IMM16_X0_HI: u32 = 27;
pub const R_TILEPRO_IMM16_X1_HI: u32 = 28;
pub const R_TILEPRO_IMM16_X0_HA: u32 = 29;
pub const R_TILEPRO_IMM16_X1_HA: u32 = 30;
pub const R_TILEPRO_IMM16_X0_PCREL: u32 = 31;
pub const R_TILEPRO_IMM16_X1_PCREL: u32 = 32;
pub const R_TILEPRO_IMM16_X0_LO_PCREL: u32 = 33;
pub const R_TILEPRO_IMM16_X1_LO_PCREL: u32 = 34;
pub const R_TILEPRO_IMM16_X0_HI_PCREL: u32 = 35;
pub const R_TILEPRO_IMM16_X1_HI_PCREL: u32 = 36;
pub const R_TILEPRO_IMM16_X0_HA_PCREL: u32 = 37;
pub const R_TILEPRO_IMM16_X1_HA_PCREL: u32 = 38;
pub const R_TILEPRO_IMM16_X0_GOT: u32 = 39;
pub const R_TILEPRO_IMM16_X1_GOT: u32 = 40;
pub const R_TILEPRO_IMM16_X0_GOT_LO: u32 = 41;
pub const R_TILEPRO_IMM16_X1_GOT_LO: u32 = 42;
pub const R_TILEPRO_IMM16_X0_GOT_HI: u32 = 43;
pub const R_TILEPRO_IMM16_X1_GOT_HI: u32 = 44;
pub const R_TILEPRO_IMM16_X0_GOT_HA: u32 = 45;
pub const R_TILEPRO_IMM16_X1_GOT_HA: u32 = 46;
pub const R_TILEPRO_MMSTART_X0: u32 = 47;
pub const R_TILEPRO_MMEND_X0: u32 = 48;
pub const R_TILEPRO_MMSTART_X1: u32 = 49;
pub const R_TILEPRO_MMEND_X1: u32 = 50;
pub const R_TILEPRO_SHAMT_X0: u32 = 51;
pub const R_TILEPRO_SHAMT_X1: u32 = 52;
pub const R_TILEPRO_SHAMT_Y0: u32 = 53;
pub const R_TILEPRO_SHAMT_Y1: u32 = 54;
pub const R_TILEPRO_DEST_IMM8_X1: u32 = 55;
/* Relocs 56-59 are currently not defined.  */


pub const R_TILEPRO_TLS_GD_CALL: u32 = 60;
pub const R_TILEPRO_IMM8_X0_TLS_GD_ADD: u32 = 61;
pub const R_TILEPRO_IMM8_X1_TLS_GD_ADD: u32 = 62;
pub const R_TILEPRO_IMM8_Y0_TLS_GD_ADD: u32 = 63;
pub const R_TILEPRO_IMM8_Y1_TLS_GD_ADD: u32 = 64;
pub const R_TILEPRO_TLS_IE_LOAD: u32 = 65;
pub const R_TILEPRO_IMM16_X0_TLS_GD: u32 = 66;
pub const R_TILEPRO_IMM16_X1_TLS_GD: u32 = 67;
pub const R_TILEPRO_IMM16_X0_TLS_GD_LO: u32 = 68;
pub const R_TILEPRO_IMM16_X1_TLS_GD_LO: u32 = 69;
pub const R_TILEPRO_IMM16_X0_TLS_GD_HI: u32 = 70;
pub const R_TILEPRO_IMM16_X1_TLS_GD_HI: u32 = 71;
pub const R_TILEPRO_IMM16_X0_TLS_GD_HA: u32 = 72;
pub const R_TILEPRO_IMM16_X1_TLS_GD_HA: u32 = 73;
pub const R_TILEPRO_IMM16_X0_TLS_IE: u32 = 74;
pub const R_TILEPRO_IMM16_X1_TLS_IE: u32 = 75;
pub const R_TILEPRO_IMM16_X0_TLS_IE_LO: u32 = 76;
pub const R_TILEPRO_IMM16_X1_TLS_IE_LO: u32 = 77;
pub const R_TILEPRO_IMM16_X0_TLS_IE_HI: u32 = 78;
pub const R_TILEPRO_IMM16_X1_TLS_IE_HI: u32 = 79;
pub const R_TILEPRO_IMM16_X0_TLS_IE_HA: u32 = 80;
pub const R_TILEPRO_IMM16_X1_TLS_IE_HA: u32 = 81;
pub const R_TILEPRO_TLS_DTPMOD32: u32 = 82;
pub const R_TILEPRO_TLS_DTPOFF32: u32 = 83;
pub const R_TILEPRO_TLS_TPOFF32: u32 = 84;
pub const R_TILEPRO_IMM16_X0_TLS_LE: u32 = 85;
pub const R_TILEPRO_IMM16_X1_TLS_LE: u32 = 86;
pub const R_TILEPRO_IMM16_X0_TLS_LE_LO: u32 = 87;
pub const R_TILEPRO_IMM16_X1_TLS_LE_LO: u32 = 88;
pub const R_TILEPRO_IMM16_X0_TLS_LE_HI: u32 = 89;
pub const R_TILEPRO_IMM16_X1_TLS_LE_HI: u32 = 90;
pub const R_TILEPRO_IMM16_X0_TLS_LE_HA: u32 = 91;
pub const R_TILEPRO_IMM16_X1_TLS_LE_HA: u32 = 92;


pub const R_TILEPRO_GNU_VTINHERIT: u32 = 128;
pub const R_TILEPRO_GNU_VTENTRY: u32 = 129;


pub const R_TILEPRO_NUM: u32 = 130;




/* TILE-Gx relocations.  */


pub const R_TILEGX_NONE: u32 = 0;
pub const R_TILEGX_64: u32 = 1;
pub const R_TILEGX_32: u32 = 2;
pub const R_TILEGX_16: u32 = 3;
pub const R_TILEGX_8: u32 = 4;
pub const R_TILEGX_64_PCREL: u32 = 5;
pub const R_TILEGX_32_PCREL: u32 = 6;
pub const R_TILEGX_16_PCREL: u32 = 7;
pub const R_TILEGX_8_PCREL: u32 = 8;
pub const R_TILEGX_HW0: u32 = 9;
pub const R_TILEGX_HW1: u32 = 10;
pub const R_TILEGX_HW2: u32 = 11;
pub const R_TILEGX_HW3: u32 = 12;
pub const R_TILEGX_HW0_LAST: u32 = 13;
pub const R_TILEGX_HW1_LAST: u32 = 14;
pub const R_TILEGX_HW2_LAST: u32 = 15;
pub const R_TILEGX_COPY: u32 = 16;
pub const R_TILEGX_GLOB_DAT: u32 = 17;
pub const R_TILEGX_JMP_SLOT: u32 = 18;
pub const R_TILEGX_RELATIVE: u32 = 19;
pub const R_TILEGX_BROFF_X1: u32 = 20;
pub const R_TILEGX_JUMPOFF_X1: u32 = 21;
pub const R_TILEGX_JUMPOFF_X1_PLT: u32 = 22;
pub const R_TILEGX_IMM8_X0: u32 = 23;
pub const R_TILEGX_IMM8_Y0: u32 = 24;
pub const R_TILEGX_IMM8_X1: u32 = 25;
pub const R_TILEGX_IMM8_Y1: u32 = 26;
pub const R_TILEGX_DEST_IMM8_X1: u32 = 27;
pub const R_TILEGX_MT_IMM14_X1: u32 = 28;
pub const R_TILEGX_MF_IMM14_X1: u32 = 29;
pub const R_TILEGX_MMSTART_X0: u32 = 30;
pub const R_TILEGX_MMEND_X0: u32 = 31;
pub const R_TILEGX_SHAMT_X0: u32 = 32;
pub const R_TILEGX_SHAMT_X1: u32 = 33;
pub const R_TILEGX_SHAMT_Y0: u32 = 34;
pub const R_TILEGX_SHAMT_Y1: u32 = 35;
pub const R_TILEGX_IMM16_X0_HW0: u32 = 36;
pub const R_TILEGX_IMM16_X1_HW0: u32 = 37;
pub const R_TILEGX_IMM16_X0_HW1: u32 = 38;
pub const R_TILEGX_IMM16_X1_HW1: u32 = 39;
pub const R_TILEGX_IMM16_X0_HW2: u32 = 40;
pub const R_TILEGX_IMM16_X1_HW2: u32 = 41;
pub const R_TILEGX_IMM16_X0_HW3: u32 = 42;
pub const R_TILEGX_IMM16_X1_HW3: u32 = 43;
pub const R_TILEGX_IMM16_X0_HW0_LAST: u32 = 44;
pub const R_TILEGX_IMM16_X1_HW0_LAST: u32 = 45;
pub const R_TILEGX_IMM16_X0_HW1_LAST: u32 = 46;
pub const R_TILEGX_IMM16_X1_HW1_LAST: u32 = 47;
pub const R_TILEGX_IMM16_X0_HW2_LAST: u32 = 48;
pub const R_TILEGX_IMM16_X1_HW2_LAST: u32 = 49;
pub const R_TILEGX_IMM16_X0_HW0_PCREL: u32 = 50;
pub const R_TILEGX_IMM16_X1_HW0_PCREL: u32 = 51;
pub const R_TILEGX_IMM16_X0_HW1_PCREL: u32 = 52;
pub const R_TILEGX_IMM16_X1_HW1_PCREL: u32 = 53;
pub const R_TILEGX_IMM16_X0_HW2_PCREL: u32 = 54;
pub const R_TILEGX_IMM16_X1_HW2_PCREL: u32 = 55;
pub const R_TILEGX_IMM16_X0_HW3_PCREL: u32 = 56;
pub const R_TILEGX_IMM16_X1_HW3_PCREL: u32 = 57;
pub const R_TILEGX_IMM16_X0_HW0_LAST_PCREL: u32 = 58;
pub const R_TILEGX_IMM16_X1_HW0_LAST_PCREL: u32 = 59;
pub const R_TILEGX_IMM16_X0_HW1_LAST_PCREL: u32 = 60;
pub const R_TILEGX_IMM16_X1_HW1_LAST_PCREL: u32 = 61;
pub const R_TILEGX_IMM16_X0_HW2_LAST_PCREL: u32 = 62;
pub const R_TILEGX_IMM16_X1_HW2_LAST_PCREL: u32 = 63;
pub const R_TILEGX_IMM16_X0_HW0_GOT: u32 = 64;
pub const R_TILEGX_IMM16_X1_HW0_GOT: u32 = 65;
/* Relocs 66-71 are currently not defined.  */


pub const R_TILEGX_IMM16_X0_HW0_LAST_GOT: u32 = 72;
pub const R_TILEGX_IMM16_X1_HW0_LAST_GOT: u32 = 73;
pub const R_TILEGX_IMM16_X0_HW1_LAST_GOT: u32 = 74;
pub const R_TILEGX_IMM16_X1_HW1_LAST_GOT: u32 = 75;
/* Relocs 76-77 are currently not defined.  */


pub const R_TILEGX_IMM16_X0_HW0_TLS_GD: u32 = 78;
pub const R_TILEGX_IMM16_X1_HW0_TLS_GD: u32 = 79;
pub const R_TILEGX_IMM16_X0_HW0_TLS_LE: u32 = 80;
pub const R_TILEGX_IMM16_X1_HW0_TLS_LE: u32 = 81;
pub const R_TILEGX_IMM16_X0_HW0_LAST_TLS_LE: u32 = 82;
pub const R_TILEGX_IMM16_X1_HW0_LAST_TLS_LE: u32 = 83;
pub const R_TILEGX_IMM16_X0_HW1_LAST_TLS_LE: u32 = 84;
pub const R_TILEGX_IMM16_X1_HW1_LAST_TLS_LE: u32 = 85;
pub const R_TILEGX_IMM16_X0_HW0_LAST_TLS_GD: u32 = 86;
pub const R_TILEGX_IMM16_X1_HW0_LAST_TLS_GD: u32 = 87;
pub const R_TILEGX_IMM16_X0_HW1_LAST_TLS_GD: u32 = 88;
pub const R_TILEGX_IMM16_X1_HW1_LAST_TLS_GD: u32 = 89;
/* Relocs 90-91 are currently not defined.  */


pub const R_TILEGX_IMM16_X0_HW0_TLS_IE: u32 = 92;
pub const R_TILEGX_IMM16_X1_HW0_TLS_IE: u32 = 93;
/* Relocs 94-99 are currently not defined.  */


pub const R_TILEGX_IMM16_X0_HW0_LAST_TLS_IE: u32 = 100;
pub const R_TILEGX_IMM16_X1_HW0_LAST_TLS_IE: u32 = 101;
pub const R_TILEGX_IMM16_X0_HW1_LAST_TLS_IE: u32 = 102;
pub const R_TILEGX_IMM16_X1_HW1_LAST_TLS_IE: u32 = 103;
/* Relocs 104-105 are currently not defined.  */


pub const R_TILEGX_TLS_DTPMOD64: u32 = 106;
pub const R_TILEGX_TLS_DTPOFF64: u32 = 107;
pub const R_TILEGX_TLS_TPOFF64: u32 = 108;
pub const R_TILEGX_TLS_DTPMOD32: u32 = 109;
pub const R_TILEGX_TLS_DTPOFF32: u32 = 110;
pub const R_TILEGX_TLS_TPOFF32: u32 = 111;
pub const R_TILEGX_TLS_GD_CALL: u32 = 112;
pub const R_TILEGX_IMM8_X0_TLS_GD_ADD: u32 = 113;
pub const R_TILEGX_IMM8_X1_TLS_GD_ADD: u32 = 114;
pub const R_TILEGX_IMM8_Y0_TLS_GD_ADD: u32 = 115;
pub const R_TILEGX_IMM8_Y1_TLS_GD_ADD: u32 = 116;
pub const R_TILEGX_TLS_IE_LOAD: u32 = 117;
pub const R_TILEGX_IMM8_X0_TLS_ADD: u32 = 118;
pub const R_TILEGX_IMM8_X1_TLS_ADD: u32 = 119;
pub const R_TILEGX_IMM8_Y0_TLS_ADD: u32 = 120;
pub const R_TILEGX_IMM8_Y1_TLS_ADD: u32 = 121;


pub const R_TILEGX_GNU_VTINHERIT: u32 = 128;
pub const R_TILEGX_GNU_VTENTRY: u32 = 129;


pub const R_TILEGX_NUM: u32 = 130;










