mod emu;
mod graphics;
mod instr_repr;
mod label_resolver;
mod location_resolver;
mod source_cursor;
mod tokens;

use std::fs::File;
use std::io::Read;

use clap::Parser;
use emu::CpuEmu;
use graphics::{draw_leds, draw_monitor, draw_switches, get_curr_button_states};
use macroquad::prelude::*;
use tokens::get_tokens;

use crate::label_resolver::resolve_labels;
use crate::location_resolver::create_location_map;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of input file containing assembly
    filename: String,
}

// const OUT_FILE_NAME: &str = "seq.code";

#[macroquad::main("Assembler Emulator")]
async fn main() {
    let cli = Cli::parse();
    let input_filepath = cli.filename;

    let mut contents = String::new();
    File::open(&input_filepath)
        .expect(&format!("could not open file: {}", &input_filepath))
        .read_to_string(&mut contents)
        .expect(&format!("error reading file: {}", &input_filepath));

    let var_loc_map = create_location_map("vars.locations");

    let (mut verbs, map) = get_tokens(contents, &var_loc_map);

    resolve_labels(&mut verbs, &map);

    for verb in &verbs {
        println!("{}", verb.as_hex_file_line());
    }

    let mut cpu_emulator = CpuEmu::new(verbs);
    let mut curr_switch_states = 0i16;

    loop {
        clear_background(LIGHTGRAY);

        let gfx_buf = cpu_emulator.get_gfx_buffer();

        draw_monitor(10.0, 10.0, 640.0, 480.0, gfx_buf).await;

        draw_leds(10.0, 500.0, cpu_emulator.get_led_output()).await;
        draw_switches(10.0, 520.0, &mut curr_switch_states).await;
        cpu_emulator.set_switch_states(curr_switch_states);
        cpu_emulator.set_button_states(get_curr_button_states().await);

        cpu_emulator.run_some_instructions();

        next_frame().await;
    }
}
