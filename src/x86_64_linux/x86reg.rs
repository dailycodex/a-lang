#![allow(unused)]
#![allow(clippy::upper_case_acronyms)]
use std::fmt;

/*
Monikers 	Description
                        (of lower 16 bits)
64-bit 	32-bit 	16-bit 	8 high bits 	8-bit
RAX 	EAX 	AX 	    AH 	            AL 	    Accumulator
RBX 	EBX 	BX 	    BH 	            BL 	    Base
RCX 	ECX 	CX 	    CH 	            CL 	    Counter
RDX 	EDX 	DX 	    DH 	            DL 	    Data (commonly extends the A register)
RSI 	ESI 	SI 	    N/A 	        SIL 	Source index for string operations
RDI 	EDI 	DI 	    N/A 	        DIL 	Destination index for string operations
RSP 	ESP 	SP 	    N/A 	        SPL 	Stack Pointer
RBP 	EBP 	BP 	    N/A 	        BPL 	Base Pointer (meant for stack frames)
R8 	    R8D 	R8W 	N/A 	        R8B 	General purpose
R9 	    R9D 	R9W 	N/A 	        R9B 	General purpose
R10 	R10D 	R10W 	N/A 	        R10B 	General purpose
R11 	R11D 	R11W 	N/A 	        R11B 	General purpose
R12 	R12D 	R12W 	N/A 	        R12B 	General purpose
R13 	R13D 	R13W 	N/A 	        R13B 	General purpose
R14 	R14D 	R14W 	N/A 	        R14B 	General purpose
R15 	R15D 	R15W 	N/A 	        R15B 	General purpose
 */

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum X86Reg {
    RegRet(X86RegRet),
    RegParam(X86RegParam),
    Reg64(X86Reg64),
    Reg32(X86Reg32),
    Reg16(X86Reg16),
    RegHigh8(X86RegHigh8),
    RegLow8(X86RegLow8),
}

impl fmt::Display for X86Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RegRet(reg) => write!(f, "{}", reg.as_64_bit()),
            Self::RegParam(reg) => write!(f, "{}", reg.as_64_bit()),
            Self::Reg64(reg) => write!(f, "{reg}"),
            Self::Reg32(reg) => write!(f, "{reg}"),
            Self::Reg16(reg) => write!(f, "{reg}"),
            Self::RegHigh8(reg) => write!(f, "{reg}"),
            Self::RegLow8(reg) => write!(f, "{reg}"),
        }
    }
}

impl X86Reg {
    pub fn as_32_bit(&self) -> X86Reg32 {
        match self {
            Self::RegRet(reg) => reg.as_64_bit().as_32_bit(),
            Self::RegParam(reg) => reg.as_64_bit().as_32_bit(),
            Self::Reg64(reg) => reg.as_32_bit(),
            Self::Reg32(reg) => *reg,
            Self::Reg16(reg) => reg.as_32_bit(),
            Self::RegHigh8(reg) => reg.as_32_bit(),
            Self::RegLow8(reg) => reg.as_32_bit(),
        }
    }

    pub fn as_16_bit(&self) -> X86Reg16 {
        match self {
            Self::RegRet(reg) => reg.as_64_bit().as_16_bit(),
            Self::RegParam(reg) => reg.as_64_bit().as_16_bit(),
            Self::Reg64(reg) => reg.as_16_bit(),
            Self::Reg32(reg) => reg.as_16_bit(),
            Self::Reg16(reg) => *reg,
            Self::RegHigh8(reg) => reg.as_16_bit(),
            Self::RegLow8(reg) => reg.as_16_bit(),
        }
    }
    pub fn as_high_8_bit(&self) -> X86RegHigh8 {
        match self {
            Self::RegRet(reg) => reg.as_64_bit().as_high_8_bit().unwrap(),
            Self::RegParam(reg) => reg.as_64_bit().as_high_8_bit().unwrap(),
            Self::Reg64(reg) => reg.as_high_8_bit().unwrap(),
            Self::Reg32(reg) => reg.as_high_8_bit().unwrap(),
            Self::Reg16(reg) => reg.as_high_8_bit().unwrap(),
            Self::RegHigh8(reg) => *reg,
            Self::RegLow8(reg) => reg.as_high_8_bit().unwrap(),
        }
    }
    pub fn as_low_8_bit(&self) -> X86RegLow8 {
        match self {
            Self::RegRet(reg) => reg.as_64_bit().as_low_8_bit(),
            Self::RegParam(reg) => reg.as_64_bit().as_low_8_bit(),
            Self::Reg64(reg) => reg.as_low_8_bit(),
            Self::Reg32(reg) => reg.as_low_8_bit(),
            Self::Reg16(reg) => reg.as_low_8_bit(),
            Self::RegHigh8(reg) => reg.as_low_8_bit(),
            Self::RegLow8(reg) => *reg,
        }
    }
}

impl From<usize> for X86Reg {
    fn from(value: usize) -> Self {
        X86RegParam::from(value).into()
    }
}

impl From<X86RegRet> for X86Reg {
    fn from(value: X86RegRet) -> Self {
        Self::RegRet(value)
    }
}

impl From<X86RegParam> for X86Reg {
    fn from(value: X86RegParam) -> Self {
        Self::RegParam(value)
    }
}

impl From<X86Reg64> for X86Reg {
    fn from(value: X86Reg64) -> Self {
        Self::Reg64(value)
    }
}

impl From<X86Reg32> for X86Reg {
    fn from(value: X86Reg32) -> Self {
        Self::Reg32(value)
    }
}

impl From<X86Reg16> for X86Reg {
    fn from(value: X86Reg16) -> Self {
        Self::Reg16(value)
    }
}

impl From<X86RegHigh8> for X86Reg {
    fn from(value: X86RegHigh8) -> Self {
        Self::RegHigh8(value)
    }
}

impl From<X86RegLow8> for X86Reg {
    fn from(value: X86RegLow8) -> Self {
        Self::RegLow8(value)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum X86RegRet {
    RAX,
    RDI,
}

impl From<usize> for X86RegRet {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::RAX,
            1 => Self::RDI,
            _ => unreachable!(),
        }
    }
}

impl X86RegRet {
    pub fn as_64_bit(&self) -> X86Reg64 {
        match self {
            Self::RAX => X86Reg64::RAX,
            Self::RDI => X86Reg64::RDI,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum X86RegParam {
    RDI,
    RSI,
    RDX,
    RCX,
    R8,
    R9,
}

impl From<usize> for X86RegParam {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::RDI,
            1 => Self::RSI,
            2 => Self::RDX,
            3 => Self::RCX,
            4 => Self::R8,
            5 => Self::R9,
            _ => unreachable!(),
        }
    }
}

impl X86RegParam {
    pub fn as_64_bit(&self) -> X86Reg64 {
        match self {
            Self::RDI => X86Reg64::RDI,
            Self::RSI => X86Reg64::RSI,
            Self::RDX => X86Reg64::RDX,
            Self::RCX => X86Reg64::RCX,
            Self::R8 => X86Reg64::R8,
            Self::R9 => X86Reg64::R9,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum X86Reg64 {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RSP,
    RBP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl fmt::Display for X86Reg64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl X86Reg64 {
    pub fn as_32_bit(&self) -> X86Reg32 {
        match self {
            Self::RAX => X86Reg32::EAX,
            Self::RBX => X86Reg32::EBX,
            Self::RCX => X86Reg32::ECX,
            Self::RDX => X86Reg32::EDX,
            Self::RSI => X86Reg32::ESI,
            Self::RDI => X86Reg32::EDI,
            Self::RSP => X86Reg32::ESP,
            Self::RBP => X86Reg32::EBP,
            Self::R8 => X86Reg32::R8D,
            Self::R9 => X86Reg32::R9D,
            Self::R10 => X86Reg32::R10D,
            Self::R11 => X86Reg32::R11D,
            Self::R12 => X86Reg32::R12D,
            Self::R13 => X86Reg32::R13D,
            Self::R14 => X86Reg32::R14D,
            Self::R15 => X86Reg32::R15D,
        }
    }
    pub fn as_16_bit(&self) -> X86Reg16 {
        match self {
            Self::RAX => X86Reg16::AX,
            Self::RBX => X86Reg16::BX,
            Self::RCX => X86Reg16::CX,
            Self::RDX => X86Reg16::DX,
            Self::RSI => X86Reg16::SI,
            Self::RDI => X86Reg16::DI,
            Self::RSP => X86Reg16::SP,
            Self::RBP => X86Reg16::BP,
            Self::R8 => X86Reg16::R8W,
            Self::R9 => X86Reg16::R9W,
            Self::R10 => X86Reg16::R10W,
            Self::R11 => X86Reg16::R11W,
            Self::R12 => X86Reg16::R12W,
            Self::R13 => X86Reg16::R13W,
            Self::R14 => X86Reg16::R14W,
            Self::R15 => X86Reg16::R15W,
        }
    }
    pub fn as_high_8_bit(&self) -> Option<X86RegHigh8> {
        match self {
            Self::RAX => Some(X86RegHigh8::AH),
            Self::RBX => Some(X86RegHigh8::BH),
            Self::RCX => Some(X86RegHigh8::CH),
            Self::RDX => Some(X86RegHigh8::DH),
            _ => None,
        }
    }
    pub fn as_low_8_bit(&self) -> X86RegLow8 {
        match self {
            Self::RAX => X86RegLow8::AL,
            Self::RBX => X86RegLow8::BL,
            Self::RCX => X86RegLow8::CL,
            Self::RDX => X86RegLow8::DL,
            Self::RSI => X86RegLow8::SIL,
            Self::RDI => X86RegLow8::DIL,
            Self::RSP => X86RegLow8::SPL,
            Self::RBP => X86RegLow8::BPL,
            Self::R8 => X86RegLow8::R8B,
            Self::R9 => X86RegLow8::R9B,
            Self::R10 => X86RegLow8::R10B,
            Self::R11 => X86RegLow8::R11B,
            Self::R12 => X86RegLow8::R12B,
            Self::R13 => X86RegLow8::R13B,
            Self::R14 => X86RegLow8::R14B,
            Self::R15 => X86RegLow8::R15B,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum X86Reg32 {
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    ESP,
    EBP,
    R8D,
    R9D,
    R10D,
    R11D,
    R12D,
    R13D,
    R14D,
    R15D,
}

impl fmt::Display for X86Reg32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl X86Reg32 {
    pub fn as_64_bit(&self) -> X86Reg64 {
        match self {
            Self::EAX => X86Reg64::RAX,
            Self::EBX => X86Reg64::RBX,
            Self::ECX => X86Reg64::RCX,
            Self::EDX => X86Reg64::RDX,
            Self::ESI => X86Reg64::RSI,
            Self::EDI => X86Reg64::RDI,
            Self::ESP => X86Reg64::RSP,
            Self::EBP => X86Reg64::RBP,
            Self::R8D => X86Reg64::R8,
            Self::R9D => X86Reg64::R9,
            Self::R10D => X86Reg64::R10,
            Self::R11D => X86Reg64::R11,
            Self::R12D => X86Reg64::R12,
            Self::R13D => X86Reg64::R13,
            Self::R14D => X86Reg64::R14,
            Self::R15D => X86Reg64::R15,
        }
    }
    pub fn as_16_bit(&self) -> X86Reg16 {
        match self {
            Self::EAX => X86Reg16::AX,
            Self::EBX => X86Reg16::BX,
            Self::ECX => X86Reg16::CX,
            Self::EDX => X86Reg16::DX,
            Self::ESI => X86Reg16::SI,
            Self::EDI => X86Reg16::DI,
            Self::ESP => X86Reg16::SP,
            Self::EBP => X86Reg16::BP,
            Self::R8D => X86Reg16::R8W,
            Self::R9D => X86Reg16::R9W,
            Self::R10D => X86Reg16::R10W,
            Self::R11D => X86Reg16::R11W,
            Self::R12D => X86Reg16::R12W,
            Self::R13D => X86Reg16::R13W,
            Self::R14D => X86Reg16::R14W,
            Self::R15D => X86Reg16::R15W,
        }
    }
    pub fn as_high_8_bit(&self) -> Option<X86RegHigh8> {
        match self {
            Self::EAX => Some(X86RegHigh8::AH),
            Self::EBX => Some(X86RegHigh8::BH),
            Self::ECX => Some(X86RegHigh8::CH),
            Self::EDX => Some(X86RegHigh8::DH),
            _ => None,
        }
    }
    pub fn as_low_8_bit(&self) -> X86RegLow8 {
        match self {
            Self::EAX => X86RegLow8::AL,
            Self::EBX => X86RegLow8::BL,
            Self::ECX => X86RegLow8::CL,
            Self::EDX => X86RegLow8::DL,
            Self::ESI => X86RegLow8::SIL,
            Self::EDI => X86RegLow8::DIL,
            Self::ESP => X86RegLow8::SPL,
            Self::EBP => X86RegLow8::BPL,
            Self::R8D => X86RegLow8::R8B,
            Self::R9D => X86RegLow8::R9B,
            Self::R10D => X86RegLow8::R10B,
            Self::R11D => X86RegLow8::R11B,
            Self::R12D => X86RegLow8::R12B,
            Self::R13D => X86RegLow8::R13B,
            Self::R14D => X86RegLow8::R14B,
            Self::R15D => X86RegLow8::R15B,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum X86Reg16 {
    AX,
    BX,
    CX,
    DX,
    SI,
    DI,
    SP,
    BP,
    R8W,
    R9W,
    R10W,
    R11W,
    R12W,
    R13W,
    R14W,
    R15W,
}

impl fmt::Display for X86Reg16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl X86Reg16 {
    pub fn as_64_bit(&self) -> X86Reg64 {
        match self {
            Self::AX => X86Reg64::RAX,
            Self::BX => X86Reg64::RBX,
            Self::CX => X86Reg64::RCX,
            Self::DX => X86Reg64::RDX,
            Self::SI => X86Reg64::RSI,
            Self::DI => X86Reg64::RDI,
            Self::SP => X86Reg64::RSP,
            Self::BP => X86Reg64::RBP,
            Self::R8W => X86Reg64::R8,
            Self::R9W => X86Reg64::R9,
            Self::R10W => X86Reg64::R10,
            Self::R11W => X86Reg64::R11,
            Self::R12W => X86Reg64::R12,
            Self::R13W => X86Reg64::R13,
            Self::R14W => X86Reg64::R14,
            Self::R15W => X86Reg64::R15,
        }
    }
    pub fn as_32_bit(&self) -> X86Reg32 {
        match self {
            Self::AX => X86Reg32::EAX,
            Self::BX => X86Reg32::EBX,
            Self::CX => X86Reg32::ECX,
            Self::DX => X86Reg32::EDX,
            Self::SI => X86Reg32::ESI,
            Self::DI => X86Reg32::EDI,
            Self::SP => X86Reg32::ESP,
            Self::BP => X86Reg32::EBP,
            Self::R8W => X86Reg32::R8D,
            Self::R9W => X86Reg32::R9D,
            Self::R10W => X86Reg32::R10D,
            Self::R11W => X86Reg32::R11D,
            Self::R12W => X86Reg32::R12D,
            Self::R13W => X86Reg32::R13D,
            Self::R14W => X86Reg32::R14D,
            Self::R15W => X86Reg32::R15D,
        }
    }
    pub fn as_high_8_bit(&self) -> Option<X86RegHigh8> {
        match self {
            Self::AX => Some(X86RegHigh8::AH),
            Self::BX => Some(X86RegHigh8::BH),
            Self::CX => Some(X86RegHigh8::CH),
            Self::DX => Some(X86RegHigh8::DH),
            _ => None,
        }
    }
    pub fn as_low_8_bit(&self) -> X86RegLow8 {
        match self {
            Self::AX => X86RegLow8::AL,
            Self::BX => X86RegLow8::BL,
            Self::CX => X86RegLow8::CL,
            Self::DX => X86RegLow8::DL,
            Self::SI => X86RegLow8::SIL,
            Self::DI => X86RegLow8::DIL,
            Self::SP => X86RegLow8::SPL,
            Self::BP => X86RegLow8::BPL,
            Self::R8W => X86RegLow8::R8B,
            Self::R9W => X86RegLow8::R9B,
            Self::R10W => X86RegLow8::R10B,
            Self::R11W => X86RegLow8::R11B,
            Self::R12W => X86RegLow8::R12B,
            Self::R13W => X86RegLow8::R13B,
            Self::R14W => X86RegLow8::R14B,
            Self::R15W => X86RegLow8::R15B,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum X86RegHigh8 {
    AH,
    BH,
    CH,
    DH,
}

impl fmt::Display for X86RegHigh8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl X86RegHigh8 {
    pub fn as_64_bit(&self) -> X86Reg64 {
        match self {
            Self::AH => X86Reg64::RAX,
            Self::BH => X86Reg64::RBX,
            Self::CH => X86Reg64::RCX,
            Self::DH => X86Reg64::RDX,
            _ => unreachable!(),
        }
    }
    pub fn as_32_bit(&self) -> X86Reg32 {
        match self {
            Self::AH => X86Reg32::EAX,
            Self::BH => X86Reg32::EBX,
            Self::CH => X86Reg32::ECX,
            Self::DH => X86Reg32::EDX,
            _ => unreachable!(),
        }
    }
    pub fn as_16_bit(&self) -> X86Reg16 {
        match self {
            Self::AH => X86Reg16::AX,
            Self::BH => X86Reg16::BX,
            Self::CH => X86Reg16::CX,
            Self::DH => X86Reg16::DX,
            _ => unreachable!(),
        }
    }
    pub fn as_low_8_bit(&self) -> X86RegLow8 {
        match self {
            Self::AH => X86RegLow8::AL,
            Self::BH => X86RegLow8::BL,
            Self::CH => X86RegLow8::CL,
            Self::DH => X86RegLow8::DL,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum X86RegLow8 {
    AL,
    BL,
    CL,
    DL,
    SIL,
    DIL,
    SPL,
    BPL,
    R8B,
    R9B,
    R10B,
    R11B,
    R12B,
    R13B,
    R14B,
    R15B,
}

impl fmt::Display for X86RegLow8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl X86RegLow8 {
    pub fn as_64_bit(&self) -> X86Reg64 {
        match self {
            Self::AL => X86Reg64::RAX,
            Self::BL => X86Reg64::RBX,
            Self::CL => X86Reg64::RCX,
            Self::DL => X86Reg64::RDX,
            Self::SIL => X86Reg64::RSI,
            Self::DIL => X86Reg64::RDI,
            Self::SPL => X86Reg64::RSP,
            Self::BPL => X86Reg64::RBP,
            Self::R8B => X86Reg64::R8,
            Self::R9B => X86Reg64::R9,
            Self::R10B => X86Reg64::R10,
            Self::R11B => X86Reg64::R11,
            Self::R12B => X86Reg64::R12,
            Self::R13B => X86Reg64::R13,
            Self::R14B => X86Reg64::R14,
            Self::R15B => X86Reg64::R15,
        }
    }
    pub fn as_32_bit(&self) -> X86Reg32 {
        match self {
            Self::AL => X86Reg32::EAX,
            Self::BL => X86Reg32::EBX,
            Self::CL => X86Reg32::ECX,
            Self::DL => X86Reg32::EDX,
            Self::SIL => X86Reg32::ESI,
            Self::DIL => X86Reg32::EDI,
            Self::SPL => X86Reg32::ESP,
            Self::BPL => X86Reg32::EBP,
            Self::R8B => X86Reg32::R8D,
            Self::R9B => X86Reg32::R9D,
            Self::R10B => X86Reg32::R10D,
            Self::R11B => X86Reg32::R11D,
            Self::R12B => X86Reg32::R12D,
            Self::R13B => X86Reg32::R13D,
            Self::R14B => X86Reg32::R14D,
            Self::R15B => X86Reg32::R15D,
        }
    }

    pub fn as_16_bit(&self) -> X86Reg16 {
        match self {
            Self::AL => X86Reg16::AX,
            Self::BL => X86Reg16::BX,
            Self::CL => X86Reg16::CX,
            Self::DL => X86Reg16::DX,
            Self::SIL => X86Reg16::SI,
            Self::DIL => X86Reg16::DI,
            Self::SPL => X86Reg16::SP,
            Self::BPL => X86Reg16::BP,
            Self::R8B => X86Reg16::R8W,
            Self::R9B => X86Reg16::R9W,
            Self::R10B => X86Reg16::R10W,
            Self::R11B => X86Reg16::R11W,
            Self::R12B => X86Reg16::R12W,
            Self::R13B => X86Reg16::R13W,
            Self::R14B => X86Reg16::R14W,
            Self::R15B => X86Reg16::R15W,
        }
    }
    pub fn as_high_8_bit(&self) -> Option<X86RegHigh8> {
        match self {
            Self::AL => Some(X86RegHigh8::AH),
            Self::BL => Some(X86RegHigh8::BH),
            Self::CL => Some(X86RegHigh8::CH),
            Self::DL => Some(X86RegHigh8::DH),
            _ => None,
        }
    }
}
