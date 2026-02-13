import pipeline::XLEN;
// Arithmetic logic unit
module alu (
    // Control
    input [2:0] funct3,
    input [6:0] funct7,

    // Operands
    input [XLEN-1:0] operand_1,
    input [XLEN-1:0] operand_2,

    output logic [XLEN-1:0] result
);
    // Funct3 values where funct7 = b0000000
    localparam BASEOP = 7'b0000000;
    localparam ADD = 3'b000; // Add op1 to op2
    localparam SL = 3'b001; // Left shift
    localparam SLT = 3'b010; // Set less than (op1 < op2)
    localparam SLTU = 3'b011; // Set less than unsigned
    localparam XOR = 3'b100; // Bitwise XOR
    localparam SRL = 3'b101; // Right shift
    localparam OR = 3'b110; // Bitwise OR
    localparam AND = 3'b111; // Bitwise AND

    // Funct3 values where funct7 = b0100000
    localparam ALTOP = 7'b0100000;
    localparam SUB = 3'b000;
    localparam SRA = 3'b101;

    localparam shamt_len = XLEN == 32 ? 5 : 6;

    always_comb begin
        case({funct7, funct3})
        {BASEOP, ADD}: result = operand_1 + operand_2;
        {BASEOP, SL}: result = operand_1 << operand_2[shamt_len-1:0];
        {BASEOP, SLT}: result = {{(XLEN-1){1'b0}}, $signed(operand_1) < $signed(operand_2)};
        {BASEOP, SLTU}: result = {{(XLEN-1){1'b0}}, operand_1 < operand_2};
        {BASEOP, XOR}: result = operand_1 ^ operand_2;
        {BASEOP, SRL}: result = operand_1>>operand_2[shamt_len-1:0];
        {BASEOP, OR}: result = operand_1 | operand_2;
        {BASEOP, AND}: result = operand_1 & operand_2;
        {ALTOP, SUB}: result = operand_1 - operand_2;
        {ALTOP, SRA}: result = $unsigned($signed(operand_1)>>>operand_2[shamt_len-1:0]);
        default: result = 'x;
        endcase
    end

endmodule
