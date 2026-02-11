module lsu_shim #(
    parameter XLEN = 32
) (
    input logic clk,
    input logic reset_n,

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
    input logic [XLEN-1:0] ieu_result,
    input logic [XLEN-1:0] rs2_data,

    // Outputs
    output logic stall,
    output logic [XLEN-1:0] data_out
);

    // Instantiate Interfaces
    c2c_r #(.XLEN(XLEN)) data_bus_r ();
    c2c_w #(.XLEN(XLEN)) data_bus_w ();

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

    lsu #(
        .XLEN(XLEN)
    ) lsu_inst (
        .clk,
        .reset_n,
        .data_bus_r(data_bus_r.master),
        .data_bus_w(data_bus_w.master),
        .mm_we,
        .mm_re,
        .funct3,
        .ieu_result,
        .rs2_data,
        .stall,
        .data(data_out)
    );

endmodule
