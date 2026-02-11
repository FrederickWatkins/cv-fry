#include "Vpc.h"
#include "verilated.h"
#include "verilated_vcd_c.h"

extern "C" {
    // Constructor & Destructor
    Vpc* vpc_init() { return new Vpc(); }
    void vpc_destroy(Vpc* dut) { delete dut; }

    // Simulation Control
    void vpc_eval(Vpc* dut) { dut->eval(); }
    
    // Port Setters (Booleans/8-bit)
    void vpc_set_clk(Vpc* dut, uint8_t val) { dut->clk = val; }
    void vpc_set_reset_n(Vpc* dut, uint8_t val) { dut->reset_n = val; }
    void vpc_set_stall(Vpc* dut, uint8_t val) { dut->stall = val; }
    void vpc_set_compressed(Vpc* dut, uint8_t val) { dut->compressed = val; }
    void vpc_set_je(Vpc* dut, uint8_t val) { dut->je = val; }
    
    // Port Setters (32-bit)
    void vpc_set_ja(Vpc* dut, uint32_t val) { dut->ja = val; }

    // Port Getters
    uint32_t vpc_get_curr_pc(Vpc* dut) { return dut->curr_pc; }
    uint32_t vpc_get_inc_pc(Vpc* dut) { return dut->inc_pc; }
    uint32_t vpc_get_next_pc(Vpc* dut) { return dut->next_pc; }

    // Tracing
    VerilatedVcdC* vpc_trace_init(Vpc* dut, const char* filename) {
        Verilated::traceEverOn(true);
        VerilatedVcdC* tfp = new VerilatedVcdC();
        dut->trace(tfp, 99);
        tfp->open(filename);
        return tfp;
    }
    void vpc_trace_dump(VerilatedVcdC* tfp, uint64_t time) { tfp->dump(time); }
    void vpc_trace_close(VerilatedVcdC* tfp) { tfp->close(); }
}