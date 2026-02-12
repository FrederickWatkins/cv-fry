import pipeline::*;
module rf (
    input logic clk,
    input logic [4:0] rs1_addr,
    input logic [4:0] rs2_addr,
    input writeback_signals signals_in,
    
    output logic [31:0] rs1_data,
    output logic [31:0] rs2_data
);
endmodule
