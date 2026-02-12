import pipeline::*;
module ieu (
    input logic [31:0] rs1_data,
    input logic [31:0] rs2_data,
    input execute_signals signals_in,

    output logic je,
    output logic [31:0] ja,
    output memory_signals signals_out
);
endmodule