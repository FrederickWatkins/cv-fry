import pipeline::*;
// Hazard controller
module hc (
    input decode_signals    signals_F,
    input execute_signals   signals_D,
    input execute_signals   signals_E,
    input memory_signals    signals_M,
    input writeback_signals signals_W,

    output logic stall_F, output logic flush_F,
    output logic stall_D, output logic flush_D,
    output logic stall_E, output logic flush_E,
    output logic stall_M, output logic flush_M,
    output logic stall_W, output logic flush_W
);
endmodule
