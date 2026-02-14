import pipeline::XLEN;
import pipeline::decode_signals;

module ifu_shim (
    input logic clk,
    input logic reset_n,

    input logic ack,
    input logic [31:0] instr,

    output logic re,
    output logic [XLEN/8-1:0] sel,
    output logic [XLEN-1:0] addr,

    input logic stall,
    input logic je,
    input logic [XLEN-1:0] ja,

    output logic [31:2] instr_out,
    output logic [XLEN-1:0] curr_pc,
    output logic [XLEN-1:0] inc_pc
);

    c2c_instr instr_bus ();
    decode_signals signals_out_F;

    assign instr_bus.ack = ack;
    assign instr_bus.instr = instr;

    always_ff @(posedge clk) begin
        re <= instr_bus.re;
        sel <= instr_bus.sel;
        addr <= instr_bus.addr;
    end

    assign instr_out = signals_out_F.instr;
    assign curr_pc = signals_out_F.curr_pc;
    assign inc_pc = signals_out_F.inc_pc;

    ifu ifu (
        .clk,
        .reset_n,

        .instr_bus(instr_bus),

        .stall,
        .je,
        .ja,

        .signals_out(signals_out_F)
    );
endmodule
