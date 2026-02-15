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
    pub fn set_data_r(&mut self, val: u64) { unsafe { vcore_set_data_r(self.ptr, val); } }

    // Bus Output Getters
    pub fn get_instr_re(&self) -> u8 { unsafe { vcore_get_instr_re(self.ptr) } }
    pub fn get_instr_addr(&self) -> u64 { unsafe { vcore_get_instr_addr(self.ptr) } }
    pub fn get_instr_sel(&self) -> u8 { unsafe { vcore_get_instr_sel(self.ptr) } }
    pub fn get_data_re(&self) -> u8 { unsafe { vcore_get_data_re(self.ptr) } }
    pub fn get_data_addr(&self) -> u64 { unsafe { vcore_get_data_addr(self.ptr) } }
    pub fn get_data_sel(&self) -> u8 { unsafe { vcore_get_data_sel(self.ptr) } }
    pub fn get_data_we(&self) -> u8 { unsafe { vcore_get_data_we(self.ptr) } }
    pub fn get_data_w(&self) -> u64 { unsafe { vcore_get_data_w(self.ptr) } }
    
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
        // Expected results are now u64
        let expected_results: [u64; 23] = [
            // --- 1. 64-bit Computational ---
            0x1122334455667899, // [0] ADDI (0x1122334455667788 + 0x111)
            0x0000000000000001, // [1] SLTI (True)
            0xE1D2C3B4A5968778, // [2] XORI
            0x11223344FFFFFFFF, // [3] ORI
            0x1122000000007788, // [4] ANDI

            // --- 2. 64-bit Shifts ---
            0x00000000000000F0, // [5] SLLI (0x8...F << 4) (Overflows top bit)
            0x0800000000000000, // [6] SRLI (Logical Right)
            0xF800000000000000, // [7] SRAI (Arithmetic Right - Sign preserved)

            // --- 3. Word (32-bit) Ops (Sign Extension Checks) ---
            0xFFFFFFFF80000000, // [8] ADDW (0x40000000 + 0x40000000 = 0x80000000 -> Sign Ext)
            0x0000000023456780, // [9] SLLW (0x12345678 << 4)
            0xFFFFFFFFFF000000, // [10] SRAW (0xF0000000 >> 4 -> 0xFF000000 -> Sign Ext)

            // --- 4. M Extension ---
            0xFFFFFFFFFFFFFC18, // [11] MUL (100 * -10 = -1000 = ...FC18)
            0xFFFFFFFFFFFFFFF6, // [12] DIV (100 / -10 = -10)
            0x000000000000000A, // [13] REM (100 % 30 = 10)
            0x0000000000000000, // [14] MULW (0x10000*0x10000=0 in 32-bit)

            // --- 5. Register Ops ---
            0x00000000FFFFFFFF, // [15] ADD
            0x0000000055555555, // [16] SUB
            0x00000000FFFFFFFF, // [17] OR
            0x0000000000000000, // [18] AND

            // --- 6. Memory Access ---
            0x8877665544332211, // [19] LD (Little Endian full load)
            0x0000000044332211, // [20] LWU (Zero extended)
            0xFFFFFFFFFFFFFFFF, // [21] LW (Sign extended -1)

            // --- Final ---
            0xDEADBEEFDEADBEEF  // [22] Final Marker
        ];

        let mut core = Core::new();
        let binary = env!("PAYLOAD_STRESSTEST");
        let mut memory = std::fs::read(binary).unwrap();
        memory.resize(0x1000000, 0);
        
        // Ensure instruction bus is set up for 64-bit if necessary, 
        // though usually instruction fetch is still 32-bit wide for standard RISC-V 
        // unless using compressed instructions or wide fetch. 
        // Assuming standard 32-bit wide instruction bus here based on `C2cInstr`.
        let mut instr_bus = C2cInstr::new(0);
        
        // Assuming Data Bus needs to handle 64-bit width now? 
        // If your C2cData is strictly 32-bit, you will need to run two cycles 
        // or update the bus width. 
        // *If the bus is 64-bit:*
        let mut data_bus = C2cData::new(0); 

        let mut instr_ack;
        let mut data_ack;
        let mut data_r;
        let mut instr;

        core.reset();
        for _ in 0..2000 { // Increased ticks for potentially longer execution
            (instr_ack, instr) = instr_bus.respond(
                &memory, 
                core.get_instr_re() == 1, 
                core.get_instr_sel(), 
                core.get_instr_addr()
            );
            core.set_instr_ack(instr_ack as u8);
            core.set_instr_data(instr);

            (data_ack, data_r) = data_bus.respond(
                &mut memory, 
                core.get_data_we() == 1, 
                core.get_data_re() == 1, 
                core.get_data_sel(), 
                core.get_data_addr(), 
                core.get_data_w()
            );
            core.set_data_ack(data_ack as u8);
            core.set_data_r(data_r);
            core.tick();
        }

        for i in 0..23 {
            // Updated to read 8 bytes (u64)
            // Offset is now i * 8
            let addr = 0x2000 + i * 8;
            let val_bytes = memory[addr..(addr + 8)].try_into().unwrap();
            let test_result = u64::from_le_bytes(val_bytes);
            
            assert_eq!(
                test_result, 
                expected_results[i], 
                "Mismatch at index [{}]: Expected {:#X}, Got {:#X}", 
                i, expected_results[i], test_result
            );
        }
    }
}
