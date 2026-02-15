#include "Vcore.h"
#include "verilated.h"
#include "verilated_vcd_c.h"

extern "C"
{
    Vcore *vcore_init() { return new Vcore(); }
    void vcore_destroy(Vcore *dut) { delete dut; }
    void vcore_eval(Vcore *dut) { dut->eval(); }

    // Control Setters
    void vcore_set_clk(Vcore *dut, uint8_t val) { dut->clk = val; }
    void vcore_set_reset_n(Vcore *dut, uint8_t val) { dut->reset_n = val; }

    // Bus Input Setters (Acks and Data)
    void vcore_set_instr_ack(Vcore *dut, uint8_t val) { dut->instr_ack = val; }
    void vcore_set_instr_data(Vcore *dut, uint32_t val) { dut->instr_data = val; }
    void vcore_set_data_ack(Vcore *dut, uint8_t val) { dut->data_ack = val; }
    void vcore_set_data_r(Vcore *dut, uint64_t val) { dut->data_r = val; }

    // Bus Output Getters
    uint8_t  vcore_get_instr_re(Vcore *dut) { return dut->instr_re; }
    uint64_t vcore_get_instr_addr(Vcore *dut) { return dut->instr_addr; }
    uint8_t  vcore_get_instr_sel(Vcore *dut) { return dut->instr_sel; }
    
    uint8_t  vcore_get_data_re(Vcore *dut) { return dut->data_re; }
    uint64_t vcore_get_data_addr(Vcore *dut) { return dut->data_addr; }
    uint8_t  vcore_get_data_sel(Vcore *dut) { return dut->data_sel; }
    uint8_t  vcore_get_data_we(Vcore *dut) { return dut->data_we; }
    uint8_t vcore_get_atomic(Vcore *dut) { return dut->atomic; }
    uint8_t vcore_get_amo_op(Vcore *dut) { return dut->amo_op; }
    uint64_t vcore_get_data_w(Vcore *dut) { return dut->data_w; }

    // Tracing boilerplate
    VerilatedVcdC *vcore_trace_init(Vcore *dut, const char *filename) {
        Verilated::traceEverOn(true);
        VerilatedVcdC *tfp = new VerilatedVcdC();
        dut->trace(tfp, 99);
        tfp->open(filename);
        return tfp;
    }
    void vcore_trace_dump(VerilatedVcdC *tfp, uint64_t time) { tfp->dump(time); }
    void vcore_trace_close(VerilatedVcdC *tfp) { tfp->close(); }
}