import pipeline::decode_signals;
import pipeline::execute_signals;
// Decoder
module idu (
    input decode_signals signals_in,

    output execute_signals signals_out
);
    import pipeline::*;

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

    assign signals_out.curr_pc = signals_in.curr_pc;
    assign signals_out.inc_pc = signals_in.inc_pc;

    assign opcode = signals_in.instr[6:2];

    always_comb begin
        signals_out.rs1_addr = signals_in.instr[19:15];
        signals_out.rs2_addr = signals_in.instr[24:20];
        signals_out.jump = 0;
        signals_out.branch = 0;
        signals_out.op1_pc = 0;
        signals_out.op2_imm = 0;
        signals_out.mm_re = 0;
        signals_out.mm_we = 0;
        signals_out.alu_funct3 = signals_in.instr[14:12];
        signals_out.funct3 = signals_in.instr[14:12];
        signals_out.funct7 = signals_in.instr[31:25];
        signals_out.imm = 0;
        signals_out.rd_addr = signals_in.instr[11:7];
        case(opcode)
        LOAD: begin
            signals_out.rs2_addr = 0;
            signals_out.op2_imm = 1;
            signals_out.mm_re = 1;
            signals_out.alu_funct3 = 'b000; // Use alu to add addresses
            signals_out.funct7 = 0;
            signals_out.imm = {{(XLEN-11){signals_in.instr[31]}}, signals_in.instr[30:20]};
        end
        STORE: begin
            signals_out.op2_imm = 1;
            signals_out.mm_we = 1;
            signals_out.alu_funct3 = 'b000; // Use alu to add addresses
            signals_out.funct7 = 0;
            signals_out.imm = {{(XLEN-11){signals_in.instr[31]}}, signals_in.instr[30:25], signals_in.instr[11:7]};
            signals_out.rd_addr = 0;
        end
        BRANCH: begin
            signals_out.branch = 1;
            signals_out.op1_pc = 1;
            signals_out.op2_imm = 1;
            signals_out.alu_funct3 = 'b000; // Use alu to add addresses
            signals_out.funct7 = 0;
            signals_out.imm = {{(XLEN-12){signals_in.instr[31]}}, signals_in.instr[7], signals_in.instr[30:25], signals_in.instr[11:8], 1'b0};
            signals_out.rd_addr = 0;
        end
        JALR: begin
            signals_out.rs2_addr = 0;
            signals_out.jump = 1;
            signals_out.op2_imm = 1;
            signals_out.alu_funct3 = 'b000; // Use alu to add addresses
            signals_out.funct7 = 0;
            signals_out.imm = {{(XLEN-11){signals_in.instr[31]}}, signals_in.instr[30:20]};
        end
        MISC_MEM: begin
            // Do nothing, no cache and in-order
            signals_out.rs1_addr = 0;
            signals_out.rs2_addr = 0;
            signals_out.rd_addr = 0;
        end
        JAL: begin
            signals_out.rs1_addr = 0;
            signals_out.rs2_addr = 0;
            signals_out.jump = 1;
            signals_out.op1_pc = 1;
            signals_out.op2_imm = 1;
            signals_out.alu_funct3 = 'b000; // Use alu to add addresses
            signals_out.funct7 = 0;
            signals_out.imm = {{(XLEN-20){signals_in.instr[31]}}, signals_in.instr[19:12], signals_in.instr[20], signals_in.instr[30:21], 1'b0};
        end
        OP_IMM: begin
            signals_out.rs2_addr = 0;
            signals_out.op2_imm = 1;
            signals_out.funct7 = 0;
            signals_out.imm = {{(XLEN-11){signals_in.instr[31]}}, signals_in.instr[30:20]};
            // Special shift case
            if(signals_out.funct3 == 'b101 | signals_out.funct3 == 'b001) begin
                signals_out.funct7 = XLEN==32?signals_in.instr[31:25]:{signals_in.instr[31:26], 1'b0};
                signals_out.imm = XLEN==32?{{(XLEN-5){1'b0}}, signals_in.instr[24:20]}:{{(XLEN-6){1'b0}}, signals_in.instr[25:20]};
            end
        end
        OP: ; // Default settings
        SYSTEM: begin
            // Do nothing, no breakpoint support
            signals_out.rs1_addr = 0;
            signals_out.rs2_addr = 0;
            signals_out.rd_addr = 0;
        end
        AUIPC: begin
            signals_out.rs1_addr = 0;
            signals_out.rs2_addr = 0;
            signals_out.op1_pc = 1;
            signals_out.op2_imm = 1;
            signals_out.alu_funct3 = 'b000; // Use alu to add immediate
            signals_out.funct7 = 0;
            signals_out.imm = {{(XLEN-31){signals_in.instr[31]}}, signals_in.instr[30:12], {12{1'b0}}};
        end
        LUI: begin
            signals_out.rs1_addr = 0;
            signals_out.rs2_addr = 0;
            signals_out.op2_imm = 1;
            signals_out.alu_funct3 = 'b000; // Use alu to add immediate to zero
            signals_out.funct7 = 0;
            signals_out.imm = {{(XLEN-31){signals_in.instr[31]}}, signals_in.instr[30:12], {12{1'b0}}};
        end
        default: begin
            $warning("Unsupported opcode");
            signals_out.rs1_addr = 0;
            signals_out.rs2_addr = 0;
            signals_out.rd_addr = 0;
        end
        endcase
    end
endmodule
