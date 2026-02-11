unsafe extern "C" {
    pub fn vjbu_init() -> *mut std::ffi::c_void;
    pub fn vjbu_destroy(dut: *mut std::ffi::c_void);
    pub fn vjbu_eval(dut: *mut std::ffi::c_void);
    
    // Setters
    pub fn vjbu_set_jump(dut: *mut std::ffi::c_void, val: u8);
    pub fn vjbu_set_branch(dut: *mut std::ffi::c_void, val: u8);
    pub fn vjbu_set_funct3(dut: *mut std::ffi::c_void, val: u8);
    pub fn vjbu_set_rs1_data(dut: *mut std::ffi::c_void, val: u32);
    pub fn vjbu_set_rs2_data(dut: *mut std::ffi::c_void, val: u32);

    // Getters
    pub fn vjbu_get_jack(dut: *mut std::ffi::c_void) -> u8;
    pub fn vjbu_get_je(dut: *mut std::ffi::c_void) -> u8;
}
