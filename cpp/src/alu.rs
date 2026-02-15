// Raw FFI definitions
unsafe extern "C" {
    pub fn valu_init() -> *mut std::ffi::c_void;
    pub fn valu_destroy(dut: *mut std::ffi::c_void);
    pub fn valu_eval(dut: *mut std::ffi::c_void);
    pub fn valu_set_word(dut: *mut std::ffi::c_void, val: u8);
    pub fn valu_set_funct3(dut: *mut std::ffi::c_void, val: u8);
    pub fn valu_set_funct7(dut: *mut std::ffi::c_void, val: u8);
    pub fn valu_set_operand_1(dut: *mut std::ffi::c_void, val: u64);
    pub fn valu_set_operand_2(dut: *mut std::ffi::c_void, val: u64);
    pub fn valu_get_result(dut: *mut std::ffi::c_void) -> u64;
}