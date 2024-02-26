`timescale 1ns / 1ps
//////////////////////////////////////////////////////////////////////////////////
// Company: 
// Engineer: 
// 
// Create Date: 11/06/2023 05:22:42 PM
// Design Name: 
// Module Name: vga
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

  
module vga(
    output [3:0] vgaRed,
    output [3:0] vgaBlue,
    output [3:0] vgaGreen,
    output Vsync,
    output Hsync,
    input [19199:0] bitmap,
    input clk    // 100Mhz
    );
    
    parameter scan_line_width = 800;
    parameter frame_height = 525; // each frame is 525 lines high
    
    parameter h_sync_begin = 655;
    parameter h_sync_end = 751;
    parameter v_sync_begin = 489;
    parameter v_sync_end = 491;
    
    parameter h_display_end = 639;
    parameter v_display_end = 479;

    reg [3:0] vgaRedReg;
    reg [3:0] vgaGreenReg;
    reg [3:0] vgaBlueReg;
    reg HsyncReg;
    reg VsyncReg;
    reg in_h_display;
    reg in_v_display;
    wire in_display;



    reg [9:0] h_counter;
    reg [9:0] v_counter;
    
    reg [1:0] vga_clk;
    
     
    wire [9:0] x_position;
    wire [9:0] y_position;
    
    assign x_position = (h_counter+1) / 4;
    assign y_position = (v_counter+1) / 4;


    
    
    assign vgaRed = vgaRedReg;
    assign vgaBlue = vgaBlueReg;
    assign vgaGreen = vgaGreenReg;
    assign Hsync = HsyncReg;
    assign Vsync = VsyncReg;
    assign in_display = in_v_display & in_h_display;
    
    initial begin
        h_counter = 0;
        v_counter = 0;
        in_h_display = 1;
        in_v_display = 1;
        vgaRedReg = 0;
        vgaBlueReg = 0;
        vgaGreenReg = 0;
        HsyncReg = 1;
        VsyncReg = 1;
        vga_clk = 0;
    end
    

    always @ (posedge clk) begin
        vga_clk = vga_clk + 1;
    end
    
    always @ (posedge vga_clk[1]) begin
        
           if (in_display) begin
                if (bitmap[y_position*160 + x_position]) begin
                    vgaRedReg[3:0] <= 4'b1111;
                    vgaBlueReg[3:0] <= 4'b1111;
                    vgaGreenReg[3:0] <= 4'b1111;
                end else begin
                    vgaRedReg[3:0] <= 4'b0000;
                    vgaBlueReg[3:0] <= 4'b0000;
                    vgaGreenReg[3:0] <= 4'b0000;
                end
           end else begin
                vgaRedReg[3:0] <= 0;
                vgaBlueReg[3:0] <= 0;
                vgaGreenReg[3:0] <= 0;
           end
        
        
            if (h_counter == scan_line_width - 1 ) begin
                h_counter <= 0;
                in_h_display <= 1;
                if (v_counter == frame_height - 1) begin
                    in_v_display <= 1; 
                    v_counter <= 0;
                end else begin
                    v_counter <= v_counter + 1;
                end
            end else begin
                h_counter <= h_counter + 1;
            end
            
            
            if (h_counter == h_sync_begin)
                HsyncReg <= 0;
            if (h_counter == h_sync_end)
                HsyncReg <= 1;
            if (v_counter == v_sync_begin)
                VsyncReg <= 0;
            if (v_counter == v_sync_end)
                VsyncReg <= 1;
            if (h_counter == h_display_end)
                in_h_display <= 0;
            if (v_counter == v_display_end)
                in_v_display <= 0;
        
    end
endmodule
