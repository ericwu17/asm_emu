`timescale 1ns / 1ps
//////////////////////////////////////////////////////////////////////////////////
// Company: 
// Engineer: 
// 
// Create Date: 11/09/2023 04:31:42 PM
// Design Name: 
// Module Name: tb
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


module tb(

    );
    reg clk = 0;
    always #5 clk = ~clk;
    
    reg btnU = 0;
    reg btnL = 0;
    reg btnR = 0;
    reg btnD = 0;
    reg btnC = 0;
    
    wire led;
    
    initial begin
        #100000
        btnL = 1;
        #100000
        btnL = 0;
        #100000
        btnL = 1;
        #100000
        btnL = 0;
        #100000
        btnL = 1;
        #100000
        btnL = 0;
        
        #10000;
    
    end
    
    cpu_unit cpu_unit_ (
        .btnU (btnU),
        .btnD (btnD),
        .btnL (btnL),
        .btnR (btnR),
        .btnC (btnC),
        
        .led(led),
        .clk (clk)
    );
    
    
endmodule
