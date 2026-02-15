use cv_fry_cpp::alu::*;
use crate::utils::dut::DutComb;

pub struct Alu {
    ptr: *mut std::ffi::c_void,
}

impl Alu {
    pub fn new() -> Self {Self { ptr: unsafe { valu_init() } }}
    pub fn set_word(&mut self, val: u8) {unsafe {valu_set_word(self.ptr, val);}}
    pub fn set_funct3(&mut self, val: u8) {unsafe {valu_set_funct3(self.ptr, val);}}
    pub fn set_funct7(&mut self, val: u8) {unsafe {valu_set_funct7(self.ptr, val);}}
    pub fn set_op1(&mut self, val: u64) {unsafe {valu_set_operand_1(self.ptr, val);}}
    pub fn set_op2(&mut self, val: u64) {unsafe {valu_set_operand_2(self.ptr, val);}}
    pub fn eval(&mut self) {unsafe {valu_eval(self.ptr);}}
    pub fn get_result(&self) -> u64 {unsafe { valu_get_result(self.ptr) }}
}

impl Drop for Alu {
    fn drop(&mut self) {
        unsafe { valu_destroy(self.ptr) };
    }
}

impl DutComb for Alu {
    fn eval(&mut self) {
        unsafe {valu_eval(self.ptr);}
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

    // Funct3 values where funct7 = b0000001
    const MULDIV: u8 = 0b0000001;
    const MUL: u8 = 0b000;
    const MULH: u8 = 0b001; // Signed
    const MULHSU: u8 = 0b010; // Signed*unsigned
    const MULHU: u8 = 0b011; // Unsigned*unsigned
    const DIV: u8 = 0b100;
    const DIVU: u8 = 0b101;
    const REM: u8 = 0b110;
    const REMU: u8 = 0b111;

    fn funct3_strategy() -> impl Strategy<Value = u8> {
        any::<u8>().prop_map(|x| x >> 5)
    }

    fn funct7_strategy() -> impl Strategy<Value = u8> {
        any::<u8>().prop_map(|x| match x % 3 {0 => BASEOP, 1 => ALTOP, 2 => MULDIV, _ => 0})
    }

    proptest!{
        #[test]
        fn any_op_64(funct3 in funct3_strategy(), funct7 in funct7_strategy(), op1 in any::<i64>(), op2 in any::<i64>()) {
            let mut alu = Alu::new();
            alu.set_word(0);
            alu.set_funct3(funct3);
            alu.set_funct7(funct7);
            alu.set_op1(op1 as u64);
            alu.set_op2(op2 as u64);
            alu.eval();
            if funct7 == MULDIV && [DIV, DIVU, REM, REMU].contains(&funct3) && op2 == 0 {
                return Ok(());
            }
            if funct7 == MULDIV && [DIV, REM].contains(&funct3) && op1 == i64::MIN && op2 == -1 {
                return Ok(());
            }
            match (funct7, funct3) {
                (BASEOP, ADD) => prop_assert_eq!(alu.get_result() as i64, op1.overflowing_add(op2).0),
                (BASEOP, SL) => prop_assert_eq!(alu.get_result() as i64, op1 << (op2 & 0x3F)),
                (BASEOP, SLT) => prop_assert_eq!(alu.get_result(), if op1 < op2 {1} else {0}),
                (BASEOP, SLTU) => prop_assert_eq!(alu.get_result(), if (op1 as u64) < (op2 as u64) {1} else {0}),
                (BASEOP, XOR) => prop_assert_eq!(alu.get_result() as i64, op1 ^ op2),
                (BASEOP, SRL) => prop_assert_eq!(alu.get_result(), (op1 as u64) >> (op2 & 0x3F)),
                (BASEOP, OR) => prop_assert_eq!(alu.get_result() as i64, op1 | op2),
                (BASEOP, AND) => prop_assert_eq!(alu.get_result() as i64, op1 & op2),
                (ALTOP, SUB) => prop_assert_eq!(alu.get_result() as i64, op1.overflowing_sub(op2).0),
                (ALTOP, SRA) => prop_assert_eq!(alu.get_result() as i64, op1 >> (op2 & 0x3F)),
                (MULDIV, MUL) => prop_assert_eq!(alu.get_result() as i64, op1.overflowing_mul(op2).0),
                (MULDIV, MULH) => prop_assert_eq!(alu.get_result() as i64, ((op1 as i128).overflowing_mul(op2 as i128).0 >> 64) as i64),
                (MULDIV, MULHSU) => prop_assert_eq!(alu.get_result(), ((op1 as u128).overflowing_mul(op2 as u64 as u128).0 >> 64) as u64),
                (MULDIV, MULHU) => prop_assert_eq!(alu.get_result(), ((op1 as u64 as u128).overflowing_mul(op2 as u64 as u128).0 >> 64) as u64),
                (MULDIV, DIV) => prop_assert_eq!(alu.get_result() as i64, if op2 == 0 {-1} else {op1.overflowing_div(op2).0}),
                (MULDIV, DIVU) => prop_assert_eq!(alu.get_result(), if op2 == 0 {u64::MAX} else {(op1 as u64).overflowing_div(op2 as u64).0}),
                (MULDIV, REM) => prop_assert_eq!(alu.get_result() as i64, if op2 == 0 {op1} else {op1.overflowing_rem(op2).0}),
                (MULDIV, REMU) => prop_assert_eq!(alu.get_result(), if op2 == 0 {op1 as u64} else {(op1 as u64).overflowing_rem(op2 as u64).0}),
                _ => (),
            }
        }
    }

    proptest!{
        #[test]
        fn div_zero_64(op1 in any::<i64>()) {
            let mut alu = Alu::new();
            alu.set_word(0);
            alu.set_funct3(DIV);
            alu.set_funct7(MULDIV);
            alu.set_op1(op1 as u64);
            alu.set_op2(0);
            alu.eval();
            prop_assert_eq!(alu.get_result() as i64, -1);
        }
    }

    proptest!{
        #[test]
        fn divu_zero_64(op1 in any::<u64>()) {
            let mut alu = Alu::new();
            alu.set_word(0);
            alu.set_funct3(DIVU);
            alu.set_funct7(MULDIV);
            alu.set_op1(op1);
            alu.set_op2(0);
            alu.eval();
            prop_assert_eq!(alu.get_result(), u64::MAX);
        }
    }

    proptest!{
        #[test]
        fn rem_zero_64(op1 in any::<i64>()) {
            let mut alu = Alu::new();
            alu.set_word(0);
            alu.set_funct3(REM);
            alu.set_funct7(MULDIV);
            alu.set_op1(op1 as u64);
            alu.set_op2(0);
            alu.eval();
            prop_assert_eq!(alu.get_result() as i64, op1);
        }
    }

    proptest!{
        #[test]
        fn remu_zero_64(op1 in any::<u64>()) {
            let mut alu = Alu::new();
            alu.set_word(0);
            alu.set_funct3(REMU);
            alu.set_funct7(MULDIV);
            alu.set_op1(op1);
            alu.set_op2(0);
            alu.eval();
            prop_assert_eq!(alu.get_result(), op1);
        }
    }

    #[test]
    fn div_overflow_64() {
        let mut alu = Alu::new();
        alu.set_word(0);
        alu.set_funct3(DIV);
        alu.set_funct7(MULDIV);
        alu.set_op1(i64::MIN as u64);
        alu.set_op2((-1 as i64) as u64);
        alu.eval();
        assert_eq!(alu.get_result() as i64, i64::MIN);
    }

    #[test]
    fn rem_overflow_64() {
        let mut alu = Alu::new();
        alu.set_word(0);
        alu.set_funct3(REM);
        alu.set_funct7(MULDIV);
        alu.set_op1(i64::MIN as u64);
        alu.set_op2((-1 as i64) as u64);
        alu.eval();
        assert_eq!(alu.get_result() as i64, 0);
    }

    proptest!{
        #[test]
        fn any_op_32(funct3 in funct3_strategy(), funct7 in funct7_strategy(), op1 in any::<i32>(), op2 in any::<i32>()) {
            let mut alu = Alu::new();
            alu.set_word(1);
            alu.set_funct3(funct3);
            alu.set_funct7(funct7);
            alu.set_op1(op1 as u64);
            alu.set_op2(op2 as u64);
            alu.eval();
            if funct7 == MULDIV && [DIV, DIVU, REM, REMU].contains(&funct3) && op2 == 0 {
                return Ok(());
            }
            if funct7 == MULDIV && [DIV, REM].contains(&funct3) && op1 == i32::MIN && op2 == -1 {
                return Ok(());
            }
            match (funct7, funct3) {
                (BASEOP, ADD) => prop_assert_eq!(alu.get_result() as i32, op1.overflowing_add(op2).0),
                (BASEOP, SL) => prop_assert_eq!(alu.get_result() as i32, op1 << (op2 & 0x1F)),
                (BASEOP, SLT) => prop_assert_eq!(alu.get_result(), if op1 < op2 {1} else {0}),
                (BASEOP, SLTU) => prop_assert_eq!(alu.get_result(), if (op1 as u32) < (op2 as u32) {1} else {0}),
                (BASEOP, XOR) => prop_assert_eq!(alu.get_result() as i32, op1 ^ op2),
                (BASEOP, SRL) => prop_assert_eq!(alu.get_result() as u32, (op1 as u32) >> (op2 & 0x1F)),
                (BASEOP, OR) => prop_assert_eq!(alu.get_result() as i32, op1 | op2),
                (BASEOP, AND) => prop_assert_eq!(alu.get_result() as i32, op1 & op2),
                (ALTOP, SUB) => prop_assert_eq!(alu.get_result() as i32, op1.overflowing_sub(op2).0),
                (ALTOP, SRA) => prop_assert_eq!(alu.get_result() as i32, op1 >> (op2 & 0x1F)),
                (MULDIV, MUL) => prop_assert_eq!(alu.get_result() as i32, op1.overflowing_mul(op2).0),
                (MULDIV, MULH) => prop_assert_eq!(alu.get_result() as i32, ((op1 as i64).overflowing_mul(op2 as i64).0 >> 32) as i32),
                (MULDIV, MULHSU) => prop_assert_eq!(alu.get_result() as u32, ((op1 as u64).overflowing_mul(op2 as u32 as u64).0 >> 32) as u32),
                (MULDIV, MULHU) => prop_assert_eq!(alu.get_result() as u32, ((op1 as u32 as u64).overflowing_mul(op2 as u32 as u64).0 >> 32) as u32),
                (MULDIV, DIV) => prop_assert_eq!(alu.get_result() as i32, if op2 == 0 {-1} else {op1.overflowing_div(op2).0}),
                (MULDIV, DIVU) => prop_assert_eq!(alu.get_result() as u32, if op2 == 0 {u32::MAX} else {(op1 as u32).overflowing_div(op2 as u32).0}),
                (MULDIV, REM) => prop_assert_eq!(alu.get_result() as i32, if op2 == 0 {op1} else {op1.overflowing_rem(op2).0}),
                (MULDIV, REMU) => prop_assert_eq!(alu.get_result() as u32, if op2 == 0 {op1 as u32} else {(op1 as u32).overflowing_rem(op2 as u32).0}),
                _ => (),
            }
        }
    }

    proptest!{
        #[test]
        fn div_zero_32(op1 in any::<i32>()) {
            let mut alu = Alu::new();
            alu.set_word(1);
            alu.set_funct3(DIV);
            alu.set_funct7(MULDIV);
            alu.set_op1(op1 as u64);
            alu.set_op2(0);
            alu.eval();
            prop_assert_eq!(alu.get_result() as i32, -1);
        }
    }

    proptest!{
        #[test]
        fn divu_zero_32(op1 in any::<u32>()) {
            let mut alu = Alu::new();
            alu.set_word(1);
            alu.set_funct3(DIVU);
            alu.set_funct7(MULDIV);
            alu.set_op1(op1 as u32 as u64);
            alu.set_op2(0);
            alu.eval();
            prop_assert_eq!(alu.get_result() as u32, u32::MAX);
        }
    }

    proptest!{
        #[test]
        fn rem_zero_32(op1 in any::<i32>()) {
            let mut alu = Alu::new();
            alu.set_word(1);
            alu.set_funct3(REM);
            alu.set_funct7(MULDIV);
            alu.set_op1(op1 as u64);
            alu.set_op2(0);
            alu.eval();
            prop_assert_eq!(alu.get_result() as i32, op1);
        }
    }

    proptest!{
        #[test]
        fn remu_zero_32(op1 in any::<u32>()) {
            let mut alu = Alu::new();
            alu.set_word(1);
            alu.set_funct3(REMU);
            alu.set_funct7(MULDIV);
            alu.set_op1(op1 as u32 as u64);
            alu.set_op2(0);
            alu.eval();
            prop_assert_eq!(alu.get_result() as u32, op1);
        }
    }

    #[test]
    fn div_overflow_32() {
        let mut alu = Alu::new();
        alu.set_word(1);
        alu.set_funct3(DIV);
        alu.set_funct7(MULDIV);
        alu.set_op1(i32::MIN as u64);
        alu.set_op2((-1 as i32) as u64);
        alu.eval();
        assert_eq!(alu.get_result() as i32, i32::MIN);
    }

    #[test]
    fn rem_overflow_32() {
        let mut alu = Alu::new();
        alu.set_word(1);
        alu.set_funct3(REM);
        alu.set_funct7(MULDIV);
        alu.set_op1(i32::MIN as u64);
        alu.set_op2((-1 as i32) as u64);
        alu.eval();
        assert_eq!(alu.get_result() as i32, 0);
    }
}