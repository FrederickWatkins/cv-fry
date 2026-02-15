#include "Vifu.h"
#include "verilated.h"
#include "verilated_vcd_c.h"

extern "C"
{
    // Constructor & Destructor
    Vifu *vifu_init() { return new Vifu(); }
    void vifu_destroy(Vifu *dut) { delete dut; }

    // Simulation Control
    void vifu_eval(Vifu *dut) { dut->eval(); }

    // Port Setters (Booleans/8-bit)
    void vifu_set_clk(Vifu *dut, uint8_t val) { dut->clk = val; }
    void vifu_set_reset_n(Vifu *dut, uint8_t val) { dut->reset_n = val; }
    void vifu_set_stall(Vifu *dut, uint8_t val) { dut->stall = val; }
    void vifu_set_je(Vifu *dut, uint8_t val) { dut->je = val; }
    void vifu_set_ack(Vifu *dut, uint8_t val) { dut->ack = val; }

    // Port Setters (32-bit)
    void vifu_set_ja(Vifu *dut, uint64_t val) { dut->ja = val; }
    void vifu_set_instr(Vifu *dut, uint32_t val) { dut->instr = val; }

    // Port Getters (Booleans/8-bit)
    uint8_t vifu_get_re(Vifu *dut) { return dut->re; }
    uint8_t vifu_get_sel(Vifu *dut) { return dut->sel; }

    // Port Getters (32-bit)
    uint64_t vifu_get_curr_pc(Vifu *dut) { return dut->curr_pc; }
    uint64_t vifu_get_inc_pc(Vifu *dut) { return dut->inc_pc; }
    uint64_t vifu_get_addr(Vifu *dut) { return dut->addr; }
    uint32_t vifu_get_instr_out(Vifu *dut) { return dut->instr_out; }

    // Tracing
    VerilatedVcdC *vifu_trace_init(Vifu *dut, const char *filename)
    {
        Verilated::traceEverOn(true);
        VerilatedVcdC *tfp = new VerilatedVcdC();
        dut->trace(tfp, 99);
        tfp->open(filename);
        return tfp;
    }
    void vifu_trace_dump(VerilatedVcdC *tfp, uint64_t time) { tfp->dump(time); }
    void vifu_trace_close(VerilatedVcdC *tfp) { tfp->close(); }
}