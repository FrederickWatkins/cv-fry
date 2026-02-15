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
    void vlsu_set_data_ack(Vlsu *dut, uint8_t val) { dut->data_ack = val; }
    void vlsu_set_mm_we(Vlsu *dut, uint8_t val) { dut->mm_we = val; }
    void vlsu_set_mm_re(Vlsu *dut, uint8_t val) { dut->mm_re = val; }
    void vlsu_set_funct3(Vlsu *dut, uint8_t val) { dut->funct3 = val; }
    void vlsu_set_rd_addr_in(Vlsu *dut, uint8_t val) { dut->rd_addr_in = val; }
    void vlsu_set_atomic_in(Vlsu *dut, uint8_t val) { dut->atomic_in = val; }
    void vlsu_set_funct5(Vlsu *dut, uint8_t val) { dut->funct5 = val; }
    
    void vlsu_set_data_r(Vlsu *dut, uint64_t val) { dut->data_r = val; }
    void vlsu_set_data_in(Vlsu *dut, uint64_t val) { dut->data_in = val; }
    void vlsu_set_mm_addr(Vlsu *dut, uint64_t val) { dut->mm_addr = val; }

    // Port Getters
    uint8_t  vlsu_get_busy(Vlsu *dut) { return dut->busy; }
    uint8_t  vlsu_get_data_re(Vlsu *dut) { return dut->data_re; }
    uint8_t  vlsu_get_data_sel(Vlsu *dut) { return dut->data_sel; }
    uint8_t  vlsu_get_data_we(Vlsu *dut) { return dut->data_we; }
    uint8_t  vlsu_get_rd_addr_out(Vlsu *dut) { return dut->rd_addr_out; }
    uint8_t vlsu_get_atomic_out(Vlsu *dut) { return dut->atomic_out; }
    uint8_t vlsu_get_amo_op(Vlsu *dut) { return dut->amo_op; }

    uint64_t vlsu_get_data_addr(Vlsu *dut) { return dut->data_addr; }
    uint64_t vlsu_get_data_w(Vlsu *dut) { return dut->data_w; }
    uint64_t vlsu_get_data_out(Vlsu *dut) { return dut->data_out; }

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