// Core to cache write interface
interface c2c_w #(
    parameter XLEN = 32
);
    logic we, ack;
    logic [XLEN/8-1:0] sel;
    logic [XLEN-1:0] addr, data;

    modport master (
        input ack,

        output we,
        output sel,
        output addr,
        output data
    );

    modport slave (
        input we,
        input sel,
        input addr,
        input data,

        output ack
    )
endinterface