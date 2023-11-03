use std::collections::HashMap;

use crate::{
    instr_repr::{Operand, Reg, Verb},
    source_cursor::SourceCodeCursor,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Verb(Verb),
    Operand(Operand),
    Label(String),
}

pub fn get_tokens(source_code_contents: String) -> (Vec<Verb>, HashMap<String, u16>) {
    // we return Vec<Vec<Token>> to indicate a list of lines in the source_code,
    // where each line has multiple tokens.
    // The reason for keeping at a line granularity is I want a 1-to-1 correspondence between source
    // code lines and instructions. This way the nth line of assembly code
    // will always get placed in address (n-1). (source code lines start at 1 while memory addresses start at 0)
    // we can generate no-op instructions to pad empty lines.

    // comments start with a ';' and labels start with a '.'

    let mut cursor = SourceCodeCursor::new(source_code_contents);

    let mut label_map = HashMap::new();

    let mut verbs = Vec::new();

    while cursor.peek().is_some() {
        // this loop will consume lines;

        let curr_num_verbs = verbs.len();

        // consume leading whitespace
        consume_whitespace(&mut cursor);
        match cursor.peek() {
            None => break,
            Some('\n') | Some(';') => {
                // empty line. Consume the empty line.
                consume_rest_of_line(&mut cursor);
                verbs.push(Verb::Nop);
            }
            Some('.') => {
                // parse label
                let mut label_name: String = String::new();

                while cursor.peek().is_some() && !cursor.peek().unwrap().is_ascii_whitespace() {
                    label_name.push(cursor.next().unwrap());
                }

                consume_rest_of_line(&mut cursor);

                label_map.insert(label_name, verbs.len() as u16);
                verbs.push(Verb::Nop);
                continue;
            }

            _ => {
                let verb = parse_verb(&mut cursor);

                verbs.push(verb);
            }
        }

        // on every loop, we should be adding exactly one verb.
        assert!(verbs.len() == curr_num_verbs + 1);
    }

    return (verbs, label_map);
}

fn consume_rest_of_line(cursor: &mut SourceCodeCursor) {
    while cursor.peek() != Some('\n') && cursor.peek() != None {
        cursor.next();
    }
    // consume newline if there is one
    cursor.next();
}

fn consume_whitespace(cursor: &mut SourceCodeCursor) {
    while cursor.peek() == Some(' ') || cursor.peek() == Some('\t') {
        cursor.next();
    }
}

fn parse_verb(cursor: &mut SourceCodeCursor) -> Verb {
    let mut verb_name: String = String::new();

    consume_whitespace(cursor);
    while cursor.peek().is_some() && cursor.peek().unwrap().is_alphabetic() {
        verb_name.push(cursor.next().unwrap());
    }

    match verb_name.as_str() {
        "mov" => {
            let operand_1 = parse_operand(cursor);
            let operand_2 = parse_operand(cursor);
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
            let operand = parse_operand(cursor);
            consume_rest_of_line(cursor);
            match operand {
                Some(o1) => match &o1 {
                    Operand::Imm(_) | Operand::Label(_) => return Verb::Jmp(o1),
                    _ => panic!("invalid operands for jmp"),
                },
                _ => panic!("not enough operands for jmp"),
            }
        }
        "jz" | "jnz" | "jpos" | "jposz" | "jneg" | "jnegz" => {
            let operand_1 = parse_operand(cursor);
            let operand_2 = parse_operand(cursor);
            consume_rest_of_line(cursor);
            match (operand_1, operand_2) {
                (Some(o1), Some(o2)) => match (&o1, &o2) {
                    (Operand::Imm(_) | Operand::Label(_), Operand::Reg(_)) => {
                        match verb_name.as_str() {
                            "jz" => return Verb::Jz(o1, o2),
                            "jnz" => return Verb::Jnz(o1, o2),
                            "jpos" => return Verb::Jpos(o1, o2),
                            "jposz" => return Verb::Jposz(o1, o2),
                            "jneg" => return Verb::Jneg(o1, o2),
                            "jnegz" => return Verb::Jnegz(o1, o2),
                            _ => unreachable!(),
                        }
                    }
                    _ => panic!("invalid operands for jz"),
                },
                _ => panic!("not enough operands for jz"),
            }
        }

        "setz" | "setnz" | "setpos" | "setposz" | "setneg" | "setnegz" => {
            let operand_1 = parse_operand(cursor);
            let operand_2 = parse_operand(cursor);
            consume_rest_of_line(cursor);
            match (operand_1, operand_2) {
                (Some(o1), Some(o2)) => match (&o1, &o2) {
                    (Operand::Reg(_), Operand::Reg(_)) => match verb_name.as_str() {
                        "setz" => return Verb::Setz(o1, o2),
                        "setnz" => return Verb::Setnz(o1, o2),
                        "setpos" => return Verb::Setpos(o1, o2),
                        "setposz" => return Verb::Setposz(o1, o2),
                        "setneg" => return Verb::Setneg(o1, o2),
                        "setnegz" => return Verb::Setnegz(o1, o2),
                        _ => unreachable!(),
                    },
                    _ => panic!("invalid operands for conditional set"),
                },
                _ => panic!("not enough operands for conditional set"),
            }
        }

        "add" | "sub" | "and" | "or" | "shl" | "shr" => {
            let operand_1 = parse_operand(cursor);
            let operand_2 = parse_operand(cursor);
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
            let operand = parse_operand(cursor);
            consume_rest_of_line(cursor);
            match operand {
                Some(Operand::Reg(_)) => return Verb::Not(operand.unwrap()),
                _ => panic!("invalid operand for not"),
            }
        }

        "dbg" => {
            let optional_operand = parse_operand(cursor);
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

        _ => panic!("unrecognized verb: {}", verb_name),
    }
}

fn parse_operand(cursor: &mut SourceCodeCursor) -> Option<Operand> {
    consume_whitespace(cursor);

    if cursor.peek() == None || cursor.peek() == Some('\n') {
        return None;
    }

    let mut operand_str: String = String::new();
    while cursor.peek().is_some() && (!cursor.peek().unwrap().is_ascii_whitespace()) {
        operand_str.push(cursor.next().unwrap());
    }

    if let Some(val) = convert_str_to_imm(&operand_str) {
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
        if let Some(val) = convert_str_to_imm(&inner_string) {
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
        "R0" => Some(Reg::R0),
        "R1" => Some(Reg::R1),
        "R2" => Some(Reg::R2),
        "R3" => Some(Reg::R3),
        "R4" => Some(Reg::R4),
        "R5" => Some(Reg::R5),
        "R6" => Some(Reg::R6),
        "R7" => Some(Reg::R7),

        "R8" => Some(Reg::R8),
        "R9" => Some(Reg::R9),
        "R10" => Some(Reg::R10),
        "R11" => Some(Reg::R11),
        "R12" => Some(Reg::R12),
        "R13" => Some(Reg::R13),
        "R14" => Some(Reg::R14),
        "R15" => Some(Reg::R15),
        _ => None,
    }
}

fn convert_str_to_imm(s: &str) -> Option<u16> {
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
