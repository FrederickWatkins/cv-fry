module core_shim #(
    parameter XLEN = 32
) (
    input logic clk,
    input logic reset_n,

    // Instruction Bus (c2c_r master)
    input  logic             instr_ack,
    input  logic [31:0]      instr_data,
    output logic             instr_re,
    output logic [XLEN/8-1:0] instr_sel,
    output logic [XLEN-1:0]  instr_addr,

    // Data Bus Read (c2c_r master)
    input  logic             dr_ack,
    input  logic [XLEN-1:0]  dr_data,
    output logic             dr_re,
    output logic [XLEN/8-1:0] dr_sel,
    output logic [XLEN-1:0]  dr_addr,

    // Data Bus Write (c2c_w master)
    input  logic             dw_ack,
    output logic             dw_we,
    output logic [XLEN/8-1:0] dw_sel,
    output logic [XLEN-1:0]  dw_addr,
    output logic [XLEN-1:0]  dw_data
);

    // Internal Interface Instances
    c2c_r #(.XLEN(XLEN)) instr_bus_if ();
    c2c_r #(.XLEN(XLEN)) data_bus_r_if ();
    c2c_w #(.XLEN(XLEN)) data_bus_w_if ();

    // Instruction Bus Mapping
    assign instr_bus_if.ack = instr_ack;
    assign instr_bus_if.data = instr_data;
    assign instr_re   = instr_bus_if.re;
    assign instr_sel  = instr_bus_if.sel;
    assign instr_addr = instr_bus_if.addr;

    // Data Read Bus Mapping
    assign data_bus_r_if.ack = dr_ack;
    assign data_bus_r_if.data = dr_data;
    assign dr_re      = data_bus_r_if.re;
    assign dr_sel     = data_bus_r_if.sel;
    assign dr_addr    = data_bus_r_if.addr;

    // Data Write Bus Mapping
    assign data_bus_w_if.ack = dw_ack;
    assign dw_we      = data_bus_w_if.we;
    assign dw_sel     = data_bus_w_if.sel;
    assign dw_addr    = data_bus_w_if.addr;
    assign dw_data    = data_bus_w_if.data;

    // DUT Instantiation
    core #(
        .XLEN(XLEN)
    ) core_inst (
        .clk,
        .reset_n,
        .instr_bus(instr_bus_if.master),
        .data_bus_r(data_bus_r_if.master),
        .data_bus_w(data_bus_w_if.master)
    );

endmodule
