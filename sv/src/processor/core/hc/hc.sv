import pipeline::*;
// Hazard controller
module hc (
    input logic je,
    input logic busy_M,

    input logic [4:0] rs1_addr, input logic [4:0] rs2_addr,
    input logic [4:0] rd_E,
    input logic mm_re_E,

    output logic stall_F,
    output logic stall_D, output logic flush_D,
    output logic stall_E, output logic flush_E,
    output logic stall_M, output logic flush_M,
    output logic stall_W, output logic flush_W
);
    // Priorities are complicated, so to explain:
    // 1. Ff the memory is busy, the whole pipeline needs to stall. A jump cannot occur while
    // stalled, so we ignore the jump until the next cycle. The execute register is stalled so
    // the jump instruction will still be there. Bubble the writeback stage.
    // 2. If the memory isn't stalled, check if we've jumped. If we have, the two instructions
    // we've fetched since the jump instruction are invalid, so flush them. In this case we don't
    // need to check for a RAW hazard since the instruction in the FD register isn't valid anyway
    // and is about to be flushed.
    // 3. If the memory isn't stalled, and we haven't jumped, check for RAW hazard. If there is
    // one, stall the ifu and FD reg and insert a bubble in the execute stage.
    always_comb begin
        stall_F = 0;
        stall_D = 0; flush_D = 0;
        stall_E = 0; flush_E = 0;
        stall_M = 0; flush_M = 0;
        stall_W = 0; flush_W = 0;
        // LSU busy, stall whole pipeline
        if(busy_M) begin
            stall_F = 1;
            stall_D = 1;
            stall_E = 1;
            stall_M = 1;
            flush_W = 1;
        end
        // Flush pipeline if jump/branch taken
        // JBU is in execute stage so E onwards is sound
        else if(je) begin
            flush_D = 1;
            flush_E = 1;
        end
        // RAW hazard
        else if(
            (rs1_addr == rd_E | rs2_addr == rd_E) && rd_E != 0 && mm_re_E
        ) begin
            stall_F = 1;
            stall_D = 1;
            flush_E = 1;
        end
    end
endmodule
