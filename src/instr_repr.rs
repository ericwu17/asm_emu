use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Verb {
    Mov(Operand, Operand),
    Jmp(Operand),

    Jz(Operand, Operand),
    Jnz(Operand, Operand),
    Jpos(Operand, Operand),
    Jposz(Operand, Operand),
    Jneg(Operand, Operand),
    Jnegz(Operand, Operand),

    Setz(Operand, Operand),
    Setnz(Operand, Operand),
    Setpos(Operand, Operand),
    Setposz(Operand, Operand),
    Setneg(Operand, Operand),
    Setnegz(Operand, Operand),

    Add(Operand, Operand),
    Sub(Operand, Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Not(Operand),
    Shl(Operand, Operand),
    Shr(Operand, Operand),

    Dbg(Operand),
    DbgRegs,
    Nop,
    Halt,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Reg {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operand {
    Reg(Reg),
    Imm(u16),
    Label(String),
    MemAtReg(Reg),
    MemAtImm(u16),
}

impl Operand {
    pub fn to_reg(&self) -> Reg {
        match self {
            Operand::Reg(r) => *r,
            _ => panic!(),
        }
    }

    pub fn to_imm(&self) -> u16 {
        match self {
            Operand::Imm(imm) => *imm,
            _ => panic!(),
        }
    }
}

impl Reg {
    fn to_id(&self) -> u8 {
        match self {
            Reg::R0 => 0,
            Reg::R1 => 1,
            Reg::R2 => 2,
            Reg::R3 => 3,
            Reg::R4 => 4,
            Reg::R5 => 5,
            Reg::R6 => 6,
            Reg::R7 => 7,
            Reg::R8 => 8,
            Reg::R9 => 9,
            Reg::R10 => 10,
            Reg::R11 => 11,
            Reg::R12 => 12,
            Reg::R13 => 13,
            Reg::R14 => 14,
            Reg::R15 => 15,
        }
    }

    fn write_into_byte_lower(&self, b: &mut u8) {
        *b &= 0xF0;
        *b |= self.to_id();
    }

    fn write_into_byte_upper(&self, b: &mut u8) {
        *b &= 0x0F;
        *b |= self.to_id() << 4;
    }
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "R{}", self.to_id())
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Reg(r) => write!(f, "{}", r),
            Operand::Imm(v) => write!(f, "0x{:X}", v),
            Operand::Label(label_name) => write!(f, "{}", label_name),
            Operand::MemAtReg(r) => write!(f, "[{}]", r),
            Operand::MemAtImm(v) => write!(f, "[0x{:X}]", v),
        }
    }
}

impl fmt::Display for Verb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Verb::Mov(o1, o2) => write!(f, "mov {} {}", o1, o2),
            Verb::Jmp(o1) => write!(f, "jmp {} ", o1),
            Verb::Jz(o1, o2) => write!(f, "jz {} {}", o1, o2),
            Verb::Jnz(o1, o2) => write!(f, "jnz {} {}", o1, o2),
            Verb::Jpos(o1, o2) => write!(f, "jpos {} {}", o1, o2),
            Verb::Jposz(o1, o2) => write!(f, "jposz {} {}", o1, o2),
            Verb::Jneg(o1, o2) => write!(f, "jneg {} {}", o1, o2),
            Verb::Jnegz(o1, o2) => write!(f, "jnegz {} {}", o1, o2),
            Verb::Setz(o1, o2) => write!(f, "setz {} {}", o1, o2),
            Verb::Setnz(o1, o2) => write!(f, "setnz {} {}", o1, o2),
            Verb::Setpos(o1, o2) => write!(f, "setpos {} {}", o1, o2),
            Verb::Setposz(o1, o2) => write!(f, "setposz {} {}", o1, o2),
            Verb::Setneg(o1, o2) => write!(f, "setneg {} {}", o1, o2),
            Verb::Setnegz(o1, o2) => write!(f, "setnegz {} {}", o1, o2),
            Verb::Add(o1, o2) => write!(f, "add {} {}", o1, o2),
            Verb::Sub(o1, o2) => write!(f, "sub {} {}", o1, o2),
            Verb::And(o1, o2) => write!(f, "and {} {}", o1, o2),
            Verb::Or(o1, o2) => write!(f, "or {} {}", o1, o2),
            Verb::Not(o1) => write!(f, "not {}", o1),
            Verb::Shl(o1, o2) => write!(f, "shl {} {}", o1, o2),
            Verb::Shr(o1, o2) => write!(f, "shr {} {}", o1, o2),
            Verb::Dbg(o1) => write!(f, "dbg {}", o1),
            Verb::DbgRegs => write!(f, "dbg"),
            Verb::Nop => write!(f, "nop"),
            Verb::Halt => write!(f, "halt"),
        }
    }
}

impl Verb {
    pub fn to_bytes(&self) -> [u8; 3] {
        let mut res = [0, 0, 0];

        match self {
            Verb::Mov(op1, op2) => match (op1, op2) {
                (Operand::Reg(r1), Operand::Imm(imm)) => {
                    res[0] = 0x10;
                    r1.write_into_byte_lower(&mut res[0]);
                    [res[1], res[2]] = imm.to_be_bytes();
                }
                (Operand::Reg(r1), Operand::MemAtImm(imm)) => {
                    res[0] = 0x20;
                    r1.write_into_byte_lower(&mut res[0]);
                    [res[1], res[2]] = imm.to_be_bytes();
                }
                (Operand::MemAtImm(imm), Operand::Reg(r1)) => {
                    res[0] = 0x30;
                    r1.write_into_byte_lower(&mut res[0]);
                    [res[1], res[2]] = imm.to_be_bytes();
                }
                (Operand::Reg(r1), Operand::Reg(r2)) => {
                    res[0] = 0xF0;
                    res[1] = 0x00;
                    r1.write_into_byte_upper(&mut res[2]);
                    r2.write_into_byte_lower(&mut res[2]);
                }
                (Operand::Reg(ra), Operand::MemAtReg(rb)) => {
                    res[0] = 0xF0;
                    res[1] = 0x01;
                    ra.write_into_byte_upper(&mut res[2]);
                    rb.write_into_byte_lower(&mut res[2]);
                }
                (Operand::MemAtReg(ra), Operand::Reg(rb)) => {
                    res[0] = 0xF0;
                    res[1] = 0x02;
                    ra.write_into_byte_upper(&mut res[2]);
                    rb.write_into_byte_lower(&mut res[2]);
                }
                _ => unreachable!(),
            },

            Verb::Jmp(operand) => {
                res[0] = 0xE3;
                [res[1], res[2]] = operand.to_imm().to_be_bytes();
            }
            Verb::Jz(imm, r)
            | Verb::Jnz(imm, r)
            | Verb::Jpos(imm, r)
            | Verb::Jposz(imm, r)
            | Verb::Jneg(imm, r)
            | Verb::Jnegz(imm, r) => {
                res[0] = match self {
                    Verb::Jz(_, _) => 0x40,
                    Verb::Jnz(_, _) => 0x50,
                    Verb::Jpos(_, _) => 0x60,
                    Verb::Jposz(_, _) => 0x70,
                    Verb::Jneg(_, _) => 0x80,
                    Verb::Jnegz(_, _) => 0x90,
                    _ => unreachable!(),
                };
                r.to_reg().write_into_byte_lower(&mut res[0]);
                [res[1], res[2]] = imm.to_imm().to_be_bytes();
            }

            Verb::Setz(ra, rb)
            | Verb::Setnz(ra, rb)
            | Verb::Setpos(ra, rb)
            | Verb::Setposz(ra, rb)
            | Verb::Setneg(ra, rb)
            | Verb::Setnegz(ra, rb) => {
                res[0] = 0xF0;
                res[1] = match self {
                    Verb::Setz(_, _) => 0x10,
                    Verb::Setnz(_, _) => 0x11,
                    Verb::Setpos(_, _) => 0x12,
                    Verb::Setposz(_, _) => 0x13,
                    Verb::Setneg(_, _) => 0x14,
                    Verb::Setnegz(_, _) => 0x15,
                    _ => unreachable!(),
                };
                ra.to_reg().write_into_byte_upper(&mut res[2]);
                rb.to_reg().write_into_byte_lower(&mut res[2]);
            }
            Verb::Add(op1, op2)
            | Verb::Sub(op1, op2)
            | Verb::And(op1, op2)
            | Verb::Or(op1, op2) => match (op1, op2) {
                (Operand::Reg(r1), Operand::Reg(r2)) => {
                    res[0] = 0xF0;
                    res[1] = match self {
                        Verb::Add(..) => 0x20,
                        Verb::Sub(..) => 0x21,
                        Verb::And(..) => 0x22,
                        Verb::Or(..) => 0x23,
                        _ => unreachable!(),
                    };
                    r1.write_into_byte_upper(&mut res[2]);
                    r2.write_into_byte_lower(&mut res[2]);
                }
                (Operand::Reg(r1), Operand::Imm(imm)) => {
                    res[0] = match self {
                        Verb::Add(..) => 0xA0,
                        Verb::Sub(..) => 0xB0,
                        Verb::And(..) => 0xC0,
                        Verb::Or(..) => 0xD0,
                        _ => unreachable!(),
                    };
                    r1.write_into_byte_lower(&mut res[0]);
                    [res[1], res[2]] = imm.to_be_bytes();
                }
                _ => unreachable!(),
            },

            Verb::Not(r) => {
                res[0] = 0xF0;
                res[1] = 0x24;
                r.to_reg().write_into_byte_upper(&mut res[2]);
            }
            Verb::Shl(op1, op2) | Verb::Shr(op1, op2) => match (op1, op2) {
                (Operand::Reg(r1), Operand::Reg(r2)) => {
                    res[0] = 0xF0;
                    res[1] = match self {
                        Verb::Shl(..) => 0x31,
                        Verb::Shr(..) => 0x33,
                        _ => unreachable!(),
                    };
                    r1.write_into_byte_upper(&mut res[2]);
                    r2.write_into_byte_lower(&mut res[2]);
                }
                (Operand::Reg(r), Operand::Imm(imm)) => {
                    res[0] = 0xF0;
                    res[1] = match self {
                        Verb::Shl(..) => 0x30,
                        Verb::Shr(..) => 0x32,
                        _ => unreachable!(),
                    };
                    r.write_into_byte_upper(&mut res[2]);
                    write_imm_to_byte_lower(*imm, &mut res[2]);
                }
                _ => unreachable!(),
            },

            Verb::Dbg(op) => {
                res[0] = 0xE0;
                [res[1], res[2]] = op.to_imm().to_be_bytes();
            }
            Verb::DbgRegs => {
                res[0] = 0xE1;
            }
            Verb::Nop => {}
            Verb::Halt => {
                res[0] = 0xFF;
                res[1] = 0xFF;
                res[2] = 0xFF;
            }
        }
        res
    }

    pub fn as_hex_file_line(&self) -> String {
        let bytes = self.to_bytes();

        // format as hex, with padding to left
        // https://doc.rust-lang.org/std/fmt/
        format!(
            "{:0>2X}_{:0>2X}_{:0>2X}  // {}",
            bytes[0], bytes[1], bytes[2], self
        )
    }
}

pub fn write_imm_to_byte_lower(imm: u16, b: &mut u8) {
    *b &= 0xF0;
    *b |= imm.to_be_bytes()[1] & 0x0F;
}
