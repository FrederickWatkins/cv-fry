use crate::utils::dut::DUT;

// Raw FFI definitions
unsafe extern "C" {
    fn vifu_init() -> *mut std::ffi::c_void;
    fn vifu_destroy(dut: *mut std::ffi::c_void);
    fn vifu_eval(dut: *mut std::ffi::c_void);
    fn vifu_set_clk(dut: *mut std::ffi::c_void, val: u8);
    fn vifu_set_stall(dut: *mut std::ffi::c_void, val: u8);
    fn vifu_set_jump(dut: *mut std::ffi::c_void, val: u8);
    fn vifu_set_jack(dut: *mut std::ffi::c_void, val: u8);
    fn vifu_set_je(dut: *mut std::ffi::c_void, val: u8);
    fn vifu_set_ja(dut: *mut std::ffi::c_void, val: u32);
    fn vifu_set_instr(dut: *mut std::ffi::c_void, val: u32);
    fn vifu_set_reset_n(dut: *mut std::ffi::c_void, val: u8);
    fn vifu_set_ack(dut: *mut std::ffi::c_void, val: u8);
    fn vifu_get_re(dut: *mut std::ffi::c_void) -> u8;
    fn vifu_get_sel(dut: *mut std::ffi::c_void) -> u8;
    fn vifu_get_curr_pc(dut: *mut std::ffi::c_void) -> u32;
    fn vifu_get_inc_pc(dut: *mut std::ffi::c_void) -> u32;
    fn vifu_get_addr(dut: *mut std::ffi::c_void) -> u32;
    fn vifu_get_instr_out(dut: *mut std::ffi::c_void) -> u32;
}
pub struct Ifu {
    ptr: *mut std::ffi::c_void,
    time: u64,
}

impl Ifu {
    pub fn new() -> Self {
        Self { ptr: unsafe { vifu_init() }, time: 0 }
    }

    pub fn set_stall(&mut self, val: u8) {
        unsafe {
            vifu_set_stall(self.ptr, val);
        }
    }

    pub fn set_jump(&mut self, val: u8) {
        unsafe {
            vifu_set_jump(self.ptr, val);
        }
    }
    
    pub fn set_jack(&mut self, val: u8) {
        unsafe {
            vifu_set_jack(self.ptr, val);
        }
    }

    pub fn set_ack(&mut self, val: u8) {
        unsafe {
            vifu_set_ack(self.ptr, val);
        }
    }

    pub fn set_je(&mut self, val: u8) {
        unsafe {
            vifu_set_je(self.ptr, val);
        }
    }

    pub fn set_ja(&mut self, val: u32) {
        unsafe {
            vifu_set_ja(self.ptr, val);
        }
    }

    pub fn set_instr(&mut self, val: u32) {
        unsafe {
            vifu_set_instr(self.ptr, val);
        }
    }

    pub fn eval(&mut self) {
        unsafe {
            vifu_eval(self.ptr);
        }
    }

    pub fn get_re(&self) -> u8 {
        unsafe { vifu_get_re(self.ptr) }
    }

    pub fn get_sel(&self) -> u8 {
        unsafe { vifu_get_sel(self.ptr) }
    }

    pub fn get_curr_pc(&self) -> u32 {
        unsafe { vifu_get_curr_pc(self.ptr) }
    }

    pub fn get_inc_pc(&self) -> u32 {
        unsafe { vifu_get_inc_pc(self.ptr) }
    }

    pub fn get_addr(&self) -> u32 {
        unsafe { vifu_get_addr(self.ptr) }
    }

    pub fn get_instr_out(&self) -> u32 {
        unsafe { vifu_get_instr_out(self.ptr) }
    }
}

impl Drop for Ifu {
    fn drop(&mut self) {
        unsafe { vifu_destroy(self.ptr) };
    }
}

impl DUT for Ifu {
    fn set_clk(&mut self, val: u8) {
        unsafe {vifu_set_clk(self.ptr, val);}
    }

    fn eval(&mut self) {
        unsafe {vifu_eval(self.ptr);}
    }

    fn timestep(&mut self) {
        self.time += 5;
    }
    
    fn reset(&mut self) {
        unsafe {
            self.set_clk(0);
            self.eval();
            vifu_set_reset_n(self.ptr, 0); // Active low reset?
            self.tick();
            vifu_set_reset_n(self.ptr, 1);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::c2c_r::C2cR;

    #[test]
    fn test_increments() {
        let mut ifu = Ifu::new();
        let memory: [u8; 64] = [
            0x00, 0x00, 0x00, 0x13,
            0x01, 0x00, 0x00, 0x13,
            0x02, 0x00, 0x00, 0x13,
            0x03, 0x00, 0x00, 0x13,
            0x04, 0x00, 0x00, 0x13,
            0x05, 0x00, 0x00, 0x13,
            0x06, 0x00, 0x00, 0x13,
            0x07, 0x00, 0x00, 0x13,
            0x08, 0x00, 0x00, 0x13,
            0x09, 0x00, 0x00, 0x13,
            0x0A, 0x00, 0x00, 0x13,
            0x0B, 0x00, 0x00, 0x13,
            0x0C, 0x00, 0x00, 0x13,
            0x0D, 0x00, 0x00, 0x13,
            0x0E, 0x00, 0x00, 0x13,
            0x0F, 0x00, 0x00, 0x13,
        ];
        let mut ack = false;
        let mut instr = 0;
        let mut instr_bus = C2cR::new(3);
        ifu.reset();
        assert_eq!(ifu.get_curr_pc(), 0);
        for i in 0..40 {
            ifu.set_ack(ack as u8);
            ifu.set_instr(instr);
            ifu.set_clk(0);
            ifu.eval();
            ifu.timestep();
            println!("{:x} {} {} {ack}", ifu.get_instr_out(), ifu.get_addr(), i);
            if !ack {
                assert_eq!(ifu.get_instr_out(), 0x00000004)
            }
            if ack {
                assert_eq!(ifu.get_instr_out()>>22, (i - 1) / 4);
            }
            (ack, instr) = instr_bus.respond(&memory, ifu.get_re()==1, ifu.get_sel(), ifu.get_addr());
            assert_eq!(ifu.get_addr(), (i / 4) * 4);
            ifu.set_clk(1);
            ifu.eval();
            ifu.timestep();
        }
        //assert!(false);
    }
}