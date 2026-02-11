use crate::utils::dut::DUT;

unsafe extern "C" {
    fn vjbu_init() -> *mut std::ffi::c_void;
    fn vjbu_destroy(dut: *mut std::ffi::c_void);
    fn vjbu_eval(dut: *mut std::ffi::c_void);
    
    // Setters
    fn vjbu_set_jump(dut: *mut std::ffi::c_void, val: u8);
    fn vjbu_set_branch(dut: *mut std::ffi::c_void, val: u8);
    fn vjbu_set_funct3(dut: *mut std::ffi::c_void, val: u8);
    fn vjbu_set_rs1_data(dut: *mut std::ffi::c_void, val: u32);
    fn vjbu_set_rs2_data(dut: *mut std::ffi::c_void, val: u32);

    // Getters
    fn vjbu_get_jack(dut: *mut std::ffi::c_void) -> u8;
    fn vjbu_get_je(dut: *mut std::ffi::c_void) -> u8;
}

pub struct Jbu {
    ptr: *mut std::ffi::c_void,
    pub time: u64,
}

impl Jbu {
    pub fn new() -> Self {
        Self { ptr: unsafe { vjbu_init() }, time: 0 }
    }

    pub fn set_jump(&mut self, val: u8) { unsafe { vjbu_set_jump(self.ptr, val); } }
    pub fn set_branch(&mut self, val: u8) { unsafe { vjbu_set_branch(self.ptr, val); } }
    pub fn set_funct3(&mut self, val: u8) { unsafe { vjbu_set_funct3(self.ptr, val); } }
    pub fn set_rs1_data(&mut self, val: u32) { unsafe { vjbu_set_rs1_data(self.ptr, val); } }
    pub fn set_rs2_data(&mut self, val: u32) { unsafe { vjbu_set_rs2_data(self.ptr, val); } }

    pub fn get_jack(&self) -> u8 { unsafe { vjbu_get_jack(self.ptr) } }
    pub fn get_je(&self) -> u8 { unsafe { vjbu_get_je(self.ptr) } }
}

impl Drop for Jbu {
    fn drop(&mut self) {
        unsafe { vjbu_destroy(self.ptr) };
    }
}

impl DUT for Jbu {
    fn set_clk(&mut self, _val: u8) {
        // Module is purely comb, do nothing
    }

    fn eval(&mut self) {
        unsafe { vjbu_eval(self.ptr); }
    }

    fn timestep(&mut self) {
        self.time += 5;
    }

    fn reset(&mut self) {
        // Module is purely comb, do nothing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    const BEQ: u8 = 0b000;
    const BNE: u8 = 0b001;
    const BLT: u8 = 0b100;
    const BGE: u8 = 0b101;
    const BLTU: u8 = 0b110;
    const BGEU: u8 = 0b111;

    fn funct3_strategy() -> impl Strategy<Value = u8> {
        any::<u8>().prop_filter("3 bits", |x| *x < 8)
    }

    proptest!{
        #[test]
        fn any_op(jump in any::<bool>(), branch in any::<bool>(), funct3 in funct3_strategy(), op1 in any::<i32>(), op2 in any::<i32>()) {
            let mut jbu = Jbu::new();
            if funct3 == 0b010 || funct3 == 0b011 {
                return Ok(());
            }
            jbu.set_branch(branch as u8);
            jbu.set_jump(jump as u8);
            jbu.set_funct3(funct3);
            jbu.set_rs1_data(op1 as u32);
            jbu.set_rs2_data(op2 as u32);
            jbu.eval();
            if jump ^ branch {
                prop_assert_eq!(jbu.get_jack(), 1);
            }
            if !jump & !branch {
                prop_assert_eq!(jbu.get_jack(), 0);
            }
            match (jump, branch, funct3) {
                (true, true, _) => (),
                (true, false, _) => prop_assert_eq!(jbu.get_je(), 1),
                (false, true, BEQ) => prop_assert_eq!(jbu.get_je() , if op1==op2 {1} else {0}),
                (false, true, BNE) => prop_assert_eq!(jbu.get_je() , if op1!=op2 {1} else {0}),
                (false, true, BLT) => prop_assert_eq!(jbu.get_je() , if op1<op2 {1} else {0}),
                (false, true, BGE) => prop_assert_eq!(jbu.get_je() , if op1>=op2 {1} else {0}),
                (false, true, BLTU) => prop_assert_eq!(jbu.get_je() , if (op1 as u32)<(op2 as u32) {1} else {0}),
                (false, true, BGEU) => prop_assert_eq!(jbu.get_je() , if (op1 as u32)>=(op2 as u32) {1} else {0}),
                (false, true, _) => (),
                (false, false, _) => prop_assert_eq!(jbu.get_je(), 0),
            }
        }
    }
}
