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
    void vcore_set_dr_ack(Vcore *dut, uint8_t val) { dut->dr_ack = val; }
    void vcore_set_dr_data(Vcore *dut, uint32_t val) { dut->dr_data = val; }
    void vcore_set_dw_ack(Vcore *dut, uint8_t val) { dut->dw_ack = val; }

    // Bus Output Getters
    uint8_t  vcore_get_instr_re(Vcore *dut) { return dut->instr_re; }
    uint32_t vcore_get_instr_addr(Vcore *dut) { return dut->instr_addr; }
    uint8_t  vcore_get_instr_sel(Vcore *dut) { return dut->instr_sel; }
    
    uint8_t  vcore_get_dr_re(Vcore *dut) { return dut->dr_re; }
    uint32_t vcore_get_dr_addr(Vcore *dut) { return dut->dr_addr; }
    uint8_t  vcore_get_dr_sel(Vcore *dut) { return dut->dr_sel; }
    
    uint8_t  vcore_get_dw_we(Vcore *dut) { return dut->dw_we; }
    uint32_t vcore_get_dw_addr(Vcore *dut) { return dut->dw_addr; }
    uint32_t vcore_get_dw_data(Vcore *dut) { return dut->dw_data; }
    uint8_t  vcore_get_dw_sel(Vcore *dut) { return dut->dw_sel; }

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