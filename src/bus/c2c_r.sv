// Core to cache read interface
interface c2c_r #(
    parameter XLEN = 32
);
    logic re, ack;
    logic [XLEN/8-1:0] sel;
    logic [XLEN-1:0] addr, data;

    modport master (
        input ack,
        input data,

        output re,
        output sel,
        output addr
    );

    modport slave (
        input re,
        input sel,
        input addr,

        output ack,
        output data
    );
endinterface
