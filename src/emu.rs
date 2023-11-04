use crate::instr_repr::{Operand, Verb};

pub struct CpuEmu {
    instrs: Vec<Verb>,
    ip: u16,
    halted: bool,
    regs: [i16; 16],
    mem: [i16; 65536],
}

impl CpuEmu {
    pub fn new(instrs: Vec<Verb>) -> Self {
        CpuEmu {
            instrs,
            ip: 0,
            halted: false,
            regs: [-2; 16],
            mem: [-2; 65536], // we use -2 to make it easier to spot errors of uninitialized memory
        }
    }

    pub fn get_gfx_buffer(&self) -> &[i16] {
        return &self.mem.as_slice()[0..1200];
    }

    pub fn get_led_output(&self) -> i16 {
        return self.mem[1204];
    }

    pub fn set_switch_states(&mut self, new_states: i16) {
        self.mem[1200] = new_states;
    }

    pub fn set_button_states(&mut self, new_states: i16) {
        self.mem[1201] = new_states;
    }

    pub fn run_some_instructions(&mut self) {
        // runs 1000 instructions
        for _ in 0..1000 {
            let next_instr = self
                .instrs
                .get(self.ip as usize)
                .expect("program execution continued into undefined instructions!");
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
                            self.mem[self.regs[reg2.to_id() as usize] as u16 as usize];
                    }
                    (Operand::MemAtReg(reg1), Operand::Reg(reg2)) => {
                        self.mem[self.regs[reg1.to_id() as usize] as u16 as usize] =
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
                        self.regs[ra.to_id() as usize] = self.regs[ra.to_id() as usize]
                            .overflowing_add(self.regs[rb.to_id() as usize])
                            .0;
                    } else {
                        self.regs[ra.to_id() as usize] = self.regs[ra.to_id() as usize]
                            .overflowing_add(op2.to_imm() as i16)
                            .0;
                    }
                }
                Verb::Sub(op1, op2) => {
                    let ra = op1.to_reg();
                    if let Operand::Reg(rb) = op2 {
                        self.regs[ra.to_id() as usize] = self.regs[ra.to_id() as usize]
                            .overflowing_sub(self.regs[rb.to_id() as usize])
                            .0;
                    } else {
                        self.regs[ra.to_id() as usize] = self.regs[ra.to_id() as usize]
                            .overflowing_sub(op2.to_imm() as i16)
                            .0;
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
                            println!("memory from 0x{:X} to 0x{:X}:", addr1, addr2);
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
                    if !self.halted {
                        self.halted = true;
                        println!("program halting.");
                    }
                    return;
                }
                Verb::Call(imm) => {
                    // store current IP value
                    let rsp = self.regs[0] as u16 as usize;
                    self.mem[rsp] = self.ip as i16;
                    // increment rsp
                    self.regs[0] = self.regs[0].overflowing_add(1).0;

                    // jump to new address minus one (because IP gets incremented at end of each cycle)
                    self.ip = imm.to_imm().overflowing_sub(1).0;
                }
                Verb::Ret => {
                    // decrement rsp
                    self.regs[0] = self.regs[0].overflowing_sub(1).0;
                    // read address to return to
                    let rsp = self.regs[0] as u16 as usize;
                    // jump there, let execution continue (so we jump to ret addr and not (ret addr) - 1)
                    self.ip = self.mem[rsp] as u16;
                }
                Verb::Push(reg) => {
                    // store reg to [rsp]
                    let rsp = self.regs[0] as u16 as usize;
                    self.mem[rsp] = self.regs[reg.to_reg().to_id() as usize];
                    // increment rsp
                    self.regs[0] = self.regs[0].overflowing_add(1).0;
                }
                Verb::Pop(reg) => {
                    // decrement rsp
                    self.regs[0] = self.regs[0].overflowing_sub(1).0;

                    // load reg from [rsp]
                    let rsp = self.regs[0] as u16 as usize;
                    self.regs[reg.to_reg().to_id() as usize] = self.mem[rsp];
                }
                Verb::Ldstk(reg, imm) => {
                    let loc = (self.regs[0] as u16).overflowing_sub(imm.to_imm() as u16).0 as usize;
                    self.regs[reg.to_reg().to_id() as usize] = self.mem[loc];
                }
                Verb::Ststk(reg, imm) => {
                    let loc = (self.regs[0] as u16).overflowing_sub(imm.to_imm() as u16).0 as usize;
                    self.mem[loc] = self.regs[reg.to_reg().to_id() as usize];
                }
            }
            self.ip = self.ip.overflowing_add(1).0;
        }
    }
}
