use std::collections::HashMap;

use crate::instr_repr::{Operand, Verb};

pub fn resolve_labels(instrs: &mut Vec<Verb>, label_map: &HashMap<String, u16>) {
    for verb in instrs {
        match verb {
            Verb::Jmp(operand)
            | Verb::Jz(operand, _)
            | Verb::Jnz(operand, _)
            | Verb::Jpos(operand, _)
            | Verb::Jposz(operand, _)
            | Verb::Jneg(operand, _)
            | Verb::Jnegz(operand, _) => {
                if let Operand::Label(s) = operand {
                    let optional_addr = label_map.get(s);
                    if let Some(addr) = optional_addr {
                        *operand = Operand::Imm(*addr);
                    } else {
                        panic!("unresolved label: {}", s);
                    }
                }
            }

            _ => {}
        }
    }
}
