unsafe extern "C" {
    pub fn vcore_init() -> *mut std::ffi::c_void;
    pub fn vcore_destroy(dut: *mut std::ffi::c_void);
    pub fn vcore_eval(dut: *mut std::ffi::c_void);
    
    pub fn vcore_set_clk(dut: *mut std::ffi::c_void, val: u8);
    pub fn vcore_set_reset_n(dut: *mut std::ffi::c_void, val: u8);
    
    // Inputs from Memory/Peripherals
    pub fn vcore_set_instr_ack(dut: *mut std::ffi::c_void, val: u8);
    pub fn vcore_set_instr_data(dut: *mut std::ffi::c_void, val: u32);
    pub fn vcore_set_data_ack(dut: *mut std::ffi::c_void, val: u8);
    pub fn vcore_set_data_r(dut: *mut std::ffi::c_void, val: u32);

    // Outputs to Memory/Peripherals
    pub fn vcore_get_instr_re(dut: *mut std::ffi::c_void) -> u8;
    pub fn vcore_get_instr_addr(dut: *mut std::ffi::c_void) -> u32;
    pub fn vcore_get_instr_sel(dut: *mut std::ffi::c_void) -> u8;
    pub fn vcore_get_data_re(dut: *mut std::ffi::c_void) -> u8;
    pub fn vcore_get_data_addr(dut: *mut std::ffi::c_void) -> u32;
    pub fn vcore_get_data_sel(dut: *mut std::ffi::c_void) -> u8;
    pub fn vcore_get_data_we(dut: *mut std::ffi::c_void) -> u8;
    pub fn vcore_get_data_w(dut: *mut std::ffi::c_void) -> u32;
    pub fn vcore_trace_init(
        dut: *mut std::ffi::c_void,
        filename: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_void;
    pub fn vcore_trace_dump(vcd: *mut std::ffi::c_void, time: u64);
    pub fn vcore_trace_close(vcd: *mut std::ffi::c_void);
}
