// Raw FFI definitions
unsafe extern "C" {
    pub fn vpc_init() -> *mut std::ffi::c_void;
    pub fn vpc_destroy(dut: *mut std::ffi::c_void);
    pub fn vpc_eval(dut: *mut std::ffi::c_void);
    pub fn vpc_set_clk(dut: *mut std::ffi::c_void, val: u8);
    pub fn vpc_set_stall(dut: *mut std::ffi::c_void, val: u8);
    pub fn vpc_set_compressed(dut: *mut std::ffi::c_void, val: u8);
    pub fn vpc_set_je(dut: *mut std::ffi::c_void, val: u8);
    pub fn vpc_set_ja(dut: *mut std::ffi::c_void, val: u32);
    pub fn vpc_set_reset_n(dut: *mut std::ffi::c_void, val: u8);
    pub fn vpc_get_curr_pc(dut: *mut std::ffi::c_void) -> u32;
    pub fn vpc_get_inc_pc(dut: *mut std::ffi::c_void) -> u32;
    pub fn vpc_get_next_pc(dut: *mut std::ffi::c_void) -> u32;
    pub fn vpc_trace_init(
        dut: *mut std::ffi::c_void,
        filename: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_void;
    pub fn vpc_trace_dump(vcd: *mut std::ffi::c_void, time: u64);
    pub fn vpc_trace_close(vcd: *mut std::ffi::c_void);
}
