use cv_fry_cpp::core::*;
use crate::utils::dut::{DutComb, DutSync};

pub struct Core {
    ptr: *mut std::ffi::c_void,
    vcd: Option<*mut std::ffi::c_void>,
    pub time: u64,
}

impl Core {
    pub fn new() -> Self {
        Self { ptr: unsafe { vcore_init() }, vcd: None, time: 0 }
    }

    // Bus Input Setters
    pub fn set_instr_ack(&mut self, val: u8) { unsafe { vcore_set_instr_ack(self.ptr, val); } }
    pub fn set_instr_data(&mut self, val: u32) { unsafe { vcore_set_instr_data(self.ptr, val); } }
    pub fn set_data_ack(&mut self, val: u8) { unsafe { vcore_set_data_ack(self.ptr, val); } }
    pub fn set_data_r(&mut self, val: u32) { unsafe { vcore_set_data_r(self.ptr, val); } }

    // Bus Output Getters
    pub fn get_instr_re(&self) -> u8 { unsafe { vcore_get_instr_re(self.ptr) } }
    pub fn get_instr_addr(&self) -> u32 { unsafe { vcore_get_instr_addr(self.ptr) } }
    pub fn get_instr_sel(&self) -> u8 { unsafe { vcore_get_instr_sel(self.ptr) } }
    pub fn get_data_re(&self) -> u8 { unsafe { vcore_get_data_re(self.ptr) } }
    pub fn get_data_addr(&self) -> u32 { unsafe { vcore_get_data_addr(self.ptr) } }
    pub fn get_data_sel(&self) -> u8 { unsafe { vcore_get_data_sel(self.ptr) } }
    pub fn get_data_we(&self) -> u8 { unsafe { vcore_get_data_we(self.ptr) } }
    pub fn get_data_w(&self) -> u32 { unsafe { vcore_get_data_w(self.ptr) } }
    
}

impl Drop for Core {
    fn drop(&mut self) {
        unsafe { vcore_destroy(self.ptr) };
        self.trace_close();
    }
}

impl DutComb for Core {
    fn eval(&mut self) {
        unsafe { vcore_eval(self.ptr); }
    }
}

impl DutSync for Core {
    fn set_clk(&mut self, val: u8) {
        unsafe { vcore_set_clk(self.ptr, val); }
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

    fn trace_init(&mut self, filename: &str) {
        self.vcd = Some(unsafe {vcore_trace_init(self.ptr, std::ffi::CString::new(filename).unwrap().as_ptr())});
    }
    
    fn trace_dump(&mut self) {
        if let Some(vcd) = self.vcd {
            unsafe {vcore_trace_dump(vcd, self.time);}
        }
    }
    
    fn trace_close(&mut self) {
        if let Some(vcd) = self.vcd {
            unsafe {vcore_trace_close(vcd);}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::c2c_instr::C2cInstr;
    use crate::bus::c2c_data::C2cData;

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
        let mut instr_bus = C2cInstr::new(1);
        let mut ack;
        let mut instr;
        core.reset();
        for _ in 0..30 {
            (ack, instr) = instr_bus.respond(&riscv_machine_code, core.get_instr_re()==1, core.get_instr_sel(), core.get_instr_addr());
            core.set_instr_ack(ack as u8);
            core.set_instr_data(instr);
            core.tick();
        }

        assert_eq!(core.get_data_w(), 0xDECAFBAD - 0xDEADBEEF);
        assert_eq!(core.get_data_we(), 1);
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
        let mut instr_bus = C2cInstr::new(0);
        let mut data_bus = C2cData::new(0);
        let mut instr_ack;
        let mut data_ack;
        let mut data_r;
        let mut instr;
        core.reset();
        for _ in 0..1000 {
            (instr_ack, instr) = instr_bus.respond(&memory, core.get_instr_re()==1, core.get_instr_sel(), core.get_instr_addr());
            core.set_instr_ack(instr_ack as u8);
            core.set_instr_data(instr);
            (data_ack, data_r) = data_bus.respond(&mut memory, core.get_data_we()==1, core.get_data_re()==1, core.get_data_sel(), core.get_data_addr(), core.get_data_w());
            core.set_data_ack(data_ack as u8);
            core.set_data_r(data_r);
            core.tick();
        }
        for i in 0..23 {
            let test_result = u32::from_le_bytes(memory[(0x2000 + i * 4)..(0x2000 + i * 4 + 4)].try_into().unwrap());
            assert_eq!(test_result, expected_results[i]);
        }
    }
}
