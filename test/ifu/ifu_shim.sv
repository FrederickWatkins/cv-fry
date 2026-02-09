module ifu_shim #(
    parameter XLEN = 32
) (
    input logic clk,
    input logic reset_n,

    input logic ack,
    input logic [31:0] instr,

    output logic re,
    output logic [XLEN/8-1:0] sel,
    output logic [XLEN-1:0] addr,

    input logic stall,
    input logic jump,
    input logic jack,
    input logic je,
    input logic [XLEN-1:0] ja,

    output logic [31:2] instr_out,
    output logic [XLEN-1:0] curr_pc,
    output logic [XLEN-1:0] inc_pc
);

    c2c_r #(.XLEN(XLEN)) instr_bus ();

    assign instr_bus.ack = ack;
    assign instr_bus.data = instr;

    always_ff @(posedge clk) begin
        re <= instr_bus.re;
        sel <= instr_bus.sel;
        addr <= instr_bus.addr;
    end

    ifu #(
        .XLEN(XLEN)
    ) ifu (
        .clk,
        .reset_n,
        
        .instr_bus,
        .stall,
        .jump,
        .jack,
        .je,
        .ja,

        .instr_out,
        .curr_pc,
        .inc_pc
    );
endmodule
