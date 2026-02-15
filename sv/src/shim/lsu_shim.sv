import pipeline::XLEN;
import pipeline::memory_signals;
import pipeline::writeback_signals;

module lsu_shim (
    // Data bus
    input  logic             data_ack,
    input  logic [XLEN-1:0]  data_r,
    output logic             data_re,
    output logic             data_we,
    output logic atomic_out,
    output logic [4:0] amo_op,
    output logic [XLEN/8-1:0] data_sel,
    output logic [XLEN-1:0]  data_addr,
    output logic [XLEN-1:0]  data_w,

    // Control/Data Inputs
    input logic mm_we,
    input logic mm_re,
    input logic atomic_in,
    input logic [4:0] funct5,
    input logic [2:0] funct3,
    input logic [XLEN-1:0] mm_addr,
    input logic [XLEN-1:0] data_in,
    input logic [4:0] rd_addr_in,

    // Outputs
    output logic busy,
    output logic [XLEN-1:0] data_out,
    output logic [4:0] rd_addr_out
);

    // Instantiate Interfaces
    c2c_data data_bus ();

    // Map Read Interface
    assign data_bus.ack  = data_ack;
    assign data_bus.data_r = data_r;
    assign data_re = data_bus.re;
    assign data_we = data_bus.we;
    assign atomic_out = data_bus.atomic;
    assign amo_op = data_bus.amo_op;
    assign data_sel = data_bus.sel;
    assign data_addr = data_bus.addr;
    assign data_w = data_bus.data_w;

    memory_signals signals_in;
    writeback_signals signals_out;

    assign signals_in.funct3 = funct3;
    assign signals_in.mm_re = mm_re;
    assign signals_in.mm_we = mm_we;
    assign signals_in.atomic = atomic_in;
    assign signals_in.funct5 = funct5;
    assign signals_in.mm_addr = mm_addr;
    assign signals_in.data = data_in;
    assign signals_in.rd_addr = rd_addr_in;

    assign data_out = signals_out.data;
    assign rd_addr_out = signals_out.rd_addr;

    lsu lsu_inst (
        .data_bus(data_bus.master),

        .signals_in(signals_in),

        .busy(busy),
        .signals_out(signals_out)
    );

endmodule
