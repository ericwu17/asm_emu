# Assembler and Emulator

This is an assembler and emulator for a custom instruction set.
The instruction set was implemented in Verilog to run on the basys3 FPGA board.
This was my final project for the CS M152A class.

The [src](/src) folder contains the rust code for the emulator and assembler,
while the [cpu_unit](/cpu_unit) folder contains the Verilog code, which can
be opened through the Xilinx Vivado IDE.

The instructions set is very simple, with each instruction encoded in 24 bits. A full list of
supported instructions is:

  - `call` and `ret` for subroutine calls
  - `mov` instructions between registers, to load/store from memory, and load immediates
  - `jmp`, `jz`, and `jnz` are unconditional and conditional jumps.
  - Arithmetic instructions include `add`, `sub`, bitwise `and`, `or`, `not`, and `shl` and `shr`
  for bit shifting
  - There are some debug instructions which are treated by no-ops by hardware, but used to
  dump processor state when executing under emulation.

## Architecture

The instruction set is meant to be run on the basys3 FPGA. I/O is memory-mapped.
The hardware contains 16 16-bit registers and 1 instruction pointer. The register R0
is used as a stack pointer by the CALL and RET instructions, and thus programs must initialize
R0 to a valid address at the beginning of each program. By default, all registers and memory are initialized
to zero.

The CPU's instruction memory is separate from data memory. Furthermore, data memory is addressed
in units of words (16 bits) instead of bytes, and instruction memory is addressed in units of instruction-words
(24 bits).

## Instruction Set

The instruction set is described in the excel file [instruction_set.xlsx](instruction_set.xlsx).
Each instruction is 24 bits, and immediates are 16 bits long.
There is a second sheet in the file called "short_representations" which contains some alternate
representations of the instruction set, with shorter immediates. This would be more efficient,
but was never actually implemented in this emulator or in Verilog.

## Final result: connect 4 game

The file [conn_4.asm](conn_4.asm) has working assembly code to play a 2-player connect 4 game.
This file references some memory locations defined in [vars.locations](vars.locations).
This assembly file can be assembled and emulated by running `cargo run conn_4.asm`. This will
generate an output file `seq.code` containing the assembled results, and also launch the emulator.

The emulator maps the wasd and x keys to the 5 buttons on the basys3 board.
