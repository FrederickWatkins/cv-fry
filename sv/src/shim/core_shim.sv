import pipeline::XLEN;

module core_shim (
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
    c2c_r #(.XLEN(XLEN)) instr_bus ();
    c2c_r #(.XLEN(XLEN)) data_bus_r ();
    c2c_w #(.XLEN(XLEN)) data_bus_w ();

    // Instruction Bus Mapping
    assign instr_bus.ack = instr_ack;
    assign instr_bus.data = instr_data;

    // Data Read Bus Mapping
    assign data_bus_r.ack = dr_ack;
    assign data_bus_r.data = dr_data;

    // Data Write Bus Mapping
    assign data_bus_w.ack = dw_ack;

    always_ff @(posedge clk) begin
        instr_re <= instr_bus.re;
        instr_sel <= instr_bus.sel;
        instr_addr <= instr_bus.addr;
        dr_re <= data_bus_r.re;
        dr_sel <= data_bus_r.sel;
        dr_addr <= data_bus_r.addr;
        dw_we <= data_bus_w.we;
        dw_sel <= data_bus_w.sel;
        dw_addr <= data_bus_w.addr;
        dw_data <= data_bus_w.data;
    end

    // DUT Instantiation
    core core (
        .clk,
        .reset_n,
        .instr_bus(instr_bus.master),
        .data_bus_r(data_bus_r.master),
        .data_bus_w(data_bus_w.master)
    );

endmodule
