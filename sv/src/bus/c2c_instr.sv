import pipeline::XLEN;
// Core to cache instruction interface
interface c2c_instr;
    logic re, ack;
    logic [3:0] sel;
    logic [XLEN-1:0] addr;
    logic [31:0] instr;

    modport master (
        input ack,
        input instr,

        output re,
        output sel,
        output addr
    );

    modport slave (
        input re,
        input sel,
        input addr,

        output ack,
        output instr
    );
endinterface
