use cv_fry_cpp::alu::*;
use crate::utils::dut::DUT;

pub struct Alu {
    ptr: *mut std::ffi::c_void,
    time: u64,
}

impl Alu {
    pub fn new() -> Self {
        Self { ptr: unsafe { valu_init() }, time: 0 }
    }

    pub fn set_funct3(&mut self, val: u8) {
        unsafe {
            valu_set_funct3(self.ptr, val);
        }
    }

    pub fn set_funct7(&mut self, val: u8) {
        unsafe {
            valu_set_funct7(self.ptr, val);
        }
    }

    pub fn set_op1(&mut self, val: u32) {
        unsafe {
            valu_set_operand_1(self.ptr, val);
        }
    }

    pub fn set_op2(&mut self, val: u32) {
        unsafe {
            valu_set_operand_2(self.ptr, val);
        }
    }

    pub fn eval(&mut self) {
        unsafe {
            valu_eval(self.ptr);
        }
    }

    pub fn get_result(&self) -> u32 {
        unsafe { valu_get_result(self.ptr) }
    }
}

impl Drop for Alu {
    fn drop(&mut self) {
        unsafe { valu_destroy(self.ptr) };
    }
}

impl DUT for Alu {
    fn set_clk(&mut self, _val: u8) {
        // Module is purely comb, do nothing
    }

    fn eval(&mut self) {
        unsafe {valu_eval(self.ptr);}
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

    const BASEOP: u8 = 0b0000000;
    const ADD: u8 = 0b000; // Add op1 to op2
    const SL: u8 = 0b001; // Left shift
    const SLT: u8 = 0b010; // Set less than (op1 < op2)
    const SLTU: u8 = 0b011; // Set less than unsigned
    const XOR: u8 = 0b100; // Bitwise XOR
    const SRL: u8 = 0b101; // Right shift
    const OR: u8 = 0b110; // Bitwise OR
    const AND: u8 = 0b111; // Bitwise AND

    const ALTOP: u8 = 0b0100000;
    const SUB: u8 = 0b000;
    const SRA: u8 = 0b101;

    fn funct3_strategy() -> impl Strategy<Value = u8> {
        any::<u8>().prop_filter("3 bits", |x| *x < 8)
    }

    fn funct7_strategy() -> impl Strategy<Value = u8> {
        any::<bool>().prop_map(|x| if x {BASEOP} else {ALTOP})
    }

    proptest!{
        #[test]
        fn any_op(funct3 in funct3_strategy(), funct7 in funct7_strategy(), op1 in any::<i32>(), op2 in any::<i32>()) {
            let mut alu = Alu::new();
            alu.set_funct3(funct3);
            alu.set_funct7(funct7);
            alu.set_op1(op1 as u32);
            alu.set_op2(op2 as u32);
            alu.eval();
            match (funct7, funct3) {
                (BASEOP, ADD) => prop_assert_eq!(alu.get_result() as i32, op1.overflowing_add(op2).0),
                (BASEOP, SL) => prop_assert_eq!(alu.get_result() as i32, op1 << (op2 & 0x1F)),
                (BASEOP, SLT) => prop_assert_eq!(alu.get_result() , if op1 < op2 {1} else {0}),
                (BASEOP, SLTU) => prop_assert_eq!(alu.get_result(), if (op1 as u32) < (op2 as u32) {1} else {0}),
                (BASEOP, XOR) => prop_assert_eq!(alu.get_result() as i32, op1 ^ op2),
                (BASEOP, SRL) => prop_assert_eq!(alu.get_result(), (op1 as u32) >> (op2 & 0x1F)),
                (BASEOP, OR) => prop_assert_eq!(alu.get_result() as i32, op1 | op2),
                (BASEOP, AND) => prop_assert_eq!(alu.get_result() as i32, op1 & op2),
                (ALTOP, SUB) => prop_assert_eq!(alu.get_result() as i32, op1.overflowing_sub(op2).0),
                (ALTOP, SRA) => prop_assert_eq!(alu.get_result() as i32, op1 >> (op2 & 0x1F)),
                _ => (),
            }
        }
    }
}