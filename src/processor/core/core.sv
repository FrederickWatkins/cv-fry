// Processor core
module core #(
    parameter XLEN = 32
) (
    input logic clk,
    input logic reset_n,

    c2c_r.master instr_bus,

    c2c_r.master data_bus_r,
    c2c_w.master data_bus_w
);
    logic stall_D, stall_M; // Stall signals decode and main memory
    // Decode stage signals
    logic jb_D;
    logic [4:0] rs1_addr_D, rs2_addr_D;
    logic [31:2] instr_D;
    logic [XLEN-1:0] curr_pc_D, inc_pc_D;
    // Execute stage signals
    logic jump_E, branch_E, op1_pc_E, op2_imm_E;
    logic [2:0] alu_funct3_E, funct3_E;
    logic [6:0] funct7_E;
    logic [XLEN-1:0] rs1_data_E, rs2_data_E, imm_E, curr_pc_E, inc_pc_E;
    // Memory stage signals
    logic mm_re_M, mm_we_M;
    logic [2:0] funct3_M;
    logic [XLEN-1:0] rs2_data_M, ieu_result_M, inc_pc_M;
    // Writeback stage signals (currently in same stage as memory)
    logic wb_ieu_W, wb_lsu_W, wb_pc_W;
    logic [4:0] rd_addr_W;
    logic [XLEN-1:0] lsu_data_W, ieu_result_W, inc_pc_W;
    // Return signals from ieu to ifu
    logic jack_E, je_E, ja_E;

    always_ff @(posedge clk or negedge reset_n) begin
        if(!(stall_D | stall_M)) begin
            curr_pc_E <= curr_pc_D;
            inc_pc_E <= inc_pc_D;
        end
        if(!stall_M) begin
            inc_pc_M <= inc_pc_E;
            inc_pc_W <= inc_pc_M;
            rs2_data_M <= rs2_data_E;
            ieu_result_W <= ieu_result_M;
        end
        if(!reset_n) begin
            curr_pc_E <= 0;
            inc_pc_E <= 0;
            inc_pc_M <= 0;
            inc_pc_W <= 0;
            rs2_data_M <= 0;
            ieu_result_W <= 0;
        end
    end

    ifu #(
        .XLEN(XLEN)
    ) ifu (
        .clk,
        .reset_n,

        .instr_bus,

        .stall(stall_D | stall_M),
        .jump(jb_D),
        .jack(jack_E),
        .je(je_E),
        .ja(ja_E),

        .instr_out(instr_D),
        .curr_pc(curr_pc_D),
        .inc_pc(inc_pc_D)
    );

    decoder #(
        .XLEN(XLEN)
    ) decoder (
        .clk,
        .reset_n,
        // Inputs
        .instr(instr_D),
        .stall_M,
        // Decode stage outputs (no reg)
        .stall_D,
        .jb_D,
        .rs1_addr_D, .rs2_addr_D,
        // Execute stage outputs (one reg)
        .jump_E, .branch_E, .op1_pc_E, .op2_imm_E,
        .alu_funct3_E, .funct3_E, .funct7_E,
        .imm_E,
        // Memory stage outputs (two reg)
        .mm_re_M, .mm_we_M,
        .funct3_M,
        // Writeback stage outputs (three reg)
        .wb_ieu_W, .wb_lsu_W, .wb_pc_W,
        .rd_addr_W
    );

    rf #(
        .XLEN(XLEN)
    ) rf (
        .clk,
        // No reset_n, registers are not reset, must be initialized by software

        .rs1_addr(rs1_addr_D),
        .rs2_addr(rs2_addr_D),
        .rd_addr(rd_addr_W),
        .lsu_data(lsu_data_W),
        .ieu_result(ieu_result_W),
        .inc_pc(inc_pc_W)

        .rs1_data(rs1_data_E),
        .rs2_data(rs2_data_E)
    );

    ieu #(
        .XLEN(XLEN)
    ) ieu (
        .clk,
        .reset_n,
        // Inputs
        .jump(jump_E), .branch(branch_E), .op1_pc(op1_pc_E), .op2_imm(op2_imm_E),
        .alu_funct3(alu_funct3_E), .funct3(funct3_E),
        .funct7(funct7_E),
        .rs1_data(rs1_data_E), .rs2_data(rs2_data_E), .imm(imm_E), .curr_pc(curr_pc_E),
        // Comb outputs
        .jack(jack_E),
        .je(je_E),
        .ja(ja_E),
        // Sync outputs
        .result(ieu_result_M)
    );

    lsu #(
        .XLEN(XLEN)
    ) lsu (
        .clk,
        .reset_n,
        .data_bus_r,
        .data_bus_w,
        // Inputs
        .mm_re(mm_re_M), .mm_we(mm_we_M),
        .funct3(funct3_M),
        .ieu_result(ieu_result_M), .rs2_data(rs2_data_M),
        // Comb outputs
        .stall(stall_M),
        // Sync outputs
        .data(lsu_data_W)
    );
endmodule
