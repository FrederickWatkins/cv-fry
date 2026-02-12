package pipeline;
    localparam XLEN = 32;

    typedef struct packed {
        logic [31:2] instr;
        logic [XLEN-1:0] curr_pc;
        logic [XLEN-1:0] inc_pc; 
    } decode_signals;

    typedef struct packed {
        logic jump;
        logic branch;
        logic op1_pc;
        logic op2_imm;
        logic [2:0] funct3;
        logic [6:0] funct7;
        logic [XLEN-1:0] imm;
        logic [XLEN-1:0] curr_pc;
        logic [XLEN-1:0] inc_pc;
        logic mm_re;
        logic mm_we;
        logic [XLEN-1:0] data;
        logic rd_we;
        logic [4:0] rd_addr;
    } execute_signals;

    typedef struct packed {
        logic [2:0] funct3;
        logic mm_re;
        logic mm_we;
        logic [XLEN-1:0] data;
        logic rd_we;
        logic [4:0] rd_addr;
    } memory_signals;

    typedef struct packed {
        logic [XLEN-1:0] data;
        logic rd_we;
        logic [4:0] rd_addr;
    } writeback_signals;
endpackage
