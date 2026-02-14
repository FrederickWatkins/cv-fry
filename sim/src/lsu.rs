use cv_fry_cpp::lsu::*;
use crate::utils::dut::DutComb;

pub struct Lsu {
    ptr: *mut std::ffi::c_void,
    pub time: u64,
}

#[rustfmt::skip]
impl Lsu {
    pub fn new() -> Self {
        Self { ptr: unsafe { vlsu_init() }, time: 0 }
    }

    // Input Methods
    pub fn set_data_ack(&mut self, val: u8) { unsafe { vlsu_set_data_ack(self.ptr, val); } }
    pub fn set_mm_we(&mut self, val: u8) { unsafe { vlsu_set_mm_we(self.ptr, val); } }
    pub fn set_mm_re(&mut self, val: u8) { unsafe { vlsu_set_mm_re(self.ptr, val); } }
    pub fn set_funct3(&mut self, val: u8) { unsafe { vlsu_set_funct3(self.ptr, val); } }
    pub fn set_data_r(&mut self, val: u32) { unsafe { vlsu_set_data_r(self.ptr, val); } }
    pub fn set_data_in(&mut self, val: u32) { unsafe { vlsu_set_data_in(self.ptr, val); } }
    pub fn set_mm_addr(&mut self, val: u32) { unsafe { vlsu_set_mm_addr(self.ptr, val); } }
    pub fn set_rd_addr_in(&mut self, val: u8) { unsafe { vlsu_set_rd_addr_in(self.ptr, val) } }

    // Output Methods
    pub fn get_busy(&self) -> u8 { unsafe { vlsu_get_busy(self.ptr) } }
    pub fn get_data_re(&self) -> u8 { unsafe { vlsu_get_data_re(self.ptr) } }
    pub fn get_data_sel(&self) -> u8 { unsafe { vlsu_get_data_sel(self.ptr) } }
    pub fn get_data_we(&self) -> u8 { unsafe { vlsu_get_data_we(self.ptr) } }
    pub fn get_data_addr(&self) -> u32 { unsafe { vlsu_get_data_addr(self.ptr) } }
    pub fn get_data_w(&self) -> u32 { unsafe { vlsu_get_data_w(self.ptr) } }
    pub fn get_data_out(&self) -> u32 { unsafe { vlsu_get_data_out(self.ptr) } }
    pub fn get_rd_addr_out(&self) -> u8 { unsafe {vlsu_get_rd_addr_out(self.ptr) } }
}

impl Drop for Lsu {
    fn drop(&mut self) {
        unsafe { vlsu_destroy(self.ptr) };
    }
}

impl DutComb for Lsu {
    fn eval(&mut self) {
        unsafe { vlsu_eval(self.ptr); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_stall() {
        let mut lsu = Lsu::new();
        lsu.set_data_ack(0);
        lsu.set_mm_re(1);
        lsu.eval();
        assert_eq!(lsu.get_busy(), 1);
        lsu.set_data_ack(1);
        lsu.eval();
        assert_eq!(lsu.get_busy(), 0);
        lsu.set_mm_re(0);
        lsu.eval();
        assert_eq!(lsu.get_busy(), 0);
    }

    #[test]
    fn write_stall() {
        let mut lsu = Lsu::new();
        lsu.set_data_ack(0);
        lsu.set_mm_we(1);
        lsu.eval();
        assert_eq!(lsu.get_busy(), 1);
        lsu.set_data_ack(1);
        lsu.eval();
        assert_eq!(lsu.get_busy(), 0);
        lsu.set_mm_we(0);
        lsu.eval();
        assert_eq!(lsu.get_busy(), 0);
    }
}
