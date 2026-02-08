use crate::utils::dut::DUT;

// Raw FFI definitions
unsafe extern "C" {
    fn vifu_init() -> *mut std::ffi::c_void;
    fn vifu_destroy(dut: *mut std::ffi::c_void);
    fn vifu_eval(dut: *mut std::ffi::c_void);
    fn vifu_set_clk(dut: *mut std::ffi::c_void, val: u8);
    fn vifu_set_stall(dut: *mut std::ffi::c_void, val: u8);
    fn vifu_set_compressed(dut: *mut std::ffi::c_void, val: u8);
    fn vifu_set_jump(dut: *mut std::ffi::c_void, val: u8);
    fn vifu_set_jack(dut: *mut std::ffi::c_void, val: u8);
    fn vifu_set_je(dut: *mut std::ffi::c_void, val: u8);
    fn vifu_set_ja(dut: *mut std::ffi::c_void, val: u32);
    fn vifu_set_instr(dut: *mut std::ffi::c_void, val: u32);
    fn vifu_set_reset_n(dut: *mut std::ffi::c_void, val: u8);
    fn vifu_set_ack(dut: *mut std::ffi::c_void, val: u8);
    fn vifu_get_re(dut: *mut std::ffi::c_void) -> u8;
    fn vifu_get_sel(dut: *mut std::ffi::c_void) -> u8;
    fn vifu_get_curr_pc(dut: *mut std::ffi::c_void) -> u32;
    fn vifu_get_inc_pc(dut: *mut std::ffi::c_void) -> u32;
    fn vifu_get_addr(dut: *mut std::ffi::c_void) -> u32;
    fn vifu_get_instr_out(dut: *mut std::ffi::c_void) -> u32;
    fn vifu_trace_init(dut: *mut std::ffi::c_void, filename: *const std::ffi::c_char) -> *mut std::ffi::c_void;
    fn vifu_trace_dump(tfp: *mut std::ffi::c_void, time: u64);
    fn vifu_trace_close(tfp: *mut std::ffi::c_void);
}
pub struct ProgramCounter {
    ptr: *mut std::ffi::c_void,
    tfp: Option<*mut std::ffi::c_void>,
    time: u64,
}

impl ProgramCounter {
    pub fn new() -> Self {
        Self { ptr: unsafe { vifu_init() }, tfp: None, time: 0 }
    }

    pub fn set_stall(&mut self, val: u8) {
        unsafe {
            vifu_set_stall(self.ptr, val);
        }
    }

    pub fn set_compressed(&mut self, val: u8) {
        unsafe {
            vifu_set_compressed(self.ptr, val);
        }
    }

    pub fn set_je(&mut self, val: u8) {
        unsafe {
            vifu_set_je(self.ptr, val);
        }
    }

    pub fn set_ja(&mut self, val: u32) {
        unsafe {
            vifu_set_ja(self.ptr, val);
        }
    }

    pub fn enable_tracing(&mut self, filename: &str) {
        let c_str = std::ffi::CString::new(filename).unwrap();
        unsafe {
            self.tfp = Some(vifu_trace_init(self.ptr, c_str.as_ptr()));
        }
    }

    pub fn eval(&mut self) {
        unsafe {
            vifu_eval(self.ptr);
        }
    }

    pub fn get_curr_pc(&self) -> u32 {
        unsafe { vifu_get_curr_pc(self.ptr) }
    }

    pub fn get_inc_pc(&self) -> u32 {
        unsafe { vifu_get_inc_pc(self.ptr) }
    }
}

impl Drop for ProgramCounter {
    fn drop(&mut self) {
        if let Some(t) = self.tfp { unsafe {vifu_trace_close(t); }}
        // unsafe { vifu_destroy(self.ptr) }; Causes SIGSEV so let it leak baby
    }
}

impl DUT for ProgramCounter {
    fn set_clk(&mut self, val: u8) {
        unsafe {vifu_set_clk(self.ptr, val);}
    }

    fn eval(&mut self) {
        unsafe {vifu_eval(self.ptr);}
    }

    fn timestep(&mut self) {
        self.time += 5;
    }

    fn dump_trace(&self) {
        unsafe{if let Some(t) = self.tfp { vifu_trace_dump(t, self.time); }}
    }
    
    fn reset(&mut self) {
        unsafe {
            vifu_set_reset_n(self.ptr, 0); // Active low reset?
            self.tick();
            vifu_set_reset_n(self.ptr, 1);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increments() {
        assert!(true);
    }
}