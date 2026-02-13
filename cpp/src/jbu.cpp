#include "Vjbu.h"
#include "verilated.h"
#include "verilated_vcd_c.h"

extern "C"
{
    // Constructor & Destructor
    Vjbu *vjbu_init() { return new Vjbu(); }
    void vjbu_destroy(Vjbu *dut) { delete dut; }

    // Simulation Control
    void vjbu_eval(Vjbu *dut) { dut->eval(); }

    // Port Setters
    void vjbu_set_jump(Vjbu *dut, uint8_t val) { dut->jump = val; }
    void vjbu_set_branch(Vjbu *dut, uint8_t val) { dut->branch = val; }
    void vjbu_set_funct3(Vjbu *dut, uint8_t val) { dut->funct3 = val; }
    void vjbu_set_rs1_data(Vjbu *dut, uint32_t val) { dut->rs1_data = val; }
    void vjbu_set_rs2_data(Vjbu *dut, uint32_t val) { dut->rs2_data = val; }

    // Port Getters
    uint8_t vjbu_get_je(Vjbu *dut) { return dut->je; }

    // Tracing
    VerilatedVcdC *vjbu_trace_init(Vjbu *dut, const char *filename)
    {
        Verilated::traceEverOn(true);
        VerilatedVcdC *tfp = new VerilatedVcdC();
        dut->trace(tfp, 99);
        tfp->open(filename);
        return tfp;
    }
    void vjbu_trace_dump(VerilatedVcdC *tfp, uint64_t time) { tfp->dump(time); }
    void vjbu_trace_close(VerilatedVcdC *tfp) { tfp->close(); }
}