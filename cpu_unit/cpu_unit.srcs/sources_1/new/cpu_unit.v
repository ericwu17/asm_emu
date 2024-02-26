`timescale 1ns / 1ps
//////////////////////////////////////////////////////////////////////////////////
// Company: 
// Engineer: 
// 
// Create Date: 11/06/2023 03:24:30 PM
// Design Name: 
// Module Name: cpu_unit
// Project Name: 
// Target Devices: 
// Tool Versions: 
// Description: 
// 
// Dependencies: 
// 
// Revision:
// Revision 0.01 - File Created
// Additional Comments:
// 
//////////////////////////////////////////////////////////////////////////////////


module cpu_unit(

    output [3:0] vgaRed,
    output [3:0] vgaBlue,
    output [3:0] vgaGreen,
    output Hsync,
    output Vsync,

    input [15:0] sw,
    output [15:0] led,
    input btnC,
    input btnU,
    input btnD,
    input btnL,
    input btnR,
    input clk
);

  integer i;

  reg [15:0] program_mem [0:1300];
  reg [23:0] instr_mem [0:1023];
  
  
  reg [15:0] registers [0:15];  // 16 16-bit registers
  
  reg [15:0] ip;  // instruction pointer
  
  //reg [2:0] curr_state;  // current state of execution
  
  wire [15:0] r0;   // for simulation debug
  wire [15:0] r1;   // for simulation debug
  wire [15:0] r2;   // for simulation debug
  wire [15:0] r3;   // for simulation debug
  
  wire [15:0] memory_addr_0;  // for simulation debug
  
  
  // begin intermediates
  reg [23:0] curr_instr;
  reg [15:0] instr_imm;
  
  reg read_from_mem = 0;
  reg write_to_mem = 0;
  
  reg [15:0] mem_read_addr = 0;
  reg [15:0] mem_write_addr = 0;
  
  reg [15:0] operand_1 = 0;
  reg [15:0] operand_2 = 0;
  reg [15:0] result = 0;
   
  reg [3:0] reg_a_id;
  reg [3:0] reg_b_id;
  reg [3:0] reg_c_id;
  reg [3:0] reg_d_id;
  reg [3:0] reg_e_id;
  
  // end intermediates
  
  reg [9:0] clk_dv;
  
  reg stage_0_active = 0;
  reg stage_1_active = 0;
  reg stage_2_active = 0;
  reg stage_3_active = 0;
  reg stage_4_active = 0;
  reg stage_5_active = 0;
  reg stage_6_active = 0;
  reg stage_7_active = 1;
  
//  assign stage_0_active = curr_state == 0;
//  assign stage_1_active = curr_state == 1;
//  assign stage_2_active = curr_state == 2;
//  assign stage_3_active = curr_state == 3;
//  assign stage_4_active = curr_state == 4;
//  assign stage_5_active = curr_state == 5;
//  assign stage_6_active = curr_state == 6;
//  assign stage_7_active = curr_state == 7;
    
  wire [19199:0] vga_bitmap;
  
  generate
    genvar x;
    for (x = 0; x < 1200; x = x + 1) begin
      assign vga_bitmap[16*x + 15 : 16*x] = program_mem[x];
    end
  endgenerate
  
  
  assign r0 = registers[0];
  assign r1 = registers[1];
  assign r2 = registers[2];
  assign r3 = registers[3];
  assign memory_addr_0 = program_mem[0];
  
  assign led = program_mem[1204];
  
  initial begin
      for (i = 0; i < 16; i = i + 1) begin
          registers[i] = 0;
      end
      ip = 16'b0;
      curr_instr = 0;
      clk_dv = 0;
  end
  initial $readmemh("seq.code", instr_mem);
  
    
  always @ (posedge clk) begin
    clk_dv <= clk_dv + 1;
  end
  
  
  always @(posedge clk_dv[9]) begin
    stage_0_active <= stage_7_active;
    stage_1_active <= stage_0_active;
    stage_2_active <= stage_1_active;
    stage_3_active <= stage_2_active;
    stage_4_active <= stage_3_active;
    stage_5_active <= stage_4_active;
    stage_6_active <= stage_5_active;
    stage_7_active <= stage_6_active;
  end
  
  
  always @(posedge stage_0_active) begin
    curr_instr <= instr_mem[ip];
  end
  
  always @(posedge stage_1_active) begin
    instr_imm[15:0] <= curr_instr[15:0];
    reg_a_id[3:0] <= curr_instr[19:16];
    reg_b_id[3:0] <= curr_instr[15:12];
    reg_c_id[3:0] <= curr_instr[11:8];
    reg_d_id[3:0] <= curr_instr[7:4];
    reg_e_id[3:0] <= curr_instr[3:0];
  end
 
  always @(posedge stage_2_active) begin
    // stage 2 computes the memory address to be read from
    
    casez (curr_instr) 
        24'b0010_????_????_????_????_????: begin
            mem_read_addr <= instr_imm;
            read_from_mem <= 1;
        end
        24'b1111_0000_0000_0001_????_????: begin
            mem_read_addr <= registers[reg_e_id];
            read_from_mem <= 1;
        end
      	24'b1111_1111_1111_1111_1111_0000: begin
            mem_read_addr <= registers[0] - 1;
            read_from_mem <= 1;
        end
        default: begin
            read_from_mem <= 0;
        end
    endcase
  end
  
  always @ (posedge stage_3_active) begin
    // stage 3: load operands
    if (read_from_mem) begin
        if (mem_read_addr == 16'h04b0) begin
            operand_1 <= sw;
        end else if (mem_read_addr == 16'h04b1) begin
            operand_1 <= {11'b000_0000_0000, btnU, btnD, btnL, btnR, btnC};
        end else begin
            operand_1 <= program_mem[mem_read_addr];
            operand_2 <= registers[0];  // for the RET instruction
        end
    end else begin
        casez (curr_instr)
            24'b0001_????_????_????_????_????: begin
                operand_1 <= instr_imm;
            end
            24'b0011_????_????_????_????_????: begin
                operand_1 <= registers[reg_a_id];
            end
            24'b1111_0000_0000_0000_????_????: begin
                operand_1 <= registers[reg_e_id];
            end
            24'b1111_0000_0000_0010_????_????: begin
                operand_1 <= registers[reg_e_id];
            end
        
            24'b1010_????_????_????_????_????: begin
                operand_1 <= instr_imm;
                operand_2 <= registers[reg_a_id];
            end
            24'b1111_0000_0010_0000_????_????: begin
                operand_2 <= registers[reg_d_id];
                operand_1 <= registers[reg_e_id];
            end
          	24'b1110_0011_????_????_????_????: begin
              	operand_1 <= instr_imm;
            end
          	24'b0100_????_????_????_????_????: begin
                operand_1 <= instr_imm;
                operand_2 <= registers[reg_a_id];
            end
          	24'b0101_????_????_????_????_????: begin
                operand_1 <= instr_imm;
                operand_2 <= registers[reg_a_id];
            end
          	24'b1110_0100_????_????_????_????: begin
              	operand_1 <= instr_imm;
            end

            24'b1011_????_????_????_????_????: begin 
                // SUB IMM
                operand_1 <= instr_imm;
                operand_2 <= registers[reg_a_id];
            end
            24'b1100_????_????_????_????_????: begin 
                // AND IMM
                operand_1 <= instr_imm;
                operand_2 <= registers[reg_a_id];
            end
            24'b1101_????_????_????_????_????: begin 
                // OR IMM
                operand_1 <= instr_imm;
                operand_2 <= registers[reg_a_id];
            end
            24'b1111_0000_0011_0000_????_????: begin 
                // SHL IMM
                operand_1 <= {12'b0, reg_e_id};
                operand_2 <= registers[reg_d_id];
            end
            24'b1111_0000_0011_0010_????_????: begin 
                // SHR IMM
                operand_1 <= {12'b0, reg_e_id};
                operand_2 <= registers[reg_d_id];
            end
            24'b1111_0000_0010_0001_????_????: begin
                // SUB
                operand_2 <= registers[reg_d_id];
                operand_1 <= registers[reg_e_id];
            end
            24'b1111_0000_0010_0010_????_????: begin
                // AND
                operand_2 <= registers[reg_d_id];
                operand_1 <= registers[reg_e_id];
            end
            24'b1111_0000_0010_0011_????_????: begin
                // OR
                operand_2 <= registers[reg_d_id];
                operand_1 <= registers[reg_e_id];
            end
            24'b1111_0000_0010_0100_????_????: begin
                // NOT
                operand_1 <= registers[reg_d_id];
            end
            24'b1111_0000_0011_0001_????_????: begin 
                // SHL REG
                operand_1 <= registers[reg_e_id];
                operand_2 <= registers[reg_d_id];
            end
            24'b1111_0000_0011_0011_????_????: begin 
                // SHR REG
                operand_1 <= registers[reg_e_id];
                operand_2 <= registers[reg_d_id];
            end

        endcase
    end
  end
  
  always @ (posedge stage_4_active) begin
    // stage 4: calculate result
    casez (curr_instr)
        24'b0001_????_????_????_????_????: begin
            result <= operand_1;
        end
        24'b0010_????_????_????_????_????: begin
            result <= operand_1;
        end
        24'b0011_????_????_????_????_????: begin
            result <= operand_1;
        end
        24'b1111_0000_0000_0000_????_????: begin
            result <= operand_1;
        end
        24'b1111_0000_0000_0001_????_????: begin
            result <= operand_1;
        end
        24'b1111_0000_0000_0010_????_????: begin
            result <= operand_1;
        end
    
        24'b1010_????_????_????_????_????: begin
            result <= operand_1 + operand_2;
        end
        24'b1111_0000_0010_0000_????_????: begin
            result <= operand_1 + operand_2;
        end
      	24'b0100_????_????_????_????_????: begin
          	result <= operand_2 == 0;
        end
        24'b0101_????_????_????_????_????: begin
          	result <= operand_2 != 0;
        end
      	24'b1110_0100_????_????_????_????: begin
          	result <= ip;
        end
      	24'b1111_1111_1111_1111_1111_0000: begin
          	result <= operand_2 - 1;
        end
        24'b1011_????_????_????_????_????: begin 
            // SUB IMM
            result = operand_2 - operand_1;
        end
        24'b1100_????_????_????_????_????: begin 
            // AND IMM
            result = operand_1 & operand_2;
        end
        24'b1101_????_????_????_????_????: begin 
            // OR IMM
            result = operand_1 | operand_2;
        end
        24'b1111_0000_0011_0000_????_????: begin 
            // SHL IMM
            result = operand_2 << operand_1;
        end
        24'b1111_0000_0011_0010_????_????: begin 
            // SHR IMM
            result = operand_2 >> operand_1;
        end
        24'b1111_0000_0010_0001_????_????: begin
            // SUB
            result = operand_2 - operand_1;
        end
        24'b1111_0000_0010_0010_????_????: begin
            // AND
            result = operand_2 & operand_1;
        end
        24'b1111_0000_0010_0011_????_????: begin
            // OR
            result = operand_2 | operand_1;
        end
        24'b1111_0000_0010_0100_????_????: begin
            // NOT
            result = ~operand_1;
        end
        24'b1111_0000_0011_0001_????_????: begin 
            // SHL REG
            result = operand_2 << operand_1;
        end
        24'b1111_0000_0011_0011_????_????: begin 
            // SHR REG
            result = operand_2 >> operand_1;
        end
    endcase
  end
  
  always @ (posedge stage_5_active) begin
    // stage 5: calculate whether we need to store to memory
    // and calculate memory address
    casez (curr_instr) 
        24'b0011_????_????_????_????_????: begin
            mem_write_addr <= instr_imm;
            write_to_mem <= 1;
        end
        24'b1111_0000_0000_0010_????_????: begin
            mem_write_addr <= registers[reg_d_id];
            write_to_mem <= 1;
        end
      	24'b1110_0100_????_????_????_????: begin
          mem_write_addr <= registers[0];
          write_to_mem <= 1;
        end
        default: begin
            write_to_mem <= 0;
        end
    endcase
  end
  
  always @ (posedge stage_6_active) begin
    // write result
    if (write_to_mem) begin
        program_mem[mem_write_addr] = result;
        if (curr_instr[23:16] == 8'b1110_0100) begin
            // CALL instruction
            registers[0] <= registers[0] + 1;
        end
    end else begin
        casez (curr_instr) 
            24'b0001_????_????_????_????_????: begin
                registers[reg_a_id] <= result;
            end
            24'b0010_????_????_????_????_????: begin
                registers[reg_a_id] <= result;
            end
            24'b1111_0000_0000_0000_????_????: begin
                registers[reg_d_id] <= result;
            end
            24'b1111_0000_0000_0001_????_????: begin
                registers[reg_d_id] <= result;
            end
            24'b1010_????_????_????_????_????: begin
                registers[reg_a_id] <= result;
            end
            24'b1111_0000_0010_0000_????_????: begin
                registers[reg_d_id] <= result;
            end
          	24'b1111_1111_1111_1111_1111_0000: begin
              	registers[0] <= result;
            end


            24'b1011_????_????_????_????_????: begin 
                // SUB IMM
                registers[reg_a_id] <= result;
            end
            24'b1100_????_????_????_????_????: begin 
                // AND IMM
                registers[reg_a_id] <= result;
            end
            24'b1101_????_????_????_????_????: begin 
                // OR IMM
                registers[reg_a_id] <= result;
            end
            24'b1111_0000_0011_0000_????_????: begin 
                // SHL IMM
                registers[reg_d_id] <= result;
            end
            24'b1111_0000_0011_0010_????_????: begin 
                // SHR IMM
                registers[reg_d_id] <= result;
            end
            24'b1111_0000_0010_0001_????_????: begin
                // SUB
                registers[reg_d_id] <= result;
            end
            24'b1111_0000_0010_0010_????_????: begin
                // AND
                registers[reg_d_id] <= result;
            end
            24'b1111_0000_0010_0011_????_????: begin
                // OR
                registers[reg_d_id] <= result;
            end
            24'b1111_0000_0010_0100_????_????: begin
                // NOT
                registers[reg_d_id] <= result;
            end

            24'b1111_0000_0011_0001_????_????: begin 
                // SHL REG
                registers[reg_d_id] <= result;
            end
            24'b1111_0000_0011_0011_????_????: begin 
                // SHR REG
                registers[reg_d_id] <= result;
            end

        endcase
    end
  end
  
  
  always @ (posedge stage_7_active) begin
    // increment instruction pointer (or jump)
    casez (curr_instr)
        24'b1110_0011_????_????_????_????: begin
            ip <= operand_1;
        end
      	24'b1110_0100_????_????_????_????: begin
          	ip <= operand_1; 
        end
      	24'b1111_1111_1111_1111_1111_0000: begin
            ip <= operand_1 + 1;
        end 
      	24'b0100_????_????_????_????_????: begin
          if (result[0]) begin
            ip <= operand_1;
          end else begin
            ip <= ip + 1;
          end
        end
      	24'b0101_????_????_????_????_????: begin
          if (result[0]) begin
            ip <= operand_1;
          end else begin
            ip <= ip + 1;
          end
        end
        default: begin
            ip <= ip + 1;
        end
    endcase
    
  end
  
  
  vga vga_(
    .vgaRed (vgaRed),
    .vgaBlue (vgaBlue),
    .vgaGreen (vgaGreen),
    .Hsync (Hsync),
    .Vsync (Vsync),
    
    .bitmap (vga_bitmap),
    .clk (clk)
  );



endmodule