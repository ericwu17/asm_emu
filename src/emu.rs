use crate::instr_repr::{Operand, Verb};

pub struct CpuEmu {
    instrs: Vec<Verb>,
    ip: u16,
    regs: [i16; 16],
    mem: [i16; 65536],
}

impl CpuEmu {
    pub fn new(instrs: Vec<Verb>) -> Self {
        CpuEmu {
            instrs,
            ip: 0,
            regs: [-258; 16],
            mem: [-258; 65536], // we use -258 to make it easier to spot errors of uninitialized memory
        }
    }

    pub fn run(&mut self) {
        loop {
            let next_instr = self.instrs.get(self.ip as usize).unwrap();
            match next_instr {
                Verb::Mov(op1, op2) => match (op1, op2) {
                    (Operand::Reg(reg), Operand::Imm(imm)) => {
                        self.regs[reg.to_id() as usize] = *imm as i16;
                    }
                    (Operand::Reg(reg), Operand::MemAtImm(imm)) => {
                        self.regs[reg.to_id() as usize] = self.mem[*imm as usize];
                    }
                    (Operand::MemAtImm(imm), Operand::Reg(reg)) => {
                        self.mem[*imm as usize] = self.regs[reg.to_id() as usize];
                    }
                    (Operand::Reg(reg1), Operand::Reg(reg2)) => {
                        self.regs[reg1.to_id() as usize] = self.regs[reg2.to_id() as usize];
                    }
                    (Operand::Reg(reg1), Operand::MemAtReg(reg2)) => {
                        self.regs[reg1.to_id() as usize] =
                            self.mem[self.regs[reg2.to_id() as usize] as usize];
                    }
                    (Operand::MemAtReg(reg1), Operand::Reg(reg2)) => {
                        self.mem[self.regs[reg1.to_id() as usize] as usize] =
                            self.regs[reg2.to_id() as usize];
                    }
                    _ => unreachable!(),
                },
                Verb::Jmp(imm) => {
                    self.ip = imm.to_imm().overflowing_sub(1).0;
                }
                Verb::Jz(imm, reg)
                | Verb::Jnz(imm, reg)
                | Verb::Jpos(imm, reg)
                | Verb::Jposz(imm, reg)
                | Verb::Jneg(imm, reg)
                | Verb::Jnegz(imm, reg) => {
                    let imm = imm.to_imm();
                    let reg = reg.to_reg();
                    let reg_value = self.regs[reg.to_id() as usize];

                    let jump_taken = match next_instr {
                        Verb::Jz(..) => reg_value == 0,
                        Verb::Jnz(..) => reg_value != 0,
                        Verb::Jpos(..) => reg_value > 0,
                        Verb::Jposz(..) => reg_value >= 0,
                        Verb::Jneg(..) => reg_value < 0,
                        Verb::Jnegz(..) => reg_value <= 0,
                        _ => unreachable!(),
                    };
                    if jump_taken {
                        self.ip = imm.overflowing_sub(1).0;
                    }
                }
                Verb::Setz(ra, rb)
                | Verb::Setnz(ra, rb)
                | Verb::Setpos(ra, rb)
                | Verb::Setposz(ra, rb)
                | Verb::Setneg(ra, rb)
                | Verb::Setnegz(ra, rb) => {
                    let ra = ra.to_reg();
                    let rb = rb.to_reg();
                    let rb_val = self.regs[rb.to_id() as usize];

                    let cond_satisfied = match next_instr {
                        Verb::Setz(..) => rb_val == 0,
                        Verb::Setnz(..) => rb_val != 0,
                        Verb::Setpos(..) => rb_val > 0,
                        Verb::Setposz(..) => rb_val >= 0,
                        Verb::Setneg(..) => rb_val < 0,
                        Verb::Setnegz(..) => rb_val <= 0,
                        _ => unreachable!(),
                    };
                    self.regs[ra.to_id() as usize] = if cond_satisfied { 1 } else { 0 };
                }
                Verb::Add(op1, op2) => {
                    let ra = op1.to_reg();
                    if let Operand::Reg(rb) = op2 {
                        self.regs[ra.to_id() as usize] += self.regs[rb.to_id() as usize];
                    } else {
                        self.regs[ra.to_id() as usize] += op2.to_imm() as i16;
                    }
                }
                Verb::Sub(op1, op2) => {
                    let ra = op1.to_reg();
                    if let Operand::Reg(rb) = op2 {
                        self.regs[ra.to_id() as usize] -= self.regs[rb.to_id() as usize];
                    } else {
                        self.regs[ra.to_id() as usize] -= op2.to_imm() as i16;
                    }
                }
                Verb::And(op1, op2) => {
                    let ra = op1.to_reg();
                    if let Operand::Reg(rb) = op2 {
                        self.regs[ra.to_id() as usize] &= self.regs[rb.to_id() as usize];
                    } else {
                        self.regs[ra.to_id() as usize] &= op2.to_imm() as i16;
                    }
                }
                Verb::Or(op1, op2) => {
                    let ra = op1.to_reg();
                    if let Operand::Reg(rb) = op2 {
                        self.regs[ra.to_id() as usize] |= self.regs[rb.to_id() as usize];
                    } else {
                        self.regs[ra.to_id() as usize] |= op2.to_imm() as i16;
                    }
                }
                Verb::Not(ra) => {
                    let ra = ra.to_reg();
                    self.regs[ra.to_id() as usize] = !self.regs[ra.to_id() as usize];
                }
                Verb::Shl(op1, op2) => {
                    let ra = op1.to_reg();
                    if let Operand::Reg(rb) = op2 {
                        self.regs[ra.to_id() as usize] <<= self.regs[rb.to_id() as usize];
                    } else {
                        self.regs[ra.to_id() as usize] <<= op2.to_imm() as i16;
                    }
                }
                Verb::Shr(op1, op2) => {
                    let ra = op1.to_reg();
                    if let Operand::Reg(rb) = op2 {
                        self.regs[ra.to_id() as usize] >>= self.regs[rb.to_id() as usize];
                    } else {
                        self.regs[ra.to_id() as usize] >>= op2.to_imm() as i16;
                    }
                }
                Verb::Dbg(op1) => {
                    self.ip += 1;
                    let next_instr = self.instrs.get(self.ip as usize).unwrap();
                    match next_instr {
                        Verb::Dbg(op2) => {
                            let addr1 = op1.to_imm();
                            let addr2 = op2.to_imm();
                            println!("==========");
                            println!("IP: {}", self.ip);
                            println!("memory from {} to {}:", addr1, addr2);
                            for i in addr1..=addr2 {
                                println!("{}", self.mem[i as usize]);
                            }
                            println!("==========");
                        }
                        _ => panic!("dbg instruction not followed by another!"),
                    }
                }
                Verb::DbgRegs => {
                    println!("==========");
                    println!("IP: {}", self.ip);
                    println!("regs: {:?}", self.regs);
                    println!("==========");
                }
                Verb::Nop => {}
                Verb::Halt => {
                    println!("program halting.");
                    return;
                }
            }
            self.ip = self.ip.overflowing_add(1).0;
        }
    }
}
