use cv_fry::utils::dut::DUT;

unsafe extern "C" {
    fn vcore_init() -> *mut std::ffi::c_void;
    fn vcore_destroy(dut: *mut std::ffi::c_void);
    fn vcore_eval(dut: *mut std::ffi::c_void);

    fn vcore_set_clk(dut: *mut std::ffi::c_void, val: u8);
    fn vcore_set_reset_n(dut: *mut std::ffi::c_void, val: u8);

    // Inputs from Memory/Peripherals
    fn vcore_set_instr_ack(dut: *mut std::ffi::c_void, val: u8);
    fn vcore_set_instr_data(dut: *mut std::ffi::c_void, val: u32);
    fn vcore_set_dr_ack(dut: *mut std::ffi::c_void, val: u8);
    fn vcore_set_dr_data(dut: *mut std::ffi::c_void, val: u32);
    fn vcore_set_dw_ack(dut: *mut std::ffi::c_void, val: u8);

    // Outputs to Memory/Peripherals
    fn vcore_get_instr_re(dut: *mut std::ffi::c_void) -> u8;
    fn vcore_get_instr_addr(dut: *mut std::ffi::c_void) -> u32;
    fn vcore_get_instr_sel(dut: *mut std::ffi::c_void) -> u8;
    fn vcore_get_dr_re(dut: *mut std::ffi::c_void) -> u8;
    fn vcore_get_dr_addr(dut: *mut std::ffi::c_void) -> u32;
    fn vcore_get_dr_sel(dut: *mut std::ffi::c_void) -> u8;
    fn vcore_get_dw_we(dut: *mut std::ffi::c_void) -> u8;
    fn vcore_get_dw_addr(dut: *mut std::ffi::c_void) -> u32;
    fn vcore_get_dw_data(dut: *mut std::ffi::c_void) -> u32;
    fn vcore_get_dw_sel(dut: *mut std::ffi::c_void) -> u8;
    fn vcore_trace_init(
        dut: *mut std::ffi::c_void,
        filename: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_void;
    fn vcore_trace_dump(dut: *mut std::ffi::c_void, time: u64);
    fn vcore_trace_close(dut: *mut std::ffi::c_void);
}

pub struct Core {
    ptr: *mut std::ffi::c_void,
    vcd: *mut std::ffi::c_void,
    pub time: u64,
}

impl Core {
    pub fn new() -> Self {
        let ptr = unsafe { vcore_init() };
        let str = std::ffi::CString::new("core.vcd").unwrap();
        Self {
            ptr: ptr,
            vcd: unsafe { vcore_trace_init(ptr, str.as_ptr()) },
            time: 0,
        }
    }

    // Bus Input Setters
    pub fn set_instr_ack(&mut self, val: u8) {
        unsafe {
            vcore_set_instr_ack(self.ptr, val);
        }
    }
    pub fn set_instr_data(&mut self, val: u32) {
        unsafe {
            vcore_set_instr_data(self.ptr, val);
        }
    }
    pub fn set_dr_ack(&mut self, val: u8) {
        unsafe {
            vcore_set_dr_ack(self.ptr, val);
        }
    }
    pub fn set_dr_data(&mut self, val: u32) {
        unsafe {
            vcore_set_dr_data(self.ptr, val);
        }
    }
    pub fn set_dw_ack(&mut self, val: u8) {
        unsafe {
            vcore_set_dw_ack(self.ptr, val);
        }
    }

    // Bus Output Getters
    pub fn get_instr_re(&self) -> u8 {
        unsafe { vcore_get_instr_re(self.ptr) }
    }
    pub fn get_instr_addr(&self) -> u32 {
        unsafe { vcore_get_instr_addr(self.ptr) }
    }
    pub fn get_instr_sel(&self) -> u8 {
        unsafe { vcore_get_instr_sel(self.ptr) }
    }
    pub fn get_dr_re(&self) -> u8 {
        unsafe { vcore_get_dr_re(self.ptr) }
    }
    pub fn get_dr_addr(&self) -> u32 {
        unsafe { vcore_get_dr_addr(self.ptr) }
    }
    pub fn get_dr_sel(&self) -> u8 {
        unsafe { vcore_get_dr_sel(self.ptr) }
    }
    pub fn get_dw_we(&self) -> u8 {
        unsafe { vcore_get_dw_we(self.ptr) }
    }
    pub fn get_dw_addr(&self) -> u32 {
        unsafe { vcore_get_dw_addr(self.ptr) }
    }
    pub fn get_dw_data(&self) -> u32 {
        unsafe { vcore_get_dw_data(self.ptr) }
    }
    pub fn get_dw_sel(&self) -> u8 {
        unsafe { vcore_get_dw_sel(self.ptr) }
    }
}

impl Drop for Core {
    fn drop(&mut self) {
        unsafe {
            vcore_trace_close(self.vcd);
            vcore_destroy(self.ptr)
        };
    }
}

impl DUT for Core {
    fn set_clk(&mut self, val: u8) {
        unsafe {
            vcore_set_clk(self.ptr, val);
        }
    }

    fn eval(&mut self) {
        unsafe {
            vcore_eval(self.ptr);
            vcore_trace_dump(self.vcd, self.time);
        }
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