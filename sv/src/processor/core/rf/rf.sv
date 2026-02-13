module rf (
    input logic clk,
    input logic [4:0] rs1_addr,
    input logic [4:0] rs2_addr,
    input writeback_signals signals_in,
    
    output logic [31:0] rs1_data,
    output logic [31:0] rs2_data
);
    import pipeline::*;
    logic [XLEN-1:0] ram [31:1];

    always_comb begin
        if(rs1_addr != 0) rs1_data = ram[rs1_addr];
        else rs1_data = 0;
        if(rs2_addr != 0) rs2_data = ram[rs2_addr];
        else rs2_data = 0;
    end

    always_ff @(posedge clk) begin
        if(signals_in.rd_addr != 0) ram[signals_in.rd_addr] <= signals_in.data;
    end
endmodule
