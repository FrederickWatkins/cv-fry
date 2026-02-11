unsafe extern "C" {
    pub fn vlsu_init() -> *mut std::ffi::c_void;
    pub fn vlsu_destroy(dut: *mut std::ffi::c_void);
    pub fn vlsu_eval(dut: *mut std::ffi::c_void);
    
    // Setters
    pub fn vlsu_set_clk(dut: *mut std::ffi::c_void, val: u8);
    pub fn vlsu_set_reset_n(dut: *mut std::ffi::c_void, val: u8);
    pub fn vlsu_set_dr_ack(dut: *mut std::ffi::c_void, val: u8);
    pub fn vlsu_set_dw_ack(dut: *mut std::ffi::c_void, val: u8);
    pub fn vlsu_set_mm_we(dut: *mut std::ffi::c_void, val: u8);
    pub fn vlsu_set_mm_re(dut: *mut std::ffi::c_void, val: u8);
    pub fn vlsu_set_funct3(dut: *mut std::ffi::c_void, val: u8);
    pub fn vlsu_set_dr_data(dut: *mut std::ffi::c_void, val: u32);
    pub fn vlsu_set_ieu_result(dut: *mut std::ffi::c_void, val: u32);
    pub fn vlsu_set_rs2_data(dut: *mut std::ffi::c_void, val: u32);

    // Getters
    pub fn vlsu_get_stall(dut: *mut std::ffi::c_void) -> u8;
    pub fn vlsu_get_dr_re(dut: *mut std::ffi::c_void) -> u8;
    pub fn vlsu_get_dr_sel(dut: *mut std::ffi::c_void) -> u8;
    pub fn vlsu_get_dw_we(dut: *mut std::ffi::c_void) -> u8;
    pub fn vlsu_get_dw_sel(dut: *mut std::ffi::c_void) -> u8;
    pub fn vlsu_get_dr_addr(dut: *mut std::ffi::c_void) -> u32;
    pub fn vlsu_get_dw_addr(dut: *mut std::ffi::c_void) -> u32;
    pub fn vlsu_get_dw_data(dut: *mut std::ffi::c_void) -> u32;
    pub fn vlsu_get_data_out(dut: *mut std::ffi::c_void) -> u32;
}
