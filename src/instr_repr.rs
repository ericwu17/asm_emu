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
