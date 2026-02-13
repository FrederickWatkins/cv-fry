// Raw FFI definitions
unsafe extern "C" {
    pub fn vifu_init() -> *mut std::ffi::c_void;
    pub fn vifu_destroy(dut: *mut std::ffi::c_void);
    pub fn vifu_eval(dut: *mut std::ffi::c_void);
    pub fn vifu_set_clk(dut: *mut std::ffi::c_void, val: u8);
    pub fn vifu_set_stall(dut: *mut std::ffi::c_void, val: u8);
    pub fn vifu_set_je(dut: *mut std::ffi::c_void, val: u8);
    pub fn vifu_set_ja(dut: *mut std::ffi::c_void, val: u32);
    pub fn vifu_set_instr(dut: *mut std::ffi::c_void, val: u32);
    pub fn vifu_set_reset_n(dut: *mut std::ffi::c_void, val: u8);
    pub fn vifu_set_ack(dut: *mut std::ffi::c_void, val: u8);
    pub fn vifu_get_re(dut: *mut std::ffi::c_void) -> u8;
    pub fn vifu_get_sel(dut: *mut std::ffi::c_void) -> u8;
    pub fn vifu_get_curr_pc(dut: *mut std::ffi::c_void) -> u32;
    pub fn vifu_get_inc_pc(dut: *mut std::ffi::c_void) -> u32;
    pub fn vifu_get_addr(dut: *mut std::ffi::c_void) -> u32;
    pub fn vifu_get_instr_out(dut: *mut std::ffi::c_void) -> u32;
}
