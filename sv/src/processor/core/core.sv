// Processor core
module core (
    input logic clk,
    input logic reset_n,

    c2c_r.master instr_bus,

    c2c_r.master data_bus_r,
    c2c_w.master data_bus_w
);
    import pipeline::*;

    logic stall_F, stall_D, stall_E, stall_M, stall_W;
    logic flush_D, flush_E, flush_M, flush_W;
    logic je, busy_M;
    logic [XLEN-1:0] ja;
    logic [4:0] rs1_addr, rs2_addr;
    logic [XLEN-1:0] rs1_data, rs2_data;
    
    decode_signals signals_out_F, signals_in_D;
    execute_signals signals_out_D, signals_in_E;
    memory_signals signals_out_E, signals_in_M;
    writeback_signals signals_out_M, signals_in_W;

    // Hazard controller
    hc hc (
        .je,
        .busy_M,

        .rs1_addr(signals_out_D.rs1_addr),
        .rs2_addr(signals_out_D.rs2_addr),
        .rd_E(signals_in_E.rd_addr),
        .rd_M(signals_in_M.rd_addr),
        .rd_W(signals_in_W.rd_addr),

        .stall_F,
        .stall_D, .flush_D,
        .stall_E, .flush_E,
        .stall_M, .flush_M,
        .stall_W, .flush_W
    );

    // Instruction fetch unit
    ifu ifu (
        .clk,
        .reset_n,

        .instr_bus(instr_bus),

        .stall(stall_F),
        .je,
        .ja,

        .signals_out(signals_out_F)
    );

    // Instruction decode unit
    idu idu (
        .signals_in(signals_in_D),

        .signals_out(signals_out_D)
    );

    // Integer execution unit
    ieu ieu (
        .rs1_data,
        .rs2_data,
        .signals_in(signals_in_E),

        .je,
        .ja,
        .rs1_addr,
        .rs2_addr,
        .signals_out(signals_out_E)
    );

    // Load store unit
    lsu lsu (
        .data_bus_r,
        .data_bus_w,

        .signals_in(signals_in_M),

        .busy(busy_M),
        .signals_out(signals_out_M)
    );

    // Register file
    rf rf (
        .clk,
        // No reset_n, register file must be initialized by software
        .rs1_addr,
        .rs2_addr,
        .signals_in(signals_in_W),
        
        .rs1_data,
        .rs2_data
    );

    // Fetch-decode pipeline register
    pipeline_reg #(
        .T(decode_signals),
        .NOP('{'h4, 0, 0})
    ) pipeline_D (
        .clk,
        .reset_n,
        .stall(stall_D),
        .flush(flush_D),
        .signals_in(signals_out_F),
        .signals_out(signals_in_D)
    );

    // Decode-exectue pipeline register
    pipeline_reg #(
        .T(execute_signals),
        .NOP('{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0})
    ) pipeline_E (
        .clk,
        .reset_n,
        .stall(stall_E),
        .flush(flush_E),
        .signals_in(signals_out_D),
        .signals_out(signals_in_E)
    );

    // Execute-memory pipeline register
    pipeline_reg #(
        .T(memory_signals),
        .NOP('{0, 0, 0, 0, 0, 0})
    ) pipeline_M (
        .clk,
        .reset_n,
        .stall(stall_M),
        .flush(flush_M),
        .signals_in(signals_out_E),
        .signals_out(signals_in_M)
    );

    // Memory-writeback pipeline register
    pipeline_reg #(
        .T(writeback_signals),
        .NOP('{0, 0})
    ) pipeline_W (
        .clk,
        .reset_n,
        .stall(stall_W),
        .flush(flush_W),
        .signals_in(signals_out_M),
        .signals_out(signals_in_W)
    );
endmodule
