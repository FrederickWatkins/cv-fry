import pipeline::XLEN;
// Arithmetic logic unit
module alu (
    // Control
    input logic [2:0] funct3,
    input logic [6:0] funct7,
    input logic word,

    // Operands
    input logic [XLEN-1:0] operand_1,
    input logic [XLEN-1:0] operand_2,

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

    // Funct3 values where funct7 = b0000001
    localparam MULDIV = 7'b0000001;
    localparam MUL = 3'b000;
    localparam MULH = 3'b001; // Signed
    localparam MULHSU = 3'b010; // Signed*unsigned
    localparam MULHU = 3'b011; // Unsigned*unsigned
    localparam DIV = 3'b100;
    localparam DIVU = 3'b101;
    localparam REM = 3'b110;
    localparam REMU = 3'b111;

    localparam shamt_len = XLEN == 32 ? 5 : 6;

    logic [XLEN-1:0] full_result;

    assign result = word?{{(XLEN-32){full_result[31]}}, full_result[31:0]}:full_result;

    always_comb begin
        case({funct7, funct3})
        {BASEOP, ADD}: full_result = operand_1 + operand_2;
        {BASEOP, SL}: begin
            if(word) full_result = operand_1 << operand_2[4:0];
            else full_result = operand_1 << operand_2[shamt_len-1:0];
        end
        {BASEOP, SLT}: full_result = {{(XLEN-1){1'b0}}, $signed(operand_1) < $signed(operand_2)};
        {BASEOP, SLTU}: full_result = {{(XLEN-1){1'b0}}, operand_1 < operand_2};
        {BASEOP, XOR}: full_result = operand_1 ^ operand_2;
        {BASEOP, SRL}: begin
            if(word) full_result = {{32{1'b0}}, operand_1[31:0]}>>operand_2[4:0]; 
            else full_result = operand_1>>operand_2[shamt_len-1:0];
        end
        {BASEOP, OR}: full_result = operand_1 | operand_2;
        {BASEOP, AND}: full_result = operand_1 & operand_2;
        {ALTOP, SUB}: full_result = operand_1 - operand_2;
        {ALTOP, SRA}: begin
            if(word) full_result = $unsigned($signed(operand_1)>>>operand_2[4:0]);
            else full_result = $unsigned($signed(operand_1)>>>operand_2[shamt_len-1:0]);
        end
        {MULDIV, MUL}: full_result = operand_1 * operand_2;
        {MULDIV, MULH}: begin
            if(word) full_result = XLEN'({{(XLEN + 32){operand_1[31]}}, operand_1[31:0]} * {{(XLEN + 32){operand_2[31]}}, operand_2[31:0]} >> 32);
            else full_result = XLEN'({{(XLEN){operand_1[XLEN-1]}}, operand_1} * {{(XLEN){operand_2[XLEN-1]}}, operand_2} >> XLEN);
        end
        {MULDIV, MULHU}: begin
            if(word) full_result = XLEN'({{(XLEN + 32){1'b0}}, operand_1[31:0]} * {{(XLEN + 32){1'b0}}, operand_2[31:0]} >> 32);
            else full_result = XLEN'({{(XLEN){1'b0}}, operand_1} * {{(XLEN){1'b0}}, operand_2} >> XLEN);
        end
        {MULDIV, MULHSU}: begin
            if(word) full_result = XLEN'({{(XLEN + 32){operand_1[31]}}, operand_1[31:0]} * {{(XLEN + 32){1'b0}}, operand_2[31:0]} >> 32);
            else full_result = XLEN'({{(XLEN){operand_1[XLEN-1]}}, operand_1} * {{(XLEN){1'b0}}, operand_2} >> XLEN);
        end
        {MULDIV, DIV}: begin
            if(operand_2 == 0) full_result = {(XLEN){1'b1}};
            else if(operand_1 == {1'b1, {(XLEN-1){1'b0}}} && operand_2 == {(XLEN){1'b1}}) full_result = {1'b1, {(XLEN-1){1'b0}}};
            else full_result = $signed(operand_1) / $signed(operand_2);
        end
        {MULDIV, DIVU}: begin
            if(operand_2 == 0) full_result = {(XLEN){1'b1}};
            else if(word) full_result = {{32{1'b0}}, operand_1[31:0]} / {{32{1'b0}}, operand_2[31:0]};
            else full_result = operand_1 / operand_2;
        end
        {MULDIV, REM}: begin
            if(operand_2 == 0) full_result = operand_1;
            else if(operand_1 == {1'b1, {(XLEN-1){1'b0}}} && operand_2 == {(XLEN){1'b1}}) full_result = 0;
            else full_result = $signed(operand_1) % $signed(operand_2);
        end
        {MULDIV, REMU}: begin
            if(operand_2 == 0) full_result = operand_1;
            else if(word) full_result = {{32{1'b0}}, operand_1[31:0]} % {{32{1'b0}}, operand_2[31:0]};
            else full_result = operand_1 % operand_2;
        end
        default: full_result = 'x;
        endcase
    end

endmodule
