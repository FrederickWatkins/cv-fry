module pipeline_reg #(
    parameter type T = logic,
    parameter T NOP = 0
) (
    input logic clk,
    input logic reset_n,
    input logic stall,
    input logic flush,
    input T signals_in,
    output T signals_out
);
    always_ff @(posedge clk or negedge reset_n) begin
        if(!reset_n) signals_out <= NOP;
        else begin
            if(!stall) signals_out <= signals_in;
            if(flush) signals_out <= NOP;
        end
    end
endmodule
