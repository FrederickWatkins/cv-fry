// Decoder
module decoder #(
    parameter XLEN = 32
) (
    input logic clk,
    input logic reset_n,
    // Inputs
    input logic [31:2] instr,
    input logic stall_M,
    // Decode stage outputs (no reg)
    output logic stall_D,
    output logic jb_D,
    output logic [4:0] rs1_addr_D, output logic [4:0] rs2_addr_D,
    // Execute stage outputs (one reg)
    output logic jump_E, output logic branch_E, output logic op1_pc_E, output logic op2_imm_E,
    output logic [2:0] alu_funct3_E, output logic [2:0] funct3_E, output logic [6:0] funct7_E,
    output logic [XLEN-1:0] imm_E,
    // Memory stage outputs (two reg)
    output logic mm_re_M, output logic mm_we_M,
    output logic [2:0] funct3_M,
    // Writeback stage outputs (three reg)
    output logic wb_ieu_W, output logic wb_lsu_W, output logic wb_pc_W,
    output logic [4:0] rd_addr_W
);
    typedef enum logic [4:0] {
        LOAD = 'b00000,
        STORE = 'b01000,
        BRANCH = 'b11000,
        JALR = 'b11001,
        MISC_MEM = 'b00011,
        JAL = 'b11011,
        OP_IMM = 'b00100,
        OP = 'b01100,
        SYSTEM = 'b11100,
        AUIPC = 'b00101,
        LUI = 'b01101
    } opcodes;

    logic [4:0] opcode;

    logic jump_D, branch_D, op1_pc_D, op2_imm_D, mm_re_D, mm_we_D, wb_ieu_D, wb_lsu_D, wb_pc_D;
    logic [2:0] alu_funct3_D, funct3_D;
    logic [6:0] funct7_D;
    logic [XLEN-1:0] imm_D;
    logic [4:0] rd_addr_D;

    logic mm_re_E, mm_we_E, wb_ieu_E, wb_lsu_E, wb_pc_E;
    logic [4:0] rd_addr_E;

    logic wb_ieu_M, wb_lsu_M, wb_pc_M;
    logic [4:0] rd_addr_M;

    assign opcode = instr[6:2]

    always_comb begin
        jb_D = 0;
        rs1_addr_D = instr[19:15];
        rs2_addr_D = instr[24:20];
        jump_D = 0;
        branch_D = 0;
        op1_pc_D = 0;
        op2_imm_D = 0;
        mm_re_D = 0;
        mm_we_D = 0;
        wb_ieu_D = 0;
        wb_lsu_D = 0;
        wb_pc_D = 0;
        alu_funct3_D = instr[14:12];
        funct3_D = instr[14:12];
        funct7_D = instr[31:25];
        imm_D = 0;
        rd_addr_D = instr[11:7];
        case(opcode)
        LOAD: begin
            rs2_addr_D = 0;
            op2_imm_D = 1;
            mm_re_D = 1;
            wb_lsu_D = 1;
            alu_funct3_D = 'b000; // Use alu to add addresses
            imm_D = {{(XLEN-11){instr[31]}}, instr[30:20]};
        end
        STORE: begin
            op2_imm_D = 1;
            mm_we_D = 1;
            alu_funct3_D = 'b000; // Use alu to add addresses
            imm_D = {{(XLEN-11){instr[31]}}, instr[30:25], instr[11:7]};
            rd_addr_D = 0;
        end
        BRANCH: begin
            jb_D = 1;
            branch_D = 1;
            op1_pc_D = 1;
            op2_imm_D = 1;
            alu_funct3_D = 'b000; // Use alu to add addresses
            imm_D = {{(XLEN-12){instr[31]}}, instr[7], instr[30:25], instr[11:8], 1'b0};
            rd_addr_D = 0;
        end
        JALR: begin
            jb_D = 1;
            rs2_addr_D = 0;
            jump_D = 1;
            op2_imm_D = 1;
            wb_pc_D = 1;
            alu_funct3_D = 'b000; // Use alu to add addresses
            imm_D = {{(XLEN-11){instr[31]}}, instr[30:20]};
        end
        MISC_MEM: ; // Do nothing, no cache and in-order
        JAL: begin
            jb_D = 1;
            rs1_addr_D = 0;
            rs2_addr_D = 0;
            jump_D = 1;
            op1_pc_D = 1;
            op2_imm_D = 1;
            wb_pc_D = 1;
            alu_funct3_D = 'b000; // Use alu to add addresses
            imm_D = {{(XLEN-20){instr[31]}}, instr[19:12], instr[20], instr[30:21], 1'b0};
        end
        OP_IMM: begin
            rs2_addr_D = 0;
            op2_imm_D = 1;
            wb_ieu_D = 1;
            imm_D = {{(XLEN-11){instr[31]}}, instr[30:20]};
            // Special shift case
            if(funct3_D == 'b101 | funct3_D == 'b001) begin
                funct7_D = XLEN==32?instr[31:25]:{instr[31:26], 1'b0};
                imm_D = XLEN==32?{{(XLEN-5){1'b0}}, instr[24:20]}:{{(XLEN-6){1'b0}}, instr[25:20]};
            end
        end
        OP: begin
            wb_ieu_D = 1;
        end
        SYSTEM: ; // Do nothing, no breakpoint support
        AUIPC: begin
            rs1_addr_D = 0;
            rs2_addr_D = 0;
            op1_pc_D = 1;
            op2_imm_D = 1;
            wb_ieu_D = 1;
            alu_funct3_D = 'b000; // Use alu to add immediate
            imm_D = {{(XLEN-31){instr[31]}}, instr[30:12], {12{1'b0}}};
        end
        LUI: begin
            rs1_addr_D = 0;
            rs2_addr_D = 0;
            op2_imm_D = 1;
            wb_ieu_D = 1;
            alu_funct3_D = 'b000; // Use alu to add immediate to zero
            imm_D = {{(XLEN-31){instr[31]}}, instr[30:12], {12{1'b0}}};
        end
        default: ;
        endcase
    end

    always_ff @(posedge clk or negedge reset_n) begin
        if(!(stall_D | stall_M | !reset_n)) begin
            jump_E <= jump_D;
            branch_E <= branch_D;
            op1_pc_E <= op1_pc_D;
            op2_imm_E <= op2_imm_D;
            alu_funct3_E <= alu_funct3_D;
            funct3_E <= funct3_D;
            funct7_E <= funct7_D;
            imm_E <= imm_D;
            mm_re_E <= mm_re_D;
            mm_we_E <= mm_we_D;
            wb_ieu_E <= wb_ieu_D;
            wb_lsu_E <= wb_lsu_D;
            wb_pc_E <= wb_pc_D;
            rd_addr_E <= rd_addr_D;
        end
        else if(stall_D | !reset_n) begin
            jump_E <= 0;
            branch_E <= 0;
            op1_pc_E <= 0;
            op2_imm_E <= 0;
            alu_funct3_E <= 0;
            funct3_E <= 0;
            funct7_E <= 0;
            imm_E <= 0;
            mm_re_E <= 0;
            mm_we_E <= 0;
            wb_ieu_E <= 0;
            wb_lsu_E <= 0;
            wb_pc_E <= 0;
            rd_addr_E <= 0;
        end
        if(!stall_M) begin
            mm_re_M <= mm_re_E;
            mm_we_M <= mm_we_E;
            funct3_M <= funct3_E;
            wb_ieu_M <= wb_ieu_E;
            wb_lsu_M <= wb_lsu_E;
            wb_pc_M <= wb_pc_E;
            rd_addr_M <= rd_addr_E;
            wb_ieu_W <= wb_ieu_M;
            wb_lsu_W <= wb_lsu_M;
            wb_pc_W <= wb_pc_M;
            rd_addr_W <= rd_addr_M;
        end
        else begin
            wb_ieu_W <= 0;
            wb_lsu_W <= 0;
            wb_pc_W <= 0;
            rd_addr_W <= 0;
        end
    end

    hc hc (
        .rs1_addr_D,
        .rs2_addr_D,
        .rd_addr({rd_addr_E, rd_addr_M, rd_addr_W}),

        .stall_D
    );
endmodule
