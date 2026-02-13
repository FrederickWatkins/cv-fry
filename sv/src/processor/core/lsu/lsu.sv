import pipeline::*;
// Load store unit
module lsu (
    c2c_r.master data_bus_r,
    c2c_w.master data_bus_w,

    input memory_signals signals_in,

    output logic busy,
    output writeback_signals signals_out
);
    localparam BYTE = 2'b00;
    localparam HALF = 2'b01;
    localparam WORD = 2'b10;

    localparam SIGNED = 1'b0;
    localparam UNSIGNED = 1'b1;

    assign data_bus_r.addr = signals_out.mm_addr;
    assign data_bus_w.addr = signals_out.mm_addr;
    assign data_bus_w.data = signals_out.data;

    assign signals_out.rd_addr = signals_in.rd_addr;
    
    always_comb begin
        case(signals_in.funct3[1:0])
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
        default: begin
            data_bus_r.sel = 'b1111;
            data_bus_w.sel = 'b1111;
        end
        endcase
        
        busy_M = 0;
        signals_out.data = signals_in.data;
        data_bus_r.re = 0;
        data_bus_w.we = 0;
        if(signals_in.mm_re) begin
            busy_M = 1;
            case(signals_in.funct3)
                {SIGNED, BYTE}: signals_out.data = {{(XLEN-7){data_bus_r.data[7]}}, data_bus_r.data[6:0]};
                {SIGNED, HALF}: signals_out.data = {{(XLEN-15){data_bus_r.data[15]}}, data_bus_r.data[14:0]};
                {SIGNED, WORD}: signals_out.data = {{(XLEN-31){data_bus_r.data[31]}}, data_bus_r.data[30:0]};
                {UNSIGNED, BYTE}: signals_out.data = {{(XLEN-8){1'b0}}, data_bus_r.data[7:0]};
                {UNSIGNED, HALF}: signals_out.data = {{(XLEN-16){1'b0}}, data_bus_r.data[15:0]};
                {UNSIGNED, WORD}: signals_out.data = {{(XLEN-32){1'b0}}, data_bus_r.data[31:0]};
                default: signals_out.data = 'x;
            endcase
            data_bus_r.re = 1;
            data_bus_w.we = 0;
            if(data_bus_r.ack) begin
                busy_M = 0;
                data_bus_r.re = 0;
            end
        end
        if(signals_in.mm_we) begin
            busy_M = 1;
            data_bus_r.re = 0;
            data_bus_w.we = 1;
            if(data_bus_w.ack) begin
                busy_M = 0;
                data_bus_w.we = 0;
            end
        end
    end
endmodule
