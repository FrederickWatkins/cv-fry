use crate::utils::dut::{DutComb, DutSync};
use cv_fry_cpp::ifu::*;

pub struct Ifu {
    ptr: *mut std::ffi::c_void,
    vcd: Option<*mut std::ffi::c_void>,
    time: u64,
}

#[rustfmt::skip]
impl Ifu {
    pub fn new() -> Self {Self { ptr: unsafe { vifu_init() }, vcd: None, time: 0 }}
    pub fn set_stall(&mut self, val: u8) {unsafe {vifu_set_stall(self.ptr, val);}}
    pub fn set_ack(&mut self, val: u8) {unsafe {vifu_set_ack(self.ptr, val);}}
    pub fn set_je(&mut self, val: u8) {unsafe {vifu_set_je(self.ptr, val);}}
    pub fn set_ja(&mut self, val: u32) {unsafe {vifu_set_ja(self.ptr, val);}}
    pub fn set_instr(&mut self, val: u32) {unsafe {vifu_set_instr(self.ptr, val);}}
    pub fn eval(&mut self) {unsafe {vifu_eval(self.ptr);}}
    pub fn get_re(&self) -> u8 {unsafe { vifu_get_re(self.ptr) }}
    pub fn get_sel(&self) -> u8 {unsafe { vifu_get_sel(self.ptr) }}
    pub fn get_curr_pc(&self) -> u32 {unsafe { vifu_get_curr_pc(self.ptr) }}
    pub fn get_inc_pc(&self) -> u32 {unsafe { vifu_get_inc_pc(self.ptr) }}
    pub fn get_addr(&self) -> u32 {unsafe { vifu_get_addr(self.ptr) }}
    pub fn get_instr_out(&self) -> u32 {unsafe { vifu_get_instr_out(self.ptr) }}
}

impl Drop for Ifu {
    fn drop(&mut self) {
        unsafe { vifu_destroy(self.ptr) };
        self.trace_close();
    }
}

impl DutComb for Ifu {
    fn eval(&mut self) {
        unsafe {
            vifu_eval(self.ptr);
        }
    }
}

impl DutSync for Ifu {
    fn set_clk(&mut self, val: u8) {
        unsafe {
            vifu_set_clk(self.ptr, val);
        }
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

    fn trace_init(&mut self, filename: &str) {
        self.vcd = Some(unsafe {vifu_trace_init(self.ptr, std::ffi::CString::new(filename).unwrap().as_ptr())});
    }
    
    fn trace_dump(&mut self) {
        if let Some(vcd) = self.vcd {
            unsafe {vifu_trace_dump(vcd, self.time);}
        }
    }
    
    fn trace_close(&mut self) {
        if let Some(vcd) = self.vcd {
            unsafe {vifu_trace_close(vcd);}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::c2c_instr::C2cInstr;

    #[test]
    fn test_increments() {
        let mut ifu = Ifu::new();
        let memory: [u8; 64] = [
            0x13, 0x00, 0x00, 0x00, 0x13, 0x00, 0x00, 0x01, 0x13, 0x00, 0x00, 0x02, 0x13, 0x00,
            0x00, 0x03, 0x13, 0x00, 0x00, 0x04, 0x13, 0x00, 0x00, 0x05, 0x13, 0x00, 0x00, 0x06,
            0x13, 0x00, 0x00, 0x07, 0x13, 0x00, 0x00, 0x08, 0x13, 0x00, 0x00, 0x09, 0x13, 0x00,
            0x00, 0x0A, 0x13, 0x00, 0x00, 0x0B, 0x13, 0x00, 0x00, 0x0C, 0x13, 0x00, 0x00, 0x0D,
            0x13, 0x00, 0x00, 0x0E, 0x13, 0x00, 0x00, 0x0F,
        ];
        let mut ack = false;
        let mut instr;
        let mut instr_bus = C2cInstr::new(3);
        ifu.reset();
        assert_eq!(ifu.get_curr_pc(), 0);
        for i in 0..40 {
            if !ack {
                assert_eq!(ifu.get_instr_out(), 0x00000004)
            }
            if ack {
                assert_eq!(ifu.get_instr_out() >> 22, (i - 1) / 4);
            }
            (ack, instr) =
                instr_bus.respond(&memory, ifu.get_re() == 1, ifu.get_sel(), ifu.get_addr());
            ifu.set_ack(ack as u8);
            ifu.set_instr(instr);
            assert_eq!(ifu.get_addr(), (i / 4) * 4);
            ifu.tick();
        }
    }
}
