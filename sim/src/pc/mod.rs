use crate::utils::dut::DUT;

// Raw FFI definitions
unsafe extern "C" {
    fn vpc_init() -> *mut std::ffi::c_void;
    fn vpc_destroy(dut: *mut std::ffi::c_void);
    fn vpc_eval(dut: *mut std::ffi::c_void);
    fn vpc_set_clk(dut: *mut std::ffi::c_void, val: u8);
    fn vpc_set_stall(dut: *mut std::ffi::c_void, val: u8);
    fn vpc_set_compressed(dut: *mut std::ffi::c_void, val: u8);
    fn vpc_set_je(dut: *mut std::ffi::c_void, val: u8);
    fn vpc_set_ja(dut: *mut std::ffi::c_void, val: u32);
    fn vpc_set_reset_n(dut: *mut std::ffi::c_void, val: u8);
    fn vpc_get_curr_pc(dut: *mut std::ffi::c_void) -> u32;
    fn vpc_get_inc_pc(dut: *mut std::ffi::c_void) -> u32;
    fn vpc_get_next_pc(dut: *mut std::ffi::c_void) -> u32;
}
pub struct ProgramCounter {
    ptr: *mut std::ffi::c_void,
    time: u64,
}

impl ProgramCounter {
    pub fn new() -> Self {
        Self { ptr: unsafe { vpc_init() }, time: 0 }
    }

    pub fn set_stall(&mut self, val: u8) {
        unsafe {
            vpc_set_stall(self.ptr, val);
        }
    }

    pub fn set_compressed(&mut self, val: u8) {
        unsafe {
            vpc_set_compressed(self.ptr, val);
        }
    }

    pub fn set_je(&mut self, val: u8) {
        unsafe {
            vpc_set_je(self.ptr, val);
        }
    }

    pub fn set_ja(&mut self, val: u32) {
        unsafe {
            vpc_set_ja(self.ptr, val);
        }
    }

    pub fn eval(&mut self) {
        unsafe {
            vpc_eval(self.ptr);
        }
    }

    pub fn get_curr_pc(&self) -> u32 {
        unsafe { vpc_get_curr_pc(self.ptr) }
    }

    pub fn get_inc_pc(&self) -> u32 {
        unsafe { vpc_get_inc_pc(self.ptr) }
    }

    pub fn get_next_pc(&self) -> u32 {
        unsafe { vpc_get_next_pc(self.ptr) }
    }
}

impl Drop for ProgramCounter {
    fn drop(&mut self) {
        unsafe { vpc_destroy(self.ptr) };
    }
}

impl DUT for ProgramCounter {
    fn set_clk(&mut self, val: u8) {
        unsafe {vpc_set_clk(self.ptr, val);}
    }

    fn eval(&mut self) {
        unsafe {vpc_eval(self.ptr);}
    }

    fn timestep(&mut self) {
        self.time += 5;
    }

    fn reset(&mut self) {
        unsafe {
            vpc_set_reset_n(self.ptr, 0); // Active low reset?
            self.tick();
            vpc_set_reset_n(self.ptr, 1);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increments() {
        let mut pc = ProgramCounter::new();
        pc.reset();
        assert_eq!(pc.get_curr_pc(), 0);
        pc.tick();
        assert_eq!(pc.get_curr_pc(), 4);
        pc.tick();
        assert_eq!(pc.get_curr_pc(), 8);
        assert_eq!(pc.get_inc_pc(), 12);
        assert_eq!(pc.get_next_pc(), 12);
        pc.set_compressed(1);
        pc.eval();
        assert_eq!(pc.get_curr_pc(), 8);
        assert_eq!(pc.get_inc_pc(), 10);
        assert_eq!(pc.get_next_pc(), 10);
        pc.tick();
        assert_eq!(pc.get_curr_pc(), 10);
        for _ in 0..1000 {
            pc.tick();
        }
    }

    #[test]
    fn test_jump() {
        let mut pc = ProgramCounter::new();
        pc.reset();
        pc.tick();
        pc.tick();
        assert_eq!(pc.get_curr_pc(), 8);
        pc.set_je(1);
        pc.set_ja(1000);
        pc.eval();
        assert_eq!(pc.get_inc_pc(), 12);
        assert_eq!(pc.get_next_pc(), 1000);
        pc.tick();
        pc.set_je(0);
        pc.eval();
        assert_eq!(pc.get_curr_pc(), 1000);
        assert_eq!(pc.get_inc_pc(), 1004);
        assert_eq!(pc.get_next_pc(), 1004);
    }

    #[test]
    fn test_stall() {
        let mut pc = ProgramCounter::new();
        pc.reset();
        pc.tick();
        pc.tick();
        assert_eq!(pc.get_curr_pc(), 8);
        pc.set_stall(1);
        pc.eval();
        assert_eq!(pc.get_inc_pc(), 12);
        assert_eq!(pc.get_next_pc(), 8);
        pc.tick();
        assert_eq!(pc.get_curr_pc(), 8);
        assert_eq!(pc.get_inc_pc(), 12);
        assert_eq!(pc.get_next_pc(), 8);
        pc.set_stall(0);
        pc.eval();
        assert_eq!(pc.get_curr_pc(), 8);
        assert_eq!(pc.get_inc_pc(), 12);
        assert_eq!(pc.get_next_pc(), 12);
        pc.tick();
        assert_eq!(pc.get_curr_pc(), 12);
        pc.eval();
    }

    #[test]
    fn test_stall_jump() {
        let mut pc = ProgramCounter::new();
        pc.reset();
        pc.tick();
        pc.tick();
        assert_eq!(pc.get_curr_pc(), 8);
        pc.set_stall(1);
        pc.set_je(1);
        pc.set_ja(1000);
        pc.eval();
        assert_eq!(pc.get_inc_pc(), 12);
        assert_eq!(pc.get_next_pc(), 8);
        pc.tick();
        assert_eq!(pc.get_curr_pc(), 8);
        assert_eq!(pc.get_inc_pc(), 12);
        assert_eq!(pc.get_next_pc(), 8);
        pc.set_stall(0);
        pc.eval();
        assert_eq!(pc.get_curr_pc(), 8);
        assert_eq!(pc.get_inc_pc(), 12);
        assert_eq!(pc.get_next_pc(), 1000);
        pc.tick();
        assert_eq!(pc.get_curr_pc(), 1000);
    }
}