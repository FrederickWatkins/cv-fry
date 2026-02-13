import pipeline::XLEN;
import pipeline::memory_signals;
import pipeline::writeback_signals;

module lsu_shim (
    // Data Bus Read Interface (c2c_r)
    input  logic             dr_ack,
    input  logic [XLEN-1:0]  dr_data,
    output logic             dr_re,
    output logic [XLEN/8-1:0] dr_sel,
    output logic [XLEN-1:0]  dr_addr,

    // Data Bus Write Interface (c2c_w)
    input  logic             dw_ack,
    output logic             dw_we,
    output logic [XLEN/8-1:0] dw_sel,
    output logic [XLEN-1:0]  dw_addr,
    output logic [XLEN-1:0]  dw_data,

    // Control/Data Inputs
    input logic mm_we,
    input logic mm_re,
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
    c2c_r data_bus_r ();
    c2c_w data_bus_w ();

    // Map Read Interface
    assign data_bus_r.ack  = dr_ack;
    assign data_bus_r.data = dr_data;
    assign dr_re           = data_bus_r.re;
    assign dr_sel          = data_bus_r.sel;
    assign dr_addr         = data_bus_r.addr;

    // Map Write Interface
    assign data_bus_w.ack  = dw_ack;
    assign dw_we           = data_bus_w.we;
    assign dw_sel          = data_bus_w.sel;
    assign dw_addr         = data_bus_w.addr;
    assign dw_data         = data_bus_w.data;

    memory_signals signals_in;
    writeback_signals signals_out;

    assign signals_in.funct3 = funct3;
    assign signals_in.mm_re = mm_re;
    assign signals_in.mm_we = mm_we;
    assign signals_in.mm_addr = mm_addr;
    assign signals_in.data = data_in;
    assign signals_in.rd_addr = rd_addr_in;

    assign data_out = signals_out.data;
    assign rd_addr_out = signals_out.rd_addr;

    lsu lsu_inst (
        .data_bus_r(data_bus_r.master),
        .data_bus_w(data_bus_w.master),

        .signals_in(signals_in),

        .busy(busy),
        .signals_out(signals_out)
    );

endmodule
