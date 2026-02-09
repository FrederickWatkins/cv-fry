// Load store unit
module lsu #(
    parameter XLEN = 32
) (
    input logic clk,
    input logic reset_n,
    // Interfaces
    c2c_r.master data_bus_r,
    c2c_w.master data_bus_w,
    // Inputs
    input logic mm_we,
    input logic mm_re,
    input logic [2:0] funct3,
    input logic [XLEN-1:0] ieu_result,
    input logic [XLEN-1:0] rs2_data,
    // Outputs
    output logic stall,
    output logic [XLEN-1:0] data
);
    // TODO add write buffering
    typedef enum logic [1:0] {
        IDLE,
        READ,
        WRITE
    } state;
    
    localparam BYTE = 2'b00;
    localparam HALF = 2'b01;
    localparam WORD = 2'b10;

    localparam SIGNED = 1'b0;
    localparam UNSIGNED = 1'b1;

    state curr_state;
    state next_state;

    assign data_bus_r.addr = ieu_result;
    assign data_bus_w.addr = ieu_result;
    assign data_bus_w.data = rs2_data;
    
    always_comb begin
        next_state = curr_state;
        case(funct3[1:0])
        BYTE: begin
            data_bus_r.sel = 'b0001;
            data_bus_w.sel = 'b0001;
        end
        HALF: begin
            data_bus_r.sel = 'b0011;
            data_bus_w.sel = 'b0011;
        end
        WORD: begin
            data_bus_r.sel = 'b1111;
            data_bus_w.sel = 'b1111;
        end
        endcase
        case(curr_state)
        IDLE: begin
            stall = 0;
            output_data = 'x;
            data_bus_r.re = 0;
            data_bus_w.we = 0;
            if(mm_re) begin
                next_state = READ;
                stall = 1;
                data_bus_r.re = 1;
            end
            if(mm_we) begin
                next_state = WRITE;
                data_bus_w.we = 1;
                stall = 1;
            end
        end
        READ: begin
            stall = 1;
            case(funct3)
                {SIGNED, BYTE}: output_data = {{(XLEN-7){data_bus_r.data[7]}}, data_bus_r.data[6:0]};
                {SIGNED, HALF}: output_data = {{(XLEN-15){data_bus_r.data[15]}}, data_bus_r.data[14:0]};
                {SIGNED, WORD}: output_data = {{(XLEN-31){data_bus_r.data[31]}}, data_bus_r.data[30:0]};
                {UNSIGNED, BYTE}: output_data = {{(XLEN-8){1'b0}}, data_bus_r.data[7:0]};
                {UNSIGNED, HALF}: output_data = {{(XLEN-16){1'b0}}, data_bus_r.data[15:0]};
                {UNSIGNED, WORD}: output_data = {{(XLEN-32){1'b0}}, data_bus_r.data[31:0]};
                default: output_data = 'x;
            endcase
            data_bus_r.re = 1;
            data_bus_w.we = 0;
            if(data_bus_r.ack) begin
                next_state = IDLE;
                stall = 0;
                data_bus_r.re = 0;
            end
        end
        WRITE: begin
            stall = 1;
            output_data = 'x;
            data_bus_r.re = 0;
            data_bus_w.we = 1;
            if(data_bus_w.ack) begin
                next_state = IDLE;
                stall = 0;
                data_bus.WE = 0;
                data_bus.STB = 0;
                data_bus.CYC = 0;
            end
        end
        default: ;
        endcase
    end

    always_ff @(posedge clk or negedge reset_n) begin
        curr_state <= next_state;
        if(!reset_n) curr_state <= IDLE;
        data <= output_data;
    end
endmodule
