// Instruction decompressor
module decomp (
    input logic [31:0] instr_in,
    output logic compressed,
    output logic [31:2] instr_out
);
    // Registers
    localparam LINK = 5'b00001;
    localparam SP = 5'b00010;

    // Decompressed opcodes
    typedef enum logic [4:0] {
        LOAD_OPCODE = 'b00000,
        STORE_OPCODE = 'b01000,
        BRANCH_OPCODE = 'b11000,
        JALR_OPCODE = 'b11001,
        MISC_MEM_OPCODE = 'b00011,
        AMO_OPCODE = 'b01011,
        JAL_OPCODE = 'b11011,
        OP_IMM_OPCODE = 'b00100,
        OP_OPCODE = 'b01100,
        SYSTEM_OPCODE = 'b11100,
        AUIPC_OPCODE = 'b00101,
        LUI_OPCODE = 'b01101,
        OP_IMM_32_OPCODE = 'b00110,
        OP_32_OPCODE = 'b01110
    } opcodes;
    // LSU funct3
    localparam WORD = 3'b010;
    localparam DOUBLE = 3'b011;

    // ALU funct3
    // Funct3 values where funct7 = b0000000
    localparam BASEOP = 7'b0000000;
    localparam ADD_FUNCT3 = 3'b000; // Add op1 to op2
    localparam SL_FUNCT3 = 3'b001; // Left shift
    localparam XOR_FUNCT3 = 3'b100; // Bitwise XOR
    localparam SRL_FUNCT3 = 3'b101; // Right shift
    localparam OR_FUNCT3 = 3'b110; // Bitwise OR
    localparam AND_FUNCT3 = 3'b111; // Bitwise AND

    // Funct3 values where funct7 = b0100000
    localparam ALTOP = 7'b0100000;
    localparam SUB_FUNCT3 = 3'b000;
    localparam SRA_FUNCT3 = 3'b101;

    // Compressed opcodes
    localparam C0 = 2'b00;
    localparam ADDI4SPN = 8'b000_?????;
    localparam LW = 8'b010_?????;
    localparam LD = 8'b011_?????;
    localparam SW = 8'b110_?????;
    localparam SD = 8'b111_?????;

    localparam C1 = 2'b01;
    localparam ADDI = 8'b000_?????;
    localparam ADDIW = 8'b001_?????;
    localparam LI = 8'b010_?????;
    localparam LUI = 8'b011_?????;
    localparam SRLI = 8'b100_?_00_??;
    localparam SRAI = 8'b100_?_01_??;
    localparam ANDI = 8'b100_?_10_??;
    localparam SUB = 8'b100_0_11_00;
    localparam XOR = 8'b100_0_11_01;
    localparam OR = 8'b100_0_11_10;
    localparam AND = 8'b100_0_11_11;
    localparam SUBW = 8'b100_1_11_00;
    localparam ADDW = 8'b100_1_11_01;
    localparam J = 8'b101_?????;
    localparam BEQZ = 8'b110_?????;
    localparam BNEZ = 8'b111_?????;

    localparam C2 = 2'b10;
    localparam SLLI = 8'b000_?????;
    localparam LWSP = 8'b010_?????;
    localparam LDSP = 8'b011_?????;
    localparam MV = 8'b100_0_????;
    localparam ADD = 8'b100_1_????;
    localparam SWSP = 8'b110_?????;
    localparam SDSP = 8'b111_?????;

    localparam C3 = 2'b11;

    logic [4:0] alt_rs1;
    logic [4:0] alt_rs2;

    always_comb begin
        compressed = 1;
        alt_rs1 = {2'b01, instr_in[9:7]};
        alt_rs2 = {2'b01, instr_in[4:2]};
        casez({instr_in[1:0], instr_in[15:10], instr_in[6:5]})
        {C0, ADDI4SPN}: instr_out = {2'b00, instr_in[10:7], instr_in[12:11], instr_in[5], instr_in[6], 2'b00, SP, 3'b000, alt_rs2, OP_IMM_OPCODE};
        {C0, LW}: instr_out = {5'b00000, instr_in[5], instr_in[12:10], instr_in[6], 2'b00, alt_rs1, WORD, alt_rs2, LOAD_OPCODE};
        {C0, LD}: instr_out = {4'b0000, instr_in[6:5], instr_in[12:10], 3'b000, alt_rs1, DOUBLE, alt_rs2, LOAD_OPCODE};
        {C0, SW}: instr_out = {5'b00000, instr_in[5], instr_in[12], alt_rs2, alt_rs1, WORD, instr_in[11:10], instr_in[6], 2'b00, STORE_OPCODE};
        {C0, SD}: instr_out = {4'b0000, instr_in[6:5], instr_in[12], alt_rs2, alt_rs1, DOUBLE, instr_in[11:10], 3'b000, STORE_OPCODE};
        {C1, ADDI}: instr_out = {{7{instr_in[12]}}, instr_in[6:2], instr_in[11:7], ADD_FUNCT3, instr_in[11:7], OP_IMM_OPCODE};
        {C1, ADDIW}: instr_out = {{7{instr_in[12]}}, instr_in[6:2], instr_in[11:7], ADD_FUNCT3, instr_in[11:7], OP_IMM_32_OPCODE};
        {C1, LI}: instr_out = {{7{instr_in[12]}}, instr_in[6:2], 5'b00000, ADD_FUNCT3, instr_in[11:7], OP_IMM_OPCODE}; // Add x0 to intermediate
        {C1, LUI}: begin
            if(instr_in[11:7]==2) instr_out = {{3{instr_in[12]}}, instr_in[4:3], instr_in[5], instr_in[2], instr_in[6], 4'b0000, SP, ADD_FUNCT3, SP, OP_IMM_OPCODE};
            else instr_out = {{15{instr_in[12]}}, instr_in[6:2], instr_in[11:7], LUI_OPCODE};
        end
        {C1, SRLI}: instr_out = {6'b000000, instr_in[12], instr_in[6:2], alt_rs1, SRL_FUNCT3, alt_rs1, OP_IMM_OPCODE};
        {C1, SRAI}: instr_out = {6'b010000, instr_in[12], instr_in[6:2], alt_rs1, SRA_FUNCT3, alt_rs1, OP_IMM_OPCODE};
        {C1, ANDI}: instr_out = {{7{instr_in[12]}}, instr_in[6:2], alt_rs1, AND_FUNCT3, alt_rs1, OP_IMM_OPCODE};
        {C1, SUB}: instr_out = {ALTOP, alt_rs2, alt_rs1, SUB_FUNCT3, alt_rs1, OP_OPCODE};
        {C1, XOR}: instr_out = {BASEOP, alt_rs2, alt_rs1, XOR_FUNCT3, alt_rs1, OP_OPCODE};
        {C1, OR}: instr_out = {BASEOP, alt_rs2, alt_rs1, OR_FUNCT3, alt_rs1, OP_OPCODE};
        {C1, AND}: instr_out = {BASEOP, alt_rs2, alt_rs1, AND_FUNCT3, alt_rs1, OP_OPCODE};
        {C1, SUBW}: instr_out = {ALTOP, alt_rs2, alt_rs1, SUB_FUNCT3, alt_rs1, OP_32_OPCODE};
        {C1, ADDW}: instr_out = {BASEOP, alt_rs2, alt_rs1, ADD_FUNCT3, alt_rs1, OP_32_OPCODE};
        {C1, J}: instr_out = {instr_in[12], instr_in[8], instr_in[10:9], instr_in[6], instr_in[7], instr_in[2], instr_in[11], instr_in[5:3], {9{instr_in[12]}}, 5'b00000, JAL_OPCODE};
        {C1, BEQZ}: instr_out = {{4{instr_in[12]}}, instr_in[6:5], instr_in[2], 5'b00000, alt_rs1, 3'b000, instr_in[11:10], instr_in[4:3], instr_in[12], BRANCH_OPCODE};
        {C1, BNEZ}: instr_out = {{4{instr_in[12]}}, instr_in[6:5], instr_in[2], 5'b00000, alt_rs1, 3'b001, instr_in[11:10], instr_in[4:3], instr_in[12], BRANCH_OPCODE};
        {C2, SLLI}: instr_out = {6'b000000, instr_in[12], instr_in[6:2], instr_in[11:7], SL_FUNCT3, instr_in[11:7], OP_IMM_OPCODE};
        {C2, LWSP}: instr_out = {4'b0000, instr_in[3:2], instr_in[12], instr_in[6:4], 2'b00, SP, WORD, instr_in[11:7], LOAD_OPCODE};
        {C2, LDSP}: instr_out = {3'b000, instr_in[4:2], instr_in[12], instr_in[6:5], 3'b000, SP, DOUBLE, instr_in[11:7], LOAD_OPCODE};
        {C2, MV}: begin
            if(instr_in[6:2] == 0) instr_out = {12'b0, instr_in[11:7], 3'b000, 5'b00000, JALR_OPCODE};
            else instr_out = {BASEOP, instr_in[6:2], 5'b00000, ADD_FUNCT3, instr_in[11:7], OP_OPCODE};
        end
        {C2, ADD}: begin
            if(instr_in[11:2] == 0) instr_out = {12'b1, 13'b0, SYSTEM_OPCODE};
            else if(instr_in[6:2] == 0) instr_out = {12'b0, instr_in[11:7], 3'b000, LINK, JALR_OPCODE};
            else instr_out = {BASEOP, instr_in[6:2], instr_in[11:7], ADD_FUNCT3, instr_in[11:7], OP_OPCODE};
        end
        {C2, SWSP}: instr_out = {4'b0000, instr_in[8:7], instr_in[12], instr_in[6:2], SP, WORD, instr_in[11:9], 2'b00, STORE_OPCODE};
        {C2, SDSP}: instr_out = {3'b000, instr_in[9:7], instr_in[12], instr_in[6:2], SP, DOUBLE, instr_in[11:10], 3'b000, STORE_OPCODE};
        {C3, 8'b????????}: begin
            compressed = 0;
            instr_out = instr_in[31:2];
        end
        default: instr_out = 'x;
        endcase
    end
endmodule
