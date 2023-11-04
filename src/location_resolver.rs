use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::source_cursor::SourceCodeCursor;
use crate::tokens::{consume_rest_of_line, consume_whitespace, convert_str_to_imm};

pub fn create_location_map(file: &str) -> HashMap<String, u16> {
    let mut contents = String::new();
    File::open(file)
        .expect(&format!("could not open file: {}", file))
        .read_to_string(&mut contents)
        .expect(&format!("error reading file: {}", file));
    let mut cursor = SourceCodeCursor::new(contents);

    let mut map = HashMap::new();

    while cursor.peek().is_some() {
        consume_whitespace(&mut cursor);

        if cursor.peek().is_none() {
            break;
        }
        if cursor.peek() == Some('\n') || cursor.peek() == Some(';') {
            consume_rest_of_line(&mut cursor);
            continue;
        }

        let mut var_name = String::new();
        while cursor.peek().is_some() && !cursor.peek().unwrap().is_ascii_whitespace() {
            var_name.push(cursor.next().unwrap());
        }
        if var_name == "" {
            panic!("should not have empty variable name!")
        }
        consume_whitespace(&mut cursor);

        let mut operand_str: String = String::new();
        while cursor.peek().is_some() && (!cursor.peek().unwrap().is_ascii_whitespace()) {
            operand_str.push(cursor.next().unwrap());
        }

        if let Some(val) = convert_str_to_imm(&operand_str, &HashMap::new()) {
            map.insert(var_name, val);
        } else {
            panic!("invalid operand string: {}", operand_str);
        }

        consume_rest_of_line(&mut cursor);
    }

    map
}
