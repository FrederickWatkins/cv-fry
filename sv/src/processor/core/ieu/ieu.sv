import pipeline::XLEN;
import pipeline::execute_signals;
import pipeline::memory_signals;

module ieu (
    input logic [XLEN-1:0] rs1_data,
    input logic [XLEN-1:0] rs2_data,
    input execute_signals signals_in,

    output logic je,
    output logic [XLEN-1:0] ja,
    output logic [4:0] rs1_addr,
    output logic [4:0] rs2_addr,
    output memory_signals signals_out
);
    import pipeline::*;

    logic [XLEN-1:0] operand_1, operand_2, alu_result;

    assign rs1_addr = signals_in.rs1_addr;
    assign rs2_addr = signals_in.rs2_addr;

    assign operand_1 = signals_in.op1_pc?signals_in.curr_pc:rs1_data;
    assign operand_2 = signals_in.op2_imm?signals_in.imm:rs2_data;

    assign signals_out.funct3 = signals_in.funct3;
    assign signals_out.mm_re = signals_in.mm_re;
    assign signals_out.mm_we = signals_in.mm_we;
    assign signals_out.mm_addr = alu_result;
    assign signals_out.data = signals_in.jump?signals_in.inc_pc:alu_result;
    assign signals_out.rd_addr = signals_in.rd_addr;
    assign ja = alu_result;

    alu alu (
        .funct3(signals_in.alu_funct3),
        .funct7(signals_in.funct7),

        .operand_1,
        .operand_2,

        .result(alu_result)
    );

    jbu jbu (
        .jump(signals_in.jump),
        .branch(signals_in.branch),
        .funct3(signals_in.funct3),

        .rs1_data,
        .rs2_data,

        .je
    );
endmodule
