#include "Vlsu.h" // Note: Verilator generates based on the top module name
#include "verilated.h"
#include "verilated_vcd_c.h"

extern "C"
{
    // Constructor & Destructor
    Vlsu *vlsu_init() { return new Vlsu(); }
    void vlsu_destroy(Vlsu *dut) { delete dut; }

    // Simulation Control
    void vlsu_eval(Vlsu *dut) { dut->eval(); }

    // Port Setters
    void vlsu_set_clk(Vlsu *dut, uint8_t val) { dut->clk = val; }
    void vlsu_set_reset_n(Vlsu *dut, uint8_t val) { dut->reset_n = val; }
    void vlsu_set_dr_ack(Vlsu *dut, uint8_t val) { dut->dr_ack = val; }
    void vlsu_set_dw_ack(Vlsu *dut, uint8_t val) { dut->dw_ack = val; }
    void vlsu_set_mm_we(Vlsu *dut, uint8_t val) { dut->mm_we = val; }
    void vlsu_set_mm_re(Vlsu *dut, uint8_t val) { dut->mm_re = val; }
    void vlsu_set_funct3(Vlsu *dut, uint8_t val) { dut->funct3 = val; }
    
    void vlsu_set_dr_data(Vlsu *dut, uint32_t val) { dut->dr_data = val; }
    void vlsu_set_ieu_result(Vlsu *dut, uint32_t val) { dut->ieu_result = val; }
    void vlsu_set_rs2_data(Vlsu *dut, uint32_t val) { dut->rs2_data = val; }

    // Port Getters
    uint8_t  vlsu_get_stall(Vlsu *dut) { return dut->stall; }
    uint8_t  vlsu_get_dr_re(Vlsu *dut) { return dut->dr_re; }
    uint8_t  vlsu_get_dr_sel(Vlsu *dut) { return dut->dr_sel; }
    uint8_t  vlsu_get_dw_we(Vlsu *dut) { return dut->dw_we; }
    uint8_t  vlsu_get_dw_sel(Vlsu *dut) { return dut->dw_sel; }

    uint32_t vlsu_get_dr_addr(Vlsu *dut) { return dut->dr_addr; }
    uint32_t vlsu_get_dw_addr(Vlsu *dut) { return dut->dw_addr; }
    uint32_t vlsu_get_dw_data(Vlsu *dut) { return dut->dw_data; }
    uint32_t vlsu_get_data_out(Vlsu *dut) { return dut->data_out; }

    // Tracing
    VerilatedVcdC *vlsu_trace_init(Vlsu *dut, const char *filename)
    {
        Verilated::traceEverOn(true);
        VerilatedVcdC *tfp = new VerilatedVcdC();
        dut->trace(tfp, 99);
        tfp->open(filename);
        return tfp;
    }
    void vlsu_trace_dump(VerilatedVcdC *tfp, uint64_t time) { tfp->dump(time); }
    void vlsu_trace_close(VerilatedVcdC *tfp) { tfp->close(); }
}