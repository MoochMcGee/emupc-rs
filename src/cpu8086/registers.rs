use bitflags::bitflags;
#[allow(dead_code)]

bitflags!(
    pub struct Flags: u16
    {
        const CARRY = 0x0001;
        const PARITY = 0x0004;
        const ADJUST = 0x0010;
        const ZERO = 0x0040;
        const SIGN = 0x0080;
        const TRAP = 0x0100;
        const INTERRUPT = 0x0200;
        const DIRECTION = 0x0400;
        const OVERFLOW = 0x0800;
        const DEFAULT = 0xf002;
    }
);

impl Default for Flags {
    fn default() -> Flags {
        Flags::DEFAULT
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Reg8 {
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
}

impl Reg8 {
    pub fn from_num(num: u8) -> Option<Reg8> {
        match num & 7 {
            0 => Some(Reg8::AL),
            1 => Some(Reg8::CL),
            2 => Some(Reg8::DL),
            3 => Some(Reg8::BL),
            4 => Some(Reg8::AH),
            5 => Some(Reg8::CH),
            6 => Some(Reg8::DH),
            7 => Some(Reg8::BH),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Reg16 {
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
    FLAGS,
}

impl Reg16 {
    pub fn from_num(num: u8) -> Option<Reg16> {
        match num & 7 {
            0 => Some(Reg16::AX),
            1 => Some(Reg16::CX),
            2 => Some(Reg16::DX),
            3 => Some(Reg16::BX),
            4 => Some(Reg16::SP),
            5 => Some(Reg16::BP),
            6 => Some(Reg16::SI),
            7 => Some(Reg16::DI),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RegisterType {
    Bits8,
    Bits16,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SegReg {
    ES,
    CS,
    SS,
    DS,
}

impl SegReg {
    pub fn from_num(num: u8) -> Option<SegReg> {
        match num & 3 {
            0 => Some(SegReg::ES),
            1 => Some(SegReg::CS),
            2 => Some(SegReg::SS),
            3 => Some(SegReg::DS),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Registers {
    pub ip: u16,
    pub gprs: [u16; 8],
    pub seg_regs: [u16; 4],
    pub flags: Flags,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            ip: 0,
            gprs: [0; 8],
            seg_regs: [0, 0xffff, 0, 0],
            flags: Flags::default(),
        }
    }

    pub fn read8(&self, reg: Reg8) -> u8 {
        use self::Reg8::*;
        match reg {
            AL => (self.gprs[0] & 0xff) as u8,
            CL => (self.gprs[1] & 0xff) as u8,
            DL => (self.gprs[2] & 0xff) as u8,
            BL => (self.gprs[3] & 0xff) as u8,
            AH => (self.gprs[0] >> 8) as u8,
            CH => (self.gprs[1] >> 8) as u8,
            DH => (self.gprs[2] >> 8) as u8,
            BH => (self.gprs[3] >> 8) as u8,
        }
    }

    pub fn write8(&mut self, reg: Reg8, value: u8) {
        use self::Reg8::*;
        match reg {
            AL => {
                self.gprs[0] &= !0xffu16;
                self.gprs[0] |= value as u16;
            }
            CL => {
                self.gprs[1] &= !0xffu16;
                self.gprs[1] |= value as u16;
            }
            DL => {
                self.gprs[2] &= !0xffu16;
                self.gprs[2] |= value as u16;
            }
            BL => {
                self.gprs[3] &= !0xffu16;
                self.gprs[3] |= value as u16;
            }
            AH => {
                self.gprs[0] &= 0xffu16;
                self.gprs[0] |= (value as u16) << 8;
            }
            CH => {
                self.gprs[1] &= 0xffu16;
                self.gprs[1] |= (value as u16) << 8;
            }
            DH => {
                self.gprs[2] &= 0xffu16;
                self.gprs[2] |= (value as u16) << 8;
            }
            BH => {
                self.gprs[3] &= 0xffu16;
                self.gprs[3] |= (value as u16) << 8;
            }
        }
    }

    pub fn read16(&self, reg: Reg16) -> u16 {
        use self::Reg16::*;
        match reg {
            AX => self.gprs[0],
            CX => self.gprs[1],
            DX => self.gprs[2],
            BX => self.gprs[3],
            SP => self.gprs[4],
            BP => self.gprs[5],
            SI => self.gprs[6],
            DI => self.gprs[7],
            FLAGS => (self.flags.bits() as u16) | 0xf002u16,
        }
    }

    pub fn write16(&mut self, reg: Reg16, value: u16) {
        use self::Reg16::*;
        match reg {
            AX => self.gprs[0] = value,
            CX => self.gprs[1] = value,
            DX => self.gprs[2] = value,
            BX => self.gprs[3] = value,
            SP => self.gprs[4] = value,
            BP => self.gprs[5] = value,
            SI => self.gprs[6] = value,
            DI => self.gprs[7] = value,
            FLAGS => self.flags = Flags::from_bits_truncate(value),
        }
    }

    pub fn readseg16(&self, seg_reg: SegReg) -> u16 {
        use self::SegReg::*;
        match seg_reg {
            ES => self.seg_regs[0],
            CS => self.seg_regs[1],
            SS => self.seg_regs[2],
            DS => self.seg_regs[3],
        }
    }

    pub fn writeseg16(&mut self, seg_reg: SegReg, value: u16) {
        use self::SegReg::*;
        match seg_reg {
            ES => self.seg_regs[0] = value,
            CS => self.seg_regs[1] = value,
            SS => self.seg_regs[2] = value,
            DS => self.seg_regs[3] = value,
        }
    }
}
