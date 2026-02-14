import pipeline::XLEN;
// Core to cache write interface
interface c2c_data;
    logic re, we, ack;
    logic [XLEN/8-1:0] sel;
    logic [XLEN-1:0] addr, data_r, data_w;

    modport master (
        input ack,
        input data_r,

        output re,
        output we,
        output sel,
        output addr,
        output data_w
    );

    modport slave (
        input re,
        input we,
        input sel,
        input addr,
        input data_w,

        output ack,
        output data_r
    );
endinterface
