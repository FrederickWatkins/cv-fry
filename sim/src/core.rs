use cv_fry_cpp::core::*;
use crate::utils::dut::DUT;

pub struct Core {
    ptr: *mut std::ffi::c_void,
    pub time: u64,
}

impl Core {
    pub fn new() -> Self {
        Self { ptr: unsafe { vcore_init() }, time: 0 }
    }

    // Bus Input Setters
    pub fn set_instr_ack(&mut self, val: u8) { unsafe { vcore_set_instr_ack(self.ptr, val); } }
    pub fn set_instr_data(&mut self, val: u32) { unsafe { vcore_set_instr_data(self.ptr, val); } }
    pub fn set_dr_ack(&mut self, val: u8) { unsafe { vcore_set_dr_ack(self.ptr, val); } }
    pub fn set_dr_data(&mut self, val: u32) { unsafe { vcore_set_dr_data(self.ptr, val); } }
    pub fn set_dw_ack(&mut self, val: u8) { unsafe { vcore_set_dw_ack(self.ptr, val); } }

    // Bus Output Getters
    pub fn get_instr_re(&self) -> u8 { unsafe { vcore_get_instr_re(self.ptr) } }
    pub fn get_instr_addr(&self) -> u32 { unsafe { vcore_get_instr_addr(self.ptr) } }
    pub fn get_instr_sel(&self) -> u8 { unsafe { vcore_get_instr_sel(self.ptr) } }
    pub fn get_dr_re(&self) -> u8 { unsafe { vcore_get_dr_re(self.ptr) } }
    pub fn get_dr_addr(&self) -> u32 { unsafe { vcore_get_dr_addr(self.ptr) } }
    pub fn get_dr_sel(&self) -> u8 { unsafe { vcore_get_dr_sel(self.ptr) } }
    pub fn get_dw_we(&self) -> u8 { unsafe { vcore_get_dw_we(self.ptr) } }
    pub fn get_dw_addr(&self) -> u32 { unsafe { vcore_get_dw_addr(self.ptr) } }
    pub fn get_dw_data(&self) -> u32 { unsafe { vcore_get_dw_data(self.ptr) } }
    pub fn get_dw_sel(&self) -> u8 { unsafe { vcore_get_dw_sel(self.ptr) } }
}

impl Drop for Core {
    fn drop(&mut self) {
        unsafe { vcore_destroy(self.ptr) };
    }
}

impl DUT for Core {
    fn set_clk(&mut self, val: u8) {
        unsafe { vcore_set_clk(self.ptr, val); }
    }

    fn eval(&mut self) {
        unsafe { vcore_eval(self.ptr); }
    }

    fn timestep(&mut self) {
        self.time += 5;
    }

    fn reset(&mut self) {
        unsafe {
            self.set_clk(0);
            self.eval();
            vcore_set_reset_n(self.ptr, 0);
            self.tick();
            vcore_set_reset_n(self.ptr, 1);
            self.eval();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::c2c_r::C2cR;
    use crate::utils::c2c_w::C2cW;

    #[test]
    fn sub_and_store() {
        let mut core = Core::new();
        let riscv_machine_code: [u8; 68] = [
            // nop
            0x13, 0x00, 0x00, 0x00,
            // nop
            0x13, 0x00, 0x00, 0x00,
            // lui a0, 0xdeadc
            0x37, 0xC5, 0xAD, 0xDE,
            // addi a0, a0, -273 (0xdeadbeef)
            0x13, 0x05, 0xF5, 0xEE,
            // lui a1, 0xdecb0
            0xB7, 0x05, 0xCB, 0xDE,
            // addi a1, a1, -1107 (0xdecafbad)
            0x93, 0x85, 0xD5, 0xBA,
            // sub a2, a1, a0
            0x33, 0x86, 0xA5, 0x40,
            // li sp, 0
            0x13, 0x01, 0x00, 0x00,
            // sw a2, 0(sp)
            0x23, 0x20, 0xC1, 0x00,
            // nop
            0x13, 0x00, 0x00, 0x00,
            // nop
            0x13, 0x00, 0x00, 0x00,
            // nop
            0x13, 0x00, 0x00, 0x00,
            // nop
            0x13, 0x00, 0x00, 0x00,
            // nop
            0x13, 0x00, 0x00, 0x00,
            // nop
            0x13, 0x00, 0x00, 0x00,
            // nop
            0x13, 0x00, 0x00, 0x00,
            // nop
            0x13, 0x00, 0x00, 0x00,
        ];
        let mut instr_bus = C2cR::new(1);
        let mut ack;
        let mut instr;
        core.reset();
        for _ in 0..30 {
            (ack, instr) = instr_bus.respond(&riscv_machine_code, core.get_instr_re()==1, core.get_instr_sel(), core.get_instr_addr());
            core.set_instr_ack(ack as u8);
            core.set_instr_data(instr);
            core.tick();
        }

        assert_eq!(core.get_dw_data(), 0xDECAFBAD - 0xDEADBEEF);
        assert_eq!(core.get_dw_we(), 1);
    }

    #[test]
    fn stresstest() {
        let expected_results: [u32; 23] = [
            0x12345E77, // [0]  ADDI (a + 0x7FF)
            0x12344E78, // [1]  ADDI (a - 0x800)
            0x00000000, // [2]  SLTI (a < 0x100) -> False
            0x00000001, // [3]  SLTIU (a < 0x7FFFFFFF) -> True
            0x12345987, // [4]  XORI
            0x12345679, // [5]  ORI
            0x00000228, // [6]  ANDI
            0x000000F0, // [7]  SLLI (0x8000000F << 4)
            0x08000000, // [8]  SRLI (Logical Shift Right)
            0xF8000000, // [9]  SRAI (Arithmetic Shift Right - Sign preserved)
            0xFFFFFFFF, // [10] ADD (0x55555555 + 0xAAAAAAAA)
            0xAAAAAAAB, // [11] SUB (0x55555555 - 0xAAAAAAAA)
            0xFFFFFFFF, // [12] XOR
            0xFFFFFFFF, // [13] OR
            0x00000000, // [14] AND
            0x00000000, // [15] SLT (Signed: Positive < Negative) -> False
            0x0000000B, // [16] branch_check (Sum of successful branches: 1 + 2 + 8)
            0x44332211, // [17] LW (Little-endian load)
            0x00006655, // [18] LH
            0xFFFFFF88, // [19] LB (Sign extended 0x88)
            0x00000088, // [20] LBU (Zero extended 0x88)
            0xABCDE000, // [21] LUI (Upper immediate)
            0xDEADBEEF  // [22] Final Success Marker
        ];
        let mut core = Core::new();
        let binary = env!("PAYLOAD_STRESSTEST");
        let mut memory = std::fs::read(binary).unwrap();
        memory.resize(0x1000000, 0);
        let mut instr_bus = C2cR::new(0);
        let mut data_bus_r = C2cR::new(0);
        let mut data_bus_w = C2cW::new(0);
        let mut instr_ack;
        let mut data_r_ack;
        let mut data_r;
        let mut data_w_ack;
        let mut instr;
        core.reset();
        for _ in 0..1000 {
            (instr_ack, instr) = instr_bus.respond(&memory, core.get_instr_re()==1, core.get_instr_sel(), core.get_instr_addr());
            core.set_instr_ack(instr_ack as u8);
            core.set_instr_data(instr);
            (data_r_ack, data_r) = data_bus_r.respond(&memory, core.get_dr_re()==1, core.get_dr_sel(), core.get_dr_addr());
            core.set_dr_ack(data_r_ack as u8);
            core.set_dr_data(data_r);
            data_w_ack = data_bus_w.respond(&mut memory, core.get_dw_we()==1, core.get_dw_sel(), core.get_dw_addr(), core.get_dw_data());
            core.set_dw_ack(data_w_ack as u8);
            core.tick();
        }
        for i in 0..23 {
            let test_result = u32::from_le_bytes(memory[(0x2000 + i * 4)..(0x2000 + i * 4 + 4)].try_into().unwrap());
            assert_eq!(test_result, expected_results[i]);
        }
    }
}
