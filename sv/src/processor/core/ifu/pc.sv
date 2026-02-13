import pipeline::XLEN;
// Program counter
module pc (
    input logic clk,
    input logic reset_n,

    input logic stall,
    input logic compressed,
    input logic je,
    input logic [XLEN-1:0] ja,

    output logic [XLEN-1:0] curr_pc,
    output logic [XLEN-1:0] inc_pc,
    output logic [XLEN-1:0] next_pc
);

    assign inc_pc = compressed?curr_pc+2:curr_pc+4;

    always_comb begin
        next_pc = inc_pc;
        if(je) begin
            next_pc = ja;
        end
        if(stall) begin
            next_pc = curr_pc;
        end
    end

    always_ff @(posedge clk or negedge reset_n) begin
        if(!reset_n) curr_pc <= 0;
        else curr_pc <= next_pc;
    end
endmodule
