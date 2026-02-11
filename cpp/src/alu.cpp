#include "Valu.h"
#include "verilated.h"
#include "verilated_vcd_c.h"

extern "C" {
    // Constructor & Destructor
    Valu* valu_init() { return new Valu(); }
    void valu_destroy(Valu* dut) { delete dut; }

    // Simulation Control
    void valu_eval(Valu* dut) { dut->eval(); }
    
    // Port Setters (Booleans/8-bit)
    void valu_set_funct3(Valu* dut, uint8_t val) { dut->funct3 = val; }
    void valu_set_funct7(Valu* dut, uint8_t val) { dut->funct7 = val; }

    // Port Setters (32-bit)
    void valu_set_operand_1(Valu* dut, uint32_t val) { dut->operand_1 = val; }
    void valu_set_operand_2(Valu* dut, uint32_t val) { dut->operand_2 = val; }

    // Port Getters
    uint32_t valu_get_result(Valu* dut) { return dut->result; }

    // Tracing
    VerilatedVcdC* valu_trace_init(Valu* dut, const char* filename) {
        Verilated::traceEverOn(true);
        VerilatedVcdC* tfp = new VerilatedVcdC();
        dut->trace(tfp, 99);
        tfp->open(filename);
        return tfp;
    }
    void valu_trace_dump(VerilatedVcdC* tfp, uint64_t time) { tfp->dump(time); }
    void valu_trace_close(VerilatedVcdC* tfp) { tfp->close(); }
}