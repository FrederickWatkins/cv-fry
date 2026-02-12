import pipeline::*;
// Load store unit
module lsu (
    input logic clk,
    input logic reset_n,

    c2c_r.master data_bus_r,
    c2c_w.master data_bus_w,

    input memory_signals signals_in,

    output writeback_signals signals_out
);
endmodule
