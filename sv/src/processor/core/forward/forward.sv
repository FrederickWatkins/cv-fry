import pipeline::XLEN;

// Operand forwarding unit
module forward (
    input logic [4:0] rs1_addr,
    input logic [XLEN-1:0] rs1_data_rf,
    input logic [4:0] rs2_addr,
    input logic [XLEN-1:0] rs2_data_rf,
    input logic [4:0] rd_addr_M,
    input logic [XLEN-1:0] rd_data_M,
    input logic [4:0] rd_addr_W,
    input logic [XLEN-1:0] rd_data_W,

    output logic [XLEN-1:0] rs1_data,
    output logic [XLEN-1:0] rs2_data
);
    always_comb begin
        if(rs1_addr == 0) rs1_data = rs1_data_rf;
        else if(rs1_addr == rd_addr_M) rs1_data = rd_data_M;
        else if(rs1_addr == rd_addr_W) rs1_data = rd_data_W;
        else rs1_data = rs1_data_rf;
        if(rs2_addr == 0) rs2_data = rs2_data_rf;
        else if(rs2_addr == rd_addr_M) rs2_data = rd_data_M;
        else if(rs2_addr == rd_addr_W) rs2_data = rd_data_W;
        else rs2_data = rs2_data_rf;
    end
endmodule
