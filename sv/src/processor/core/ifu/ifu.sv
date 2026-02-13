import pipeline::*;
// Instruction fetch unit
module ifu (
    input logic clk,
    input logic reset_n,

    c2c_r.master instr_bus,

    input logic stall,
    input logic je,
    input logic [31:0] ja,

    output decode_signals signals_out
);
    logic stall_pc, compressed;
    logic [31:2] decomp_instr;

    assign instr_bus.sel = 4'b1111;
    assign instr_bus.re = 1;

    always_comb begin
        stall_pc = stall;
        if(!instr_bus.ack) begin
            signals_out.instr = 'h4;
            stall_pc = 1;
        end
        else begin
            signals_out.instr = decomp_instr;
        end
    end
    
    pc pc (
        .clk,
        .reset_n,

        .stall(stall_pc),
        .compressed,
        .je,
        .ja,

        .curr_pc(signals_out.curr_pc),
        .inc_pc(signals_out.inc_pc),
        .next_pc(instr_bus.addr)
    )

    decomp decomp (
        .instr_in(instr_bus.data),

        .compressed,
        .instr_out(decomp_instr)
    )
endmodule
