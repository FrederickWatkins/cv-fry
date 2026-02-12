import pipeline::*;
// Instruction fetch unit
module ifu (
    input logic clk,
    input logic reset_n,
    input logic stall,
    input logic flush,
    c2c_r.master instr_bus, // The plugin needs to see this port exists!
    input logic je,
    input logic [31:0] ja,
    output decode_signals signals_out
);
    // No logic here
endmodule
