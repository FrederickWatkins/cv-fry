use crate::utils::dut::DUT;

unsafe extern "C" {
    fn vlsu_init() -> *mut std::ffi::c_void;
    fn vlsu_destroy(dut: *mut std::ffi::c_void);
    fn vlsu_eval(dut: *mut std::ffi::c_void);
    
    // Setters
    fn vlsu_set_clk(dut: *mut std::ffi::c_void, val: u8);
    fn vlsu_set_reset_n(dut: *mut std::ffi::c_void, val: u8);
    fn vlsu_set_dr_ack(dut: *mut std::ffi::c_void, val: u8);
    fn vlsu_set_dw_ack(dut: *mut std::ffi::c_void, val: u8);
    fn vlsu_set_mm_we(dut: *mut std::ffi::c_void, val: u8);
    fn vlsu_set_mm_re(dut: *mut std::ffi::c_void, val: u8);
    fn vlsu_set_funct3(dut: *mut std::ffi::c_void, val: u8);
    fn vlsu_set_dr_data(dut: *mut std::ffi::c_void, val: u32);
    fn vlsu_set_ieu_result(dut: *mut std::ffi::c_void, val: u32);
    fn vlsu_set_rs2_data(dut: *mut std::ffi::c_void, val: u32);

    // Getters
    fn vlsu_get_stall(dut: *mut std::ffi::c_void) -> u8;
    fn vlsu_get_dr_re(dut: *mut std::ffi::c_void) -> u8;
    fn vlsu_get_dr_sel(dut: *mut std::ffi::c_void) -> u8;
    fn vlsu_get_dw_we(dut: *mut std::ffi::c_void) -> u8;
    fn vlsu_get_dw_sel(dut: *mut std::ffi::c_void) -> u8;
    fn vlsu_get_dr_addr(dut: *mut std::ffi::c_void) -> u32;
    fn vlsu_get_dw_addr(dut: *mut std::ffi::c_void) -> u32;
    fn vlsu_get_dw_data(dut: *mut std::ffi::c_void) -> u32;
    fn vlsu_get_data_out(dut: *mut std::ffi::c_void) -> u32;
}

pub struct Lsu {
    ptr: *mut std::ffi::c_void,
    pub time: u64,
}

impl Lsu {
    pub fn new() -> Self {
        Self { ptr: unsafe { vlsu_init() }, time: 0 }
    }

    // Input Methods
    pub fn set_dr_ack(&mut self, val: u8) { unsafe { vlsu_set_dr_ack(self.ptr, val); } }
    pub fn set_dw_ack(&mut self, val: u8) { unsafe { vlsu_set_dw_ack(self.ptr, val); } }
    pub fn set_mm_we(&mut self, val: u8) { unsafe { vlsu_set_mm_we(self.ptr, val); } }
    pub fn set_mm_re(&mut self, val: u8) { unsafe { vlsu_set_mm_re(self.ptr, val); } }
    pub fn set_funct3(&mut self, val: u8) { unsafe { vlsu_set_funct3(self.ptr, val); } }
    pub fn set_dr_data(&mut self, val: u32) { unsafe { vlsu_set_dr_data(self.ptr, val); } }
    pub fn set_ieu_result(&mut self, val: u32) { unsafe { vlsu_set_ieu_result(self.ptr, val); } }
    pub fn set_rs2_data(&mut self, val: u32) { unsafe { vlsu_set_rs2_data(self.ptr, val); } }

    // Output Methods
    pub fn get_stall(&self) -> u8 { unsafe { vlsu_get_stall(self.ptr) } }
    pub fn get_dr_re(&self) -> u8 { unsafe { vlsu_get_dr_re(self.ptr) } }
    pub fn get_dr_sel(&self) -> u8 { unsafe { vlsu_get_dr_sel(self.ptr) } }
    pub fn get_dw_we(&self) -> u8 { unsafe { vlsu_get_dw_we(self.ptr) } }
    pub fn get_dw_sel(&self) -> u8 { unsafe { vlsu_get_dw_sel(self.ptr) } }
    pub fn get_dr_addr(&self) -> u32 { unsafe { vlsu_get_dr_addr(self.ptr) } }
    pub fn get_dw_addr(&self) -> u32 { unsafe { vlsu_get_dw_addr(self.ptr) } }
    pub fn get_dw_data(&self) -> u32 { unsafe { vlsu_get_dw_data(self.ptr) } }
    pub fn get_data_out(&self) -> u32 { unsafe { vlsu_get_data_out(self.ptr) } }
}

impl Drop for Lsu {
    fn drop(&mut self) {
        unsafe { vlsu_destroy(self.ptr) };
    }
}

impl DUT for Lsu {
    fn set_clk(&mut self, val: u8) {
        unsafe { vlsu_set_clk(self.ptr, val); }
    }

    fn eval(&mut self) {
        unsafe { vlsu_eval(self.ptr); }
    }

    fn timestep(&mut self) {
        self.time += 5;
    }

    fn reset(&mut self) {
        unsafe {
            self.set_clk(0);
            self.eval();
            vlsu_set_reset_n(self.ptr, 0); 
            self.tick(); // Assuming tick() is defined in your DUT trait/helper
            vlsu_set_reset_n(self.ptr, 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_stall() {
        let mut lsu = Lsu::new();
        lsu.reset();
        lsu.set_dr_ack(0);
        lsu.set_mm_re(1);
        lsu.tick();
        lsu.set_mm_re(0);
        lsu.tick();
        assert_eq!(lsu.get_stall(), 1);
        lsu.set_dr_ack(1);
        lsu.tick();
        assert_eq!(lsu.get_stall(), 0);
    }

    #[test]
    fn write_stall() {
        let mut lsu = Lsu::new();
        lsu.reset();
        lsu.set_dw_ack(0);
        lsu.set_mm_we(1);
        lsu.tick();
        lsu.set_mm_we(0);
        lsu.tick();
        assert_eq!(lsu.get_stall(), 1);
        lsu.set_dw_ack(1);
        lsu.tick();
        assert_eq!(lsu.get_stall(), 0);
    }
}
