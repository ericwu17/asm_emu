use std::collections::HashMap;

use crate::{
    instr_repr::{Operand, Reg, Verb},
    source_cursor::SourceCodeCursor,
};

pub fn get_tokens(
    source_code_contents: String,
    var_loc_map: &HashMap<String, u16>,
) -> (Vec<Verb>, HashMap<String, u16>) {
    let mut cursor = SourceCodeCursor::new(source_code_contents);

    let mut label_map = HashMap::new();

    let mut verbs = Vec::new();

    while cursor.peek().is_some() {
        // this loop will consume one line per iteration:

        // consume leading whitespace
        consume_whitespace(&mut cursor);
        match cursor.peek() {
            None => break,
            Some('\n') | Some(';') => {
                // empty line. Consume the empty line.
                consume_rest_of_line(&mut cursor);
            }
            Some('.') => {
                // parse label
                let mut label_name: String = String::new();

                while cursor.peek().is_some() && !cursor.peek().unwrap().is_ascii_whitespace() {
                    label_name.push(cursor.next().unwrap());
                }

                consume_rest_of_line(&mut cursor);

                label_map.insert(label_name, verbs.len() as u16);
                continue;
            }

            _ => {
                let verb = parse_verb(&mut cursor, var_loc_map);

                verbs.push(verb);
            }
        }
    }

    return (verbs, label_map);
}

pub fn consume_rest_of_line(cursor: &mut SourceCodeCursor) {
    while cursor.peek() != Some('\n') && cursor.peek() != None {
        cursor.next();
    }
    // consume newline if there is one
    cursor.next();
}

pub fn consume_whitespace(cursor: &mut SourceCodeCursor) {
    while cursor.peek() == Some(' ') || cursor.peek() == Some('\t') {
        cursor.next();
    }
}

fn parse_verb(cursor: &mut SourceCodeCursor, var_loc_map: &HashMap<String, u16>) -> Verb {
    let mut verb_name: String = String::new();

    consume_whitespace(cursor);
    while cursor.peek().is_some() && cursor.peek().unwrap().is_alphabetic() {
        verb_name.push(cursor.next().unwrap());
    }

    match verb_name.as_str() {
        "mov" => {
            let operand_1 = parse_operand(cursor, var_loc_map);
            let operand_2 = parse_operand(cursor, var_loc_map);
            consume_rest_of_line(cursor);

            match (operand_1, operand_2) {
                (Some(o1), Some(o2)) => match (&o1, &o2) {
                    (Operand::Reg(_), Operand::Imm(_))
                    | (Operand::Reg(_), Operand::MemAtImm(_))
                    | (Operand::MemAtImm(_), Operand::Reg(_))
                    | (Operand::Reg(_), Operand::Reg(_))
                    | (Operand::Reg(_), Operand::MemAtReg(_))
                    | (Operand::MemAtReg(_), Operand::Reg(_)) => return Verb::Mov(o1, o2),
                    _ => panic!("invalid operands for mov"),
                },
                _ => panic!("not enough operands for mov"),
            }
        }

        "jmp" => {
            let operand = parse_operand(cursor, var_loc_map);
            consume_rest_of_line(cursor);
            match operand {
                Some(o1) => match &o1 {
                    Operand::Imm(_) | Operand::Label(_) => return Verb::Jmp(o1),
                    _ => panic!("invalid operands for jmp"),
                },
                _ => panic!("not enough operands for jmp"),
            }
        }
        "jz" | "jnz" => {
            let operand_1 = parse_operand(cursor, var_loc_map);
            let operand_2 = parse_operand(cursor, var_loc_map);
            consume_rest_of_line(cursor);
            match (operand_1, operand_2) {
                (Some(o1), Some(o2)) => match (&o1, &o2) {
                    (Operand::Imm(_) | Operand::Label(_), Operand::Reg(_)) => {
                        match verb_name.as_str() {
                            "jz" => return Verb::Jz(o1, o2),
                            "jnz" => return Verb::Jnz(o1, o2),
                            _ => unreachable!(),
                        }
                    }
                    _ => panic!("invalid operands for conditional jump"),
                },
                _ => panic!("not enough operands for conditional jump"),
            }
        }

        "add" | "sub" | "and" | "or" | "shl" | "shr" => {
            let operand_1 = parse_operand(cursor, var_loc_map);
            let operand_2 = parse_operand(cursor, var_loc_map);
            consume_rest_of_line(cursor);
            match (operand_1, operand_2) {
                (Some(o1), Some(o2)) => match (&o1, &o2) {
                    (Operand::Reg(_), Operand::Reg(_)) | (Operand::Reg(_), Operand::Imm(_)) => {
                        match verb_name.as_str() {
                            "add" => return Verb::Add(o1, o2),
                            "sub" => return Verb::Sub(o1, o2),
                            "and" => return Verb::And(o1, o2),
                            "or" => return Verb::Or(o1, o2),
                            "shl" => return Verb::Shl(o1, o2),
                            "shr" => return Verb::Shr(o1, o2),
                            _ => unreachable!(),
                        }
                    }
                    _ => panic!("invalid operands for arithmetic operator"),
                },
                _ => panic!("not enough operands for arithmetic operator"),
            }
        }

        "not" => {
            let operand = parse_operand(cursor, var_loc_map);
            consume_rest_of_line(cursor);
            match operand {
                Some(Operand::Reg(_)) => return Verb::Not(operand.unwrap()),
                _ => panic!("invalid operand for not"),
            }
        }

        "dbg" => {
            let optional_operand = parse_operand(cursor, var_loc_map);
            consume_rest_of_line(cursor);
            match optional_operand {
                None => return Verb::DbgRegs,
                Some(operand) => match operand {
                    Operand::Imm(_) => return Verb::Dbg(operand),
                    _ => panic!("invalid operand for debug"),
                },
            }
        }

        "nop" => {
            consume_rest_of_line(cursor);
            return Verb::Nop;
        }
        "halt" => {
            consume_rest_of_line(cursor);
            return Verb::Halt;
        }

        "call" => {
            let operand = parse_operand(cursor, var_loc_map);
            consume_rest_of_line(cursor);
            match operand {
                Some(o1) => match &o1 {
                    Operand::Imm(_) | Operand::Label(_) => return Verb::Call(o1),
                    _ => panic!("invalid operands for call"),
                },
                _ => panic!("not enough operands for call"),
            }
        }
        "ret" => {
            consume_rest_of_line(cursor);
            return Verb::Ret;
        }

        _ => panic!("unrecognized verb: {}", verb_name),
    }
}

fn parse_operand(
    cursor: &mut SourceCodeCursor,
    var_loc_map: &HashMap<String, u16>,
) -> Option<Operand> {
    consume_whitespace(cursor);

    if cursor.peek() == None || cursor.peek() == Some('\n') {
        return None;
    }

    let mut operand_str: String = String::new();
    while cursor.peek().is_some() && (!cursor.peek().unwrap().is_ascii_whitespace()) {
        operand_str.push(cursor.next().unwrap());
    }

    if let Some(val) = convert_str_to_imm(&operand_str, var_loc_map) {
        return Some(Operand::Imm(val));
    }

    if operand_str.starts_with('.') {
        return Some(Operand::Label(operand_str));
    }

    if operand_str.starts_with('[') {
        if !operand_str.ends_with(']') {
            panic!("expected operand `{}` to end with `]`", operand_str);
        }
        let inner_string: String = operand_str
            .chars()
            .skip(1)
            .take(operand_str.len() - 2)
            .collect();
        if let Some(val) = convert_str_to_imm(&inner_string, var_loc_map) {
            return Some(Operand::MemAtImm(val));
        }
        match convert_str_to_reg(&inner_string) {
            Some(reg) => return Some(Operand::MemAtReg(reg)),
            None => panic!(
                "expected either immediate address or register, found {}",
                inner_string
            ),
        }
    }

    if let Some(reg) = convert_str_to_reg(&operand_str) {
        return Some(Operand::Reg(reg));
    }

    panic!("invalid operand: `{}`", operand_str);
}

fn convert_str_to_reg(s: &str) -> Option<Reg> {
    match s {
        "R0" | "r0" => Some(Reg::R0),
        "R1" | "r1" => Some(Reg::R1),
        "R2" | "r2" => Some(Reg::R2),
        "R3" | "r3" => Some(Reg::R3),
        "R4" | "r4" => Some(Reg::R4),
        "R5" | "r5" => Some(Reg::R5),
        "R6" | "r6" => Some(Reg::R6),
        "R7" | "r7" => Some(Reg::R7),

        "R8" | "r8" => Some(Reg::R8),
        "R9" | "r9" => Some(Reg::R9),
        "R10" | "r10" => Some(Reg::R10),
        "R11" | "r11" => Some(Reg::R11),
        "R12" | "r12" => Some(Reg::R12),
        "R13" | "r13" => Some(Reg::R13),
        "R14" | "r14" => Some(Reg::R14),
        "R15" | "r15" => Some(Reg::R15),
        _ => None,
    }
}

pub fn convert_str_to_imm(s: &str, var_loc_map: &HashMap<String, u16>) -> Option<u16> {
    if let Some(val) = var_loc_map.get(s) {
        return Some(*val);
    }

    let parse_res = if s.starts_with("0x") {
        u64::from_str_radix(&s[2..], 16)
    } else {
        // dec
        u64::from_str_radix(s, 10)
    };
    match parse_res {
        Ok(v) => {
            return Some(v as u16);
        }
        Err(_) => None,
    }
}
