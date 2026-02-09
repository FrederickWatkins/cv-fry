module ieu #(
    parameter XLEN = 32
) (
    input logic clk,
    input logic reset_n,
    
    // Inputs
    input logic jump,
    input logic branch,
    input logic op1_pc,
    input logic op2_imm,
    input logic [2:0] alu_funct3,
    input logic [2:0] funct3,
    input logic [6:0] funct7,
    input logic [XLEN-1:0] rs1_data,
    input logic [XLEN-1:0] rs2_data,
    input logic [XLEN-1:0] imm,
    input logic [XLEN-1:0] curr_pc,

    // Comb outputs
    output logic jack,
    output logic je,
    output logic [XLEN-1:0] ja,

    // Sync outputs
    output logic [XLEN-1:0]   result
);

    logic [XLEN-1:0] alu_result_comb;

    assign ja = alu_result_comb;

    always_ff @(posedge clk or negedge reset_n) begin
        if (!reset_n) begin
            result <= '0;
        end else begin
            result <= alu_result_comb;
        end
    end

    alu #(
        .XLEN(XLEN)
    ) alu_inst (
        .funct3(alu_funct3),
        .funct7,
        .operand_1(op1_pc?curr_pc:rs1_data),
        .operand_2(op2_imm?imm:rs2_data),
        .result(alu_result_comb)
    );


    jbu #(
        .XLEN(XLEN)
    ) jbu_inst (
        .jump,
        .branch,
        .funct3,
        .rs1_data,
        .rs2_data,
        .jack,
        .je
    );
endmodule
