// Instruction fetch unit
module ifu #(
    parameter XLEN = 32
) (
    input logic clk,
    input logic reset_n,

    c2c_r.master instr_bus,

    input logic stall,
    input logic jump,
    input logic jack,
    input logic je,
    input logic [XLEN-1:0] ja,

    output logic [31:2] instr_out,
    output logic [XLEN-1:0] curr_pc,
    output logic [XLEN-1:0] inc_pc
);
    typedef enum logic {
        RUNNING, // Running normally (still stall if pipeline stalls)
        WAIT_JUMP // Waiting for acknowledgement of earlier jump instruction
    } state;

    state curr_state;
    state next_state;

    logic [XLEN-1:0] next_pc;
    logic [31:2] instr_decomp;
    logic pc_stall, compressed;

    assign instr_bus.re = 1'b1;
    assign instr_bus.addr = next_pc;
    assign instr_bus.sel = 4'b1111;

    always_comb begin
        pc_stall = stall | !instr_bus.ack;
        case(curr_state)
        RUNNING: begin
            if(instr_bus.ack) begin
                instr_out = instr_decomp;
            end
            else begin
                instr_out = 'h4; // NOP
            end
            if(jump & !stall) begin
                pc_stall = 1;
                next_state = WAIT_JUMP;
            end
        end
        WAIT_JUMP: begin
            instr_out = 'h4;
            if(jack) begin
                pc_stall = 0;
                next_state = RUNNING;
            end
        end
        endcase
    end

    always_ff @(posedge clk or negedge reset_n) begin
        curr_state <= next_state;
        if(!reset_n) curr_state <= RUNNING;
    end

    pc #(
        .XLEN(XLEN)
    ) pc (
        .clk,
        .reset_n,

        .stall(pc_stall),
        .compressed,
        .je,
        .ja,

        .curr_pc,
        .inc_pc,
        .next_pc
    );

    decomp decomp (
        .instr_in(instr_bus.data),

        .compressed,
        .instr_out(instr_decomp)
    );
endmodule
