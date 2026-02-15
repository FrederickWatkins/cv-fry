import pipeline::memory_signals;
import pipeline::writeback_signals;
// Load store unit
module lsu (
    c2c_data.master data_bus,

    input memory_signals signals_in,

    output logic busy,
    output writeback_signals signals_out
);
    import pipeline::*;
    
    localparam BYTE = 2'b00;
    localparam HALF = 2'b01;
    localparam WORD = 2'b10;
    localparam DOUBLE = 2'b11;

    localparam SIGNED = 1'b0;
    localparam UNSIGNED = 1'b1;

    assign data_bus.addr = signals_in.mm_addr;
    assign data_bus.data_w = signals_in.data;

    assign signals_out.rd_addr = signals_in.rd_addr;
    
    always_comb begin
        case(signals_in.funct3[1:0])
        BYTE: begin
            data_bus.sel = 'b00000001;
        end
        HALF: begin
            data_bus.sel = 'b00000011;
        end
        WORD: begin
            data_bus.sel = 'b00001111;
        end
        DOUBLE: begin
            data_bus.sel = 'b11111111;
        end
        endcase
        
        busy = 0;
        signals_out.data = signals_in.data;
        data_bus.re = 0;
        data_bus.we = 0;
        if(signals_in.mm_re) begin
            busy = 1;
            case(signals_in.funct3)
                {SIGNED, BYTE}: signals_out.data = {{(XLEN-7){data_bus.data_r[7]}}, data_bus.data_r[6:0]};
                {SIGNED, HALF}: signals_out.data = {{(XLEN-15){data_bus.data_r[15]}}, data_bus.data_r[14:0]};
                {SIGNED, WORD}: signals_out.data = {{(XLEN-31){data_bus.data_r[31]}}, data_bus.data_r[30:0]};
                {SIGNED, DOUBLE}: signals_out.data = data_bus.data_r;
                {UNSIGNED, BYTE}: signals_out.data = {{(XLEN-8){1'b0}}, data_bus.data_r[7:0]};
                {UNSIGNED, HALF}: signals_out.data = {{(XLEN-16){1'b0}}, data_bus.data_r[15:0]};
                {UNSIGNED, WORD}: signals_out.data = {{(XLEN-32){1'b0}}, data_bus.data_r[31:0]};
                default: signals_out.data = 'x;
            endcase
            data_bus.re = 1;
            data_bus.we = 0;
            if(data_bus.ack) begin
                busy = 0;
                data_bus.re = 0;
            end
        end
        if(signals_in.mm_we) begin
            busy = 1;
            data_bus.re = 0;
            data_bus.we = 1;
            if(data_bus.ack) begin
                busy = 0;
                data_bus.we = 0;
            end
        end
    end
endmodule
