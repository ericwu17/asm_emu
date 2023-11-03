mod emu;
mod instr_repr;
mod label_resolver;
mod source_cursor;
mod tokens;

use std::fs::File;
use std::io::Read;

use clap::Parser;
use tokens::get_tokens;

use crate::label_resolver::resolve_labels;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of input file containing assembly
    filename: String,
}

// const OUT_FILE_NAME: &str = "seq.code";

fn main() {
    let cli = Cli::parse();
    let input_filepath = cli.filename;

    let mut contents = String::new();
    File::open(&input_filepath)
        .expect(&format!("could not open file: {}", &input_filepath))
        .read_to_string(&mut contents)
        .expect(&format!("error reading file: {}", &input_filepath));

    let (mut verbs, map) = get_tokens(contents);

    resolve_labels(&mut verbs, &map);

    for verb in verbs {
        println!("{}", verb.as_hex_file_line());
    }
    dbg!(&map);
}
