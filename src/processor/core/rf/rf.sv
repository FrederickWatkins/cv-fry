module rf #(
    parameter XLEN = 32
) (
    input  logic clk,
    // No reset_n, registers are not reset, must be initialized by software

    input logic [4:0] rs1_addr,
    input logic [4:0] rs2_addr,
    input logic [4:0] rd_addr,
    input logic [XLEN-1:0] ieu_result,
    input logic [XLEN-1:0] lsu_data,
    input logic [XLEN-1:0] inc_pc,
    input logic stall,
    input logic wb_ieu,
    input logic wb_lsu,
    input logic wb_pc,

    output logic [XLEN-1:0] rs1_data,
    output logic [XLEN-1:0] rs2_data
);

    logic [XLEN-1:0] ram [31:1];
    logic rd_we;
    logic [XLEN-1:0] rd_data;

    always_comb begin
        rd_we = 0;
        rd_data = 'x;
        if(wb_ieu) begin
            rd_we = 1;
            rd_data = ieu_result;
        end
        if(wb_lsu) begin
            rd_we = 1;
            rd_data = lsu_data;
        end
        if(wb_pc) begin
            rd_we = 1;
            rd_data = inc_pc;
        end
    end

    always_ff @(posedge clk) begin
        if(!stall) begin
            if(rs1_addr != 0) rs1_data <= ram[rs1_addr];
            else rs1_data <= 0;
            if(rs2_addr != 0) rs2_data <= ram[rs2_addr];
            else rs2_data <= 0;
        end
        if(rd_addr != 0 & rd_we) ram[rd_addr] <= rd_data;
    end

endmodule
