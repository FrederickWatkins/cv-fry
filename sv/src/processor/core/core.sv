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
    logic flush_F, flush_D, flush_E, flush_M, flush_W;
    
    decode_signals signals_out_F, signals_in_D;
    execute_signals signals_out_D, signals_in_E;
    memory_signals signals_out_E, signals_in_M;
    writeback_signals signals_out_M, signals_in_W;

    // Hazard controller
    hc hc (
        .signals_F(signals_out_F),
        .signals_D(signals_out_D),
        .signals_E(signals_in_E),
        .signals_M(signals_in_M),
        .signals_W(signals_in_W),

        .stall_F, .flush_F,
        .stall_D, .flush_D,
        .stall_E, .flush_E,
        .stall_M, .flush_M,
        .stall_W, .flush_W
    );

    // Instruction fetch unit
    ifu ifu (
        .clk,
        .reset_n,
        .stall(stall_F),
        .flush(flush_F),
        .instr_bus,
        .je,
        .ja,

        .signals_out(signals_out_F)
    );

    // Instruction decode unit
    idu idu (
        .signals_in(signals_in_D),

        .rs1_addr,
        .rs2_addr,
        .signals_out(signals_out_D)
    );

    // Integer execution unit
    ieu ieu (
        .rs1_data,
        .rs2_data,
        .signals_in(signals_in_E),

        .je,
        .ja,
        .signals_out(signals_out_E)
    );

    // Load store unit
    lsu lsu (
        .clk,
        .reset_n,
        .signals_in(signals_in_M),

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
        .NOP('{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0})
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
        .NOP('{0, 0, 0})
    ) pipeline_W (
        .clk,
        .reset_n,
        .stall(stall_W),
        .flush(flush_W),
        .signals_in(signals_out_M),
        .signals_out(signals_in_W)
    );
endmodule
