import pipeline::XLEN;
module core_shim (
    input logic clk,
    input logic reset_n,

    // Instruction Bus (c2c_instr master)
    input  logic             instr_ack,
    input  logic [31:0]      instr_data,
    output logic             instr_re,
    output logic [XLEN/8-1:0] instr_sel,
    output logic [XLEN-1:0]  instr_addr,

    // Data Bus (c2c_data master)
    input  logic             data_ack,
    input  logic [XLEN-1:0]  data_r,
    output logic             data_re,
    output logic             data_we,
    output logic [XLEN/8-1:0] data_sel,
    output logic [XLEN-1:0]  data_addr,
    output logic [XLEN-1:0]  data_w
);

    // Internal Interface Instances
    c2c_instr instr_bus ();
    c2c_data data_bus ();

    // Instruction Bus Mapping
    assign instr_bus.ack = instr_ack;
    assign instr_bus.instr = instr_data;

    // Data Read Bus Mapping
    assign data_bus.ack = data_ack;
    assign data_bus.data_r = data_r;

    always_ff @(posedge clk) begin
        instr_re <= instr_bus.re;
        instr_sel <= instr_bus.sel;
        instr_addr <= instr_bus.addr;
        data_re <= data_bus.re;
        data_we <= data_bus.we;
        data_sel <= data_bus.sel;
        data_addr <= data_bus.addr;
        data_w <= data_bus.data_w;
    end

    // DUT Instantiation
    core core (
        .clk,
        .reset_n,
        .instr_bus(instr_bus.master),
        .data_bus(data_bus.master)
    );

endmodule
