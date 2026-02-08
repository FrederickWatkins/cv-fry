// Instruction decompressor
module decomp (
    input logic [31:0] instr_in,
    output logic compressed,
    output logic [31:2] instr_out
);
    always_comb begin
        compressed = 0;
        if(instr_in[1:0] == 'b11) instr_out = instr_in[31:2];
        else begin
            //$warning("decompression unimplemented %0d", instr_in);
            // TODO decompression logic
            compressed = 1;
            instr_out = 30'h4; // NOP
        end
    end
endmodule
